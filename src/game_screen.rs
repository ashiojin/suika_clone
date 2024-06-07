use crate::prelude::*;
use bevy::{
    prelude::*, sprite::{Material2d, MaterialMesh2dBundle, Mesh2dHandle}
};
use bevy_xpbd_2d::prelude::*;
use itertools::Itertools;

use bevy_rand::prelude::*;
use bevy_rand::resource::GlobalEntropy;
use bevy_prng::ChaCha8Rng;

mod common;
use common::*;
mod game_over_popup;
use game_over_popup::*;
mod pause_popup;
use pause_popup::*;

pub struct ScGameScreenPlugin;

// Physics Engine Settings
const GRAVITY_SCALE: f32 = 9.81 * 100.;
const XPBD_SUBSTEP: u32 = 24;

impl Plugin for ScGameScreenPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Gravity(Vec2::NEG_Y * GRAVITY_SCALE));
        app.insert_resource(SubstepCount(XPBD_SUBSTEP));

        app.insert_state(GameScreenState::Inactive);

        app.add_event::<BallEvent>();
        app.add_event::<PlayerInputEvent>();
        app.add_event::<BallSpawnEvent>();

        app.add_systems(OnEnter(GameState::InGame), (
            activate_game_screen,
        ));
        app.add_systems(OnExit(GameState::InGame), (
            cleanup_ingame_entites,
            cleanup_gameover_popup,
            stop_bgm,
            inactivate_game_screen,
        ));

        app.add_systems(OnEnter(GameScreenState::Init), (
            setup_bottle,
            spawn_player,
            spawn_score_view,
            spwan_holding_ball_view,
            start_play_bgm,

            start_playing,
        ));

        app.add_systems(Update, (
            read_keyboard_for_player_actions,
            grow_ball_spawned,
            check_ball_collisions,
            check_dropping_ball,
            move_puppeteer
                .after(read_keyboard_for_player_actions),
            puppet_player_pos.after(move_puppeteer),
            sync_guide.after(puppet_player_pos),
            pause_game
                .after(read_keyboard_for_player_actions),
            action_player
                .after(read_keyboard_for_player_actions)
                .after(check_dropping_ball),
            combine_balls_touched
                .after(check_ball_collisions),
            spawn_ball
                .after(action_player)
                .after(combine_balls_touched),
            play_se_combine_balls
                .after(action_player)
                .after(combine_balls_touched),
            update_player_view
                .after(action_player),
            score_ball_events,
            check_game_over,
        ).run_if(in_state(GameScreenState::Playing)));

        // game over
        app.add_systems(OnEnter(GameScreenState::GameOver), (
            physics_pause,
            setup_gameover_popup,
        ));

        app.add_systems(Update, (
            read_keyboard_for_gameover_popup,
        ).run_if(in_state(GameScreenState::GameOver)));

        app.add_systems(OnExit(GameScreenState::GameOver), (
            physics_restart,
            cleanup_gameover_popup,
            cleanup_ingame_entites
        ));

        // pausing
        app.add_systems(OnEnter(GameScreenState::Pausing), (
            physics_pause,
            setup_pause_popup,
        ));

        app.add_systems(Update, (
            read_keyboard_for_pause_popup,
        ).run_if(in_state(GameScreenState::Pausing)));

        app.add_systems(OnExit(GameScreenState::Pausing), (
            physics_restart,
            cleanup_pause_popup,
        ));

    }
}

fn activate_game_screen(
    mut next_state: ResMut<NextState<GameScreenState>>
) {
    next_state.set(GameScreenState::Init);
}

fn inactivate_game_screen(
    mut next_state: ResMut<NextState<GameScreenState>>
) {
    next_state.set(GameScreenState::Inactive);
}



#[derive(Component, Debug)]
struct Bottle;
#[derive(Component, Debug)]
struct Background;

// # Screen Layout
// +: (0.0, 0.0)
//                                             
//                      S                       
//                      v                      
//--------------------------------------------- (height: 1080-120=960)
//                                             
//                                             
//                      P                      
//                               SCORE: xx     
//                *         *    HOLDING-+     
//                *         *    |       |     
//                *         *    +-------+     
//                *    +    *    sample--+     
//                *         *    | ...   |     
//                *         *    | ...   |     
//                *         *    | ...   |     
//                **bottle***    +-------+     
//                                             
//---------------------------------------------
//                                             
//                                             
const BOTTOLE_MARGIN_RIGHT: f32 = 60.;
const MARGEN_Y_RIGHT_SIDE: f32 = 10.;
const SCORE_TEXT_WEIGHT: f32 = 30.;
const SCORE_WIDTH: f32 = 360.; // "Score: 12345" (12) 12 * 30.
const SCORE_HEIGHT: f32 = 40.;
const HOLDING_VIEW_TEXT_WEIGHT: f32 = 30.;
const HOLDING_VIEW_WIDTH: f32 = 360.;
const HOLDING_VIEW_HEIGHT: f32 = 360. + HOLDING_VIEW_TEXT_WEIGHT;



//  A Y+
//  |
//  +-> X+
//
//         _ : thickness
//
//  |      | A : height
//  |      | |
//  |      | |
//  |      | |
//  |      | V
//  +------+   | : thickness
//
//   <~~~~> : width
const BOTTLE_CENTER_Y: f32 = -100.0;
const BOTTLE_WIDTH: f32 = 740.0;
const BOTTLE_HEIGHT: f32 = 650.0;
const BOTTLE_THICKNESS: f32 = 30.0;

const BOTTLE_BOTTOM_SIZE: Vec2 = Vec2::new(BOTTLE_WIDTH + 2.*BOTTLE_THICKNESS, BOTTLE_THICKNESS);
const BOTTLE_SIDE_SIZE: Vec2 = Vec2::new(BOTTLE_THICKNESS, BOTTLE_HEIGHT);

const BOTTLE_OUTER_LEFT_TOP: Vec2 = Vec2::new(
        -1. * BOTTLE_BOTTOM_SIZE.x * 0.5,
        -1. * -BOTTLE_SIDE_SIZE.y * 0.5 + BOTTLE_CENTER_Y,
    );
const BOTTLE_OUTER_RIGHT_BOTTOM: Vec2 = Vec2::new(
        BOTTLE_BOTTOM_SIZE.x * 0.5,
        -BOTTLE_SIDE_SIZE.y * 0.5 + BOTTLE_CENTER_Y,
    );

const PLAYER_GAP_WALL: f32 = 50.;
const PLAYER_Y: f32 = BOTTLE_OUTER_LEFT_TOP.y + PLAYER_GAP_WALL;
const PLAYER_GAP_TO_MAX: f32 = 200.;
const PLAYER_Y_MAX: f32 = PLAYER_Y + PLAYER_GAP_TO_MAX;


const SCORE_CENTER: Vec2 =
    Vec2::new(
        BOTTLE_OUTER_RIGHT_BOTTOM.x + BOTTOLE_MARGIN_RIGHT + SCORE_WIDTH * 0.5,
        BOTTLE_OUTER_LEFT_TOP.y + 0.0 - SCORE_HEIGHT * 0.5,
    );

const HOLDING_VIEW_CENTER: Vec2 =
    Vec2::new(
        BOTTLE_OUTER_RIGHT_BOTTOM.x + BOTTOLE_MARGIN_RIGHT + HOLDING_VIEW_WIDTH * 0.5,
        BOTTLE_OUTER_LEFT_TOP.y + 0.0 - SCORE_HEIGHT - MARGEN_Y_RIGHT_SIDE - HOLDING_VIEW_HEIGHT * 0.5,
    );




#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct BallLevel(pub usize);

impl Default for BallLevel {
    fn default() -> Self {
        Self(BALL_LEVEL_MIN)
    }
}
impl BallLevel {
    pub const fn new(lv: usize) -> Self {
        assert!(lv >= BALL_LEVEL_MIN);
        Self(lv)
    }
    pub fn from_rand_u32(rnd: u32, min: BallLevel, max: BallLevel) -> Self {
        Self::new(
            (rnd as usize % (max.0-min.0+1)) + min.0
        )
    }
}

#[derive(Component, Debug, Default, PartialEq, Eq)]
struct Ball {
    level: BallLevel,
}

impl Ball {
    fn new(level: BallLevel) -> Self {
        Self { level }
    }
    fn get_level(&self) -> &BallLevel {
        &self.level
    }
}


fn physics_restart(
    mut physics_time: ResMut<Time<Physics>>,
) {
    physics_time.unpause();
}

fn physics_pause(
    mut physics_time: ResMut<Time<Physics>>,
) {
    physics_time.pause();
}

fn setup_bottle(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let outer_l_t = BOTTLE_OUTER_LEFT_TOP;
    let bottom_l_t = Vec2::new(0., -BOTTLE_HEIGHT) + outer_l_t;
    let left_bottle_l_t = outer_l_t;
    let right_bottle_l_t = Vec2::new(BOTTLE_WIDTH + BOTTLE_THICKNESS, 0.) + outer_l_t;

    fn inv_y(v: Vec2) -> Vec2 { Vec2::new(v.x, -v.y) }
    let bottom_c = bottom_l_t + 0.5 * inv_y(BOTTLE_BOTTOM_SIZE);
    let left_bottle_c = left_bottle_l_t + 0.5 * inv_y(BOTTLE_SIDE_SIZE);
    let right_bottle_c = right_bottle_l_t + 0.5 * inv_y(BOTTLE_SIDE_SIZE);

    let bottle_color = Color::rgb(0.9, 0.7, 0.1);
    let bottle_material = materials.add(bottle_color);

    // Bottom
    commands.spawn((
        Bottle,
        RigidBody::Static,
        Collider::rectangle(BOTTLE_BOTTOM_SIZE.x, BOTTLE_BOTTOM_SIZE.y),
        MaterialMesh2dBundle {
            mesh: meshes.add(Rectangle::from_size(BOTTLE_BOTTOM_SIZE)).into(),
            transform: Transform::from_translation(bottom_c.extend(Z_WALL)),
            material: bottle_material.clone(),
            ..default()
        },
    ));

    // Left bottle
    commands.spawn((
        Bottle,
        RigidBody::Static,
        Collider::rectangle(BOTTLE_SIDE_SIZE.x, BOTTLE_SIDE_SIZE.y),
        MaterialMesh2dBundle {
            mesh: meshes.add(Rectangle::from_size(BOTTLE_SIDE_SIZE)).into(),
            transform: Transform::from_translation(left_bottle_c.extend(Z_WALL)),
            material: bottle_material.clone(),
            ..default()
        },
    ));

    // Right bottle
    commands.spawn((
        Bottle,
        RigidBody::Static,
        Collider::rectangle(BOTTLE_SIDE_SIZE.x, BOTTLE_SIDE_SIZE.y),
        MaterialMesh2dBundle {
            mesh: meshes.add(Rectangle::from_size(BOTTLE_SIDE_SIZE)).into(),
            transform: Transform::from_translation(right_bottle_c.extend(Z_WALL)),
            material: bottle_material.clone(),
            ..default()
        },
    ));

    // BACK
    commands.spawn((
        Background,
        MaterialMesh2dBundle {
            mesh: meshes.add(Rectangle::from_size(Vec2::new(900., 900.))).into(),
            transform: Transform::from_translation(
                (Vec2::new(0., 0.))
                    .extend(Z_BACK)
            ),
            material: materials.add(Color::WHITE),
            ..default()
        },
    ));

}

#[derive(Event, Clone, Copy, PartialEq, Debug)]
enum BallEvent {
    TouchSameLevel(Entity, Entity),
}

#[derive(Event, Clone, Copy, PartialEq, Debug)]
enum BallSpawnEvent {
    Drop(Vec2, BallLevel),
    Combine(Vec2, Option<BallLevel>),
}

fn check_ball_collisions(
    mut ev_colls: EventReader<Collision>,
    mut ev_ball: EventWriter<BallEvent>,
    q_balls: Query<(Entity, &Ball)>,
) {
    let mut touches = vec![];
    for Collision(contacts) in ev_colls.read() {
        let b1 = q_balls.get(contacts.entity1);
        let b2 = q_balls.get(contacts.entity2);
        if let (Ok(b1), Ok(b2)) = (b1, b2) {
            if b1.1.get_level() == b2.1.get_level() {
                touches.push((
                        std::cmp::min(b1.0, b2.0),
                        std::cmp::max(b1.0, b2.0),
                        ));
            }
        }
    }

    // check whether 3 balls are colliding in same frame.
    let touches = touches.into_iter()
        .sorted_by(|l, r| Ord::cmp(&l.0, &r.0))
        .coalesce(|l, r|
            if l.0 == r.0 {
                Ok(l)
            } else {
                Err((l, r))
            }
        );

    for touch in touches {
        ev_ball.send(BallEvent::TouchSameLevel(touch.0, touch.1));
    }
}

fn combine_balls_touched(
    mut commands: Commands,
    mut ev_ball: EventReader<BallEvent>,
    mut ev_ball_spawn: EventWriter<BallSpawnEvent>,

    q_ball: Query<(Entity, &Transform, &Ball)>,
    sc_asset: Res<GameAssets>,
) {
    for ev in ev_ball.read() {
        match ev {
            BallEvent::TouchSameLevel(e1, e2) => {
                let b1 = q_ball.get(*e1);
                let b2 = q_ball.get(*e2);
                commands.entity(*e1).despawn_recursive();
                commands.entity(*e2).despawn_recursive();

                if let (Ok((_, t1, b1)), Ok((_, t2, _))) = (b1, b2) {
                    let pos = (t1.translation.xy() + t2.translation.xy()) / 2.;
                    let cur_lv = b1.get_level();

                    if *cur_lv == sc_asset.get_ball_max_level() {
                        ev_ball_spawn.send(
                            BallSpawnEvent::Combine(pos, None));
                    } else {
                        ev_ball_spawn.send(
                            BallSpawnEvent::Combine(pos, Some(BallLevel(cur_lv.0 + 1))));
                    }
                }

            }
        }
    }
}

fn score_ball_events(
    mut q_player: Query<&mut Player>,
    mut ev_ball: EventReader<BallSpawnEvent>,

    sc_asset: Res<GameAssets>,
) {
    if let Ok(mut player) = q_player.get_single_mut() {
        let score: u32 = ev_ball.read()
            .map(|ev| match ev {
                BallSpawnEvent::Drop(_, _level) => {
                    //level.0 as u32 * 1
                    0
                },
                BallSpawnEvent::Combine(_, level) => {
                    let level_combined = level.map(|l| l.0-1)
                        .unwrap_or(sc_asset.get_ball_max_level().0);
                    level_combined.pow(2) as u32 // * 1
                },
            })
            .sum();
        player.score += score;
    }
}

fn check_game_over(
    q_balls: Query<&Transform, With<Ball>>,
    mut next_state: ResMut<NextState<GameScreenState>>,
    config: Res<Config>,
) {
    let Area { min_x, max_x, min_y, max_y } = config.area;
    if let Some(_ball) = q_balls.iter().find(|t| {
        let t = t.translation;
        let x = t.x;
        let y = t.y;
        !(min_x..=max_x).contains(&x) ||
            !(min_y..=max_y).contains(&y)
    }) {
        next_state.set(GameScreenState::GameOver);
    }
}


/// Player inputs
#[derive(Event, Debug, Clone, Copy, PartialEq)]
enum PlayerInputEvent {
    Drop,
    Move(f32), // [-1, 1]
    Hold,
    Pause,
}

fn read_keyboard_for_player_actions(
    q_player: Query<&Player>,
    keyboard: Res<ButtonInput<KeyCode>>,

    mut ev_player_act: EventWriter<PlayerInputEvent>,
) {
    if q_player.get_single().is_ok() {
        let mut lr = 0.;
        if keyboard.pressed(KeyCode::ArrowLeft) {
            lr += -1.;
        }
        if keyboard.pressed(KeyCode::ArrowRight) {
            lr += 1.;
        }

        if keyboard.just_pressed(KeyCode::Space) {
            ev_player_act.send(PlayerInputEvent::Drop);
        }

        if keyboard.just_pressed(KeyCode::ArrowUp) {
            ev_player_act.send(PlayerInputEvent::Hold);
        }

        if keyboard.just_pressed(KeyCode::KeyP) {
            ev_player_act.send(PlayerInputEvent::Pause);
        }

        if lr != 0. {
            ev_player_act.send(PlayerInputEvent::Move(lr));
        }
    }
}

#[derive(Component, Debug, Default)]
struct PlayerPuppeteer {
}
#[derive(Component, Debug, Default)]
struct DroppingBallGuide;
#[derive(Component, Debug, Default)]
struct DroppingBallGuideBody;


/// spwans player / puppetter / guide for dropping a ball
fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,

    mut global_ent: ResMut<GlobalEntropy<ChaCha8Rng>>,
    assets: Res<GameAssets>,
) {
    // puppetter
    commands.spawn((
        PlayerPuppeteer{},
        TransformBundle::from_transform(
            Transform::from_translation(Vec2::new(0., PLAYER_Y_MAX).extend(Z_PLAYER))
        ),
        ShapeCaster::new(
            Collider::circle(10.),
            Vec2::ZERO,
            0.,
            Direction2d::NEG_Y
        ),
    ));

    // player
    let player_y = PLAYER_Y;
    let mut player = Player::new(assets.player_settings.speed, BallLevel::new(1), assets.drop_ball_level_max);
    let mut rng = global_ent.fork_rng();
    player.set_next_ball_level_from_rng(&mut rng);


    let player_material = materials.add(assets.player_settings.h_image.clone());
    let player_mesh = Rectangle::new(
        assets.player_settings.view_width,
        assets.player_settings.view_height,
    );
    let player_offset = Vec2::new(
        assets.player_settings.offset_x,
        assets.player_settings.offset_y,
    );

    commands.spawn((
        player,
        SpatialBundle {
            transform: Transform::from_translation(
                Vec2::new(0., player_y).extend(Z_PLAYER)),
            ..default()
        },
        rng,
    )).with_children(|b| {
        b.spawn((
            MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(player_mesh)),
                material: player_material,
                transform: Transform::from_translation(
                    player_offset.extend(0.01)),
                ..default()
            },
        ));
    });

    // guide
    let guide_material = materials.add(assets.player_settings.guide_color);
    commands.spawn((
        DroppingBallGuide,
        SpatialBundle {
            transform: Transform::from_translation(Vec2::new(0., player_y).extend(Z_GUIDE)),
            visibility: Visibility::Visible,
            ..default()
        },
    )).with_children(|b| {
        b.spawn((
            DroppingBallGuideBody,
            MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(Rectangle::new(10.0, 1.0))),
                material: guide_material,
                transform: Transform::from_translation(Vec2::new(0., -0.5).extend(0.01)),
                visibility: Visibility::Inherited,
                ..default()
            },
        ));
    });
}

fn move_puppeteer(
    q_player: Query<&Player>,
    mut q_puppeteer: Query<(&mut Transform, &PlayerPuppeteer)>,
    mut ev_player_act: EventReader<PlayerInputEvent>,
) {
    if let Ok((mut trans, _)) = q_puppeteer.get_single_mut() {
        if let Ok(player) = q_player.get_single() {
            for ev in ev_player_act.read() {
                if let PlayerInputEvent::Move(lr) = ev {
                    trans.translation.x =
                        (trans.translation.x + lr * player.speed)
                            .clamp(-BOTTLE_WIDTH/2., BOTTLE_WIDTH/2.);
                }
            }
        }
    }
}

fn get_shortest_hit(hits: &ShapeHits) -> Option<&ShapeHitData> {
    hits.iter().min_by(|a, b| a.time_of_impact.partial_cmp(&b.time_of_impact).unwrap())
}

fn puppet_player_pos(
    mut q_player: Query<(&mut Transform, &mut Player)>,
    q_puppeteer: Query<(&Transform, &ShapeCaster, &ShapeHits), Without<Player>>,
    sc_asset: Res<GameAssets>,
) {
    if let Ok((trans, _caster, hits)) = q_puppeteer.get_single() {
        if let Some(hit) = get_shortest_hit(hits) {

            if let Ok((mut player_trans, player)) = q_player.get_single_mut() {
                let ball_r = if player.is_fakeball_exists() {
                    sc_asset.get_ball_r(player.next_ball_level)
                } else {
                    0.
                };
                let player_y = f32::max(
                    PLAYER_Y,
                    trans.translation.y
                        - hit.time_of_impact
                        + ball_r
                );

                player_trans.translation.x = trans.translation.x;
                player_trans.translation.y = player_y;
            }
        }

    }
}

#[allow(clippy::type_complexity)]
fn sync_guide(
    mut set: ParamSet<(
        Query<(&mut Transform, &mut Visibility), With<DroppingBallGuide>>,
        Query<&mut Transform, With<DroppingBallGuideBody>>,
        Query<(&Transform, &Player)>,
        Query<(&Transform, &ShapeCaster, &ShapeHits), Without<Player>>,
    )>,
) {
    // 1st: Origin/Visibility of Guide
    let player_trans = set.p2().get_single().map(|(o, p)| (o.translation.x, o.translation.y, p.is_fakeball_exists()));
    if let Ok((player_x, player_y, has_fake_ball)) = player_trans {
        if let Ok((mut trans, mut vis)) = set.p0().get_single_mut() {
            trans.translation.x = player_x;
            trans.translation.y = player_y;
            *vis = if has_fake_ball { Visibility::Visible } else { Visibility::Hidden };
        }
    }

    // 2nd: Guide Body Length
    let puppetter_data = set.p3().get_single().map(|(t, _, hits)| (t.translation.y, get_shortest_hit(hits).cloned()));
    if let (Ok((puppetter_y, Some(hit))), Ok((_, player_y, _))) = (puppetter_data, player_trans) {
        if let Ok(mut trans) = set.p1().get_single_mut() {
            let len = hit.time_of_impact - (puppetter_y - player_y);
            trans.translation.y = -0.5 * len;
            trans.scale.y = len;
        }
    }
}

fn action_player(
    mut q_player: Query<(&Transform, &mut Player, &mut EntropyComponent<ChaCha8Rng>)>,
    mut ev_player_act: EventReader<PlayerInputEvent>,
    mut ev_ball_spawn: EventWriter<BallSpawnEvent>,
) {
    if let Ok((trans, mut player, mut rng)) = q_player.get_single_mut() {

        for ev in ev_player_act.read() {
            match ev {
                PlayerInputEvent::Drop => {
                    if player.can_drop {
                        let pos = trans.translation.xy();
                        let lv = player.next_ball_level;

                        ev_ball_spawn.send(BallSpawnEvent::Drop(pos, lv));

                        player.set_next_ball_level_from_rng(&mut rng);
                        player.can_drop = false;
                    }
                },
                PlayerInputEvent::Hold => {
                    if player.can_drop {
                        let lv = player.next_ball_level;
                        if let Some(hold_level) = player.hold_ball {
                            player.next_ball_level = hold_level;
                        } else {
                            player.set_next_ball_level_from_rng(&mut rng);
                        }
                        player.hold_ball = Some(lv);
                    }
                }
                PlayerInputEvent::Move(_lr) => {
                },
                PlayerInputEvent::Pause => {
                }
            }
        }
    }
}

#[derive(Component, Debug)]
struct DroppingBall;

#[derive(Component, Debug)]
struct FakeBall(pub BallLevel);

#[derive(Component, Debug)]
struct ScoreView;

#[derive(Component, Debug)]
struct ScoreText;

fn spawn_score_view(
    mut commands: Commands,
    my_assets: Res<GameAssets>,
) {
    let score_size = Vec2::new(SCORE_WIDTH, SCORE_HEIGHT);
    let score_center = SCORE_CENTER;
    commands
        .spawn((
            ScoreView,
            SpriteBundle { // as frame
                sprite: Sprite {
                    color: Color::BLACK,
                    custom_size: Some(score_size),
                    ..default()
                },
                transform: Transform::from_translation(
                               score_center.extend(Z_UI)),
                ..default()
            },
        ))
        .with_children(|b| {
            let text_style = TextStyle {
                font: my_assets.h_font.clone(),
                font_size: SCORE_TEXT_WEIGHT,
                color: Color::WHITE,
            };
            b.spawn((
                ScoreText,
                Text2dBundle {
                    text: Text::from_section("***", text_style.clone()),
                    transform: Transform::from_translation(Vec3::Z * 0.01),
                    ..default()
                },
            ));
        });
}

#[derive(Component, Debug)]
struct HoldingBallView;
#[derive(Component, Debug)]
struct HoldingBallImage(Option<BallLevel>);

fn spwan_holding_ball_view(
    mut commands: Commands,
    my_assets: Res<GameAssets>,
) {
    commands.spawn((
        HoldingBallView,
        SpriteBundle {
            sprite: Sprite {
                color: Color::BLACK,
                custom_size: Some(Vec2::new(HOLDING_VIEW_WIDTH, HOLDING_VIEW_HEIGHT)),
                ..default()
            },
            transform: Transform::from_translation(
                           HOLDING_VIEW_CENTER.extend(Z_UI)),
            ..default()
        }

    ))
    .with_children(|b| {
        let label_pos =
            Vec2::new(0., HOLDING_VIEW_HEIGHT/2.- HOLDING_VIEW_TEXT_WEIGHT/2.)
            ;
        let image_pos =
            Vec2::new(0., -HOLDING_VIEW_TEXT_WEIGHT/2.);
        let text_style = TextStyle {
            font: my_assets.h_font.clone(),
            font_size: HOLDING_VIEW_TEXT_WEIGHT,
            color: Color::WHITE,
        };
        b.spawn((
            Text2dBundle {
                text: Text::from_section("Hold:", text_style.clone()),
                transform: Transform::from_translation(label_pos.extend(0.02)),
                ..default()
            },
        ));

        b.spawn((
            SpatialBundle {
                transform: Transform::from_translation(
                               image_pos.extend(0.01)
                           ),
                ..default()
            },
        ))
        .with_children(|b| {
            b.spawn((
                HoldingBallImage(None),
                SpatialBundle {
                    transform: Transform::from_translation(
                                   Vec2::ZERO.extend(0.01)
                               ),
                    ..default()
                }
            ));
        });
    });
}

fn start_playing(
    mut next_state: ResMut<NextState<GameScreenState>>,
) {
    next_state.set(GameScreenState::Playing);
}

#[allow(clippy::too_many_arguments)]
fn update_player_view(
    q_player: Query<(Entity, &Player)>,

    q_fakeball: Query<(Entity, &FakeBall)>,

    q_holding_ball: Query<(Entity, &HoldingBallImage)>,

    mut q_score_text: Query<&mut Text, With<ScoreText>>,

    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    my_assets: Res<GameAssets>,
) {
    if let Ok((plyer_entity, player)) = q_player.get_single() {
        // Fake ball
        let fakeball = q_fakeball.get_single();

        if player.is_fakeball_exists() {
            // need fake ball
            // - if there already is, check its level and update it if necessary.
            // - if there is not, spawn it.
            if let Ok((fakeball_entity, FakeBall(fakeball_level))) = fakeball {
                if *fakeball_level != player.next_ball_level { // Holding a ball cieses this.
                    // update
                    let ball_view = create_ball_view(
                        &mut meshes, &mut materials, player.next_ball_level,
                        Vec2::ZERO, &my_assets);
                    commands.entity(fakeball_entity)
                        .insert((
                            FakeBall(player.next_ball_level),
                            ball_view,
                        ));
                }
            } else {
                commands.entity(plyer_entity)
                    .with_children(|b| {
                        let ball_view = create_ball_view(
                            &mut meshes, &mut materials, player.next_ball_level,
                            Vec2::ZERO, &my_assets);
                        b.spawn((
                            FakeBall(player.next_ball_level),
                            ball_view,
                        ));
                    });
            }
        } else {
            // don't need fake ball
            if let Ok((fakeball, _)) = fakeball {
                commands.entity(fakeball)
                    .despawn_recursive();
            }
        }

        // Holding
        if let Ok((hold_ball_entity, hold_ball)) = q_holding_ball.get_single() {
            if hold_ball.0 != player.hold_ball {
                if let Some(hold_ball) = player.hold_ball {
                    let ball_view = create_ball_view(
                        &mut meshes, &mut materials, hold_ball,
                        Vec2::ZERO, &my_assets);
                    commands.entity(hold_ball_entity)
                        .insert(HoldingBallImage(Some(hold_ball)))
                        .insert(ball_view);
                } else {
                    commands.entity(hold_ball_entity)
                        .insert(HoldingBallImage(None))
                        .insert(Visibility::Hidden);
                }
            }
        }

        // Score
        if let Ok(mut text) = q_score_text.get_single_mut() {
            if let Some(score_text) = text.sections.first_mut() {
                score_text.value = format!("Score:{:>6}", player.score);
            }
        }
    }
}


#[derive(Bundle)]
struct BallBundle<M: Material2d> {
    ball: Ball,
    rigit_body: RigidBody,
    collider: Collider,
    restitution: Restitution,
    mat_mesh2_bundle: MaterialMesh2dBundle<M>
}

impl<M: Material2d> Default for BallBundle<M> {
    fn default() -> Self {
        Self {
            ball: default(),
            rigit_body: RigidBody::Dynamic,
            collider: Collider::circle(1.),
            restitution: Restitution::new(0.01),
            mat_mesh2_bundle: default(),
        }
    }
}


#[derive(Component, Debug, Clone, Copy)]
struct BallGrowing {
    sec: f32,
    sec_max: f32,
}
impl BallGrowing {
    fn new(sec_max: f32) -> Self {
        Self { sec: 0., sec_max }
    }
}


fn create_ball_view(
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,

    level: BallLevel,
    pos: Vec2,

    my_assets: &Res<GameAssets>,
) -> impl Bundle {

    let ball_material = materials.add(my_assets.get_ball_image(level).clone());
    let (mesh_w, mesh_h) = my_assets.get_ball_mesh_wh(level);
    MaterialMesh2dBundle {
        mesh: meshes.add(Rectangle::new(mesh_w, mesh_h)).into(),
        transform: Transform::from_translation(
             pos.extend(Z_BALL + Z_BALL_D_BY_LEVEL * level.0 as f32)
        ),
        material: ball_material,
        ..default()
    }
}

fn spawn_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,

    mut ev_ball_spawn: EventReader<BallSpawnEvent>,
    my_assets: Res<GameAssets>,
    config: Res<Config>,
) {
    for ev in ev_ball_spawn.read() {
        use BallSpawnEvent::*;
        match *ev {
            Drop(pos, level) => {
                let ball_view = create_ball_view(&mut meshes, &mut materials,
                                                 level, pos, &my_assets);
                commands.spawn((
                    DroppingBall,
                    Ball::new(level),
                    RigidBody::Dynamic,
                    Collider::circle(my_assets.get_ball_r(level)),
                    ball_view,
                ));
            },
            Combine(pos, Some(level)) => {
                let ball_r_start = my_assets.get_ball_start_r(level);
                let ball_view = create_ball_view(&mut meshes, &mut materials,
                                                 level, pos, &my_assets);
                commands.spawn((
                    Ball::new(level),
                    RigidBody::Dynamic,
                    Collider::circle(ball_r_start),
                    BallGrowing::new(config.grow_time),
                    ball_view,
                ));
            },
            Combine(_, None) => {
                // Nothing to do
            }
        }
    }
}

fn check_dropping_ball(
    mut commands: Commands,
    mut q_player: Query<&mut Player, Without<DroppingBall>>,
    q_ball: Query<(Entity, &CollidingEntities), With<DroppingBall>>,
) {
    if let Ok(mut player) = q_player.get_single_mut() {
        if let Ok((entity, colliding_entities)) = q_ball.get_single() {
            if !colliding_entities.is_empty() {
                // touch anything
                commands.entity(entity)
                    .remove::<DroppingBall>();
                player.can_drop = true;
            }
        } else {
            player.can_drop = true;
        }
    }
}

fn play_se_combine_balls(
    mut commands: Commands,
    mut ev_ball_spawn: EventReader<BallSpawnEvent>,
    sc_assets: Res<GameAssets>,
    config: Res<Config>,
) {
    for ev in ev_ball_spawn.read() {
        use BallSpawnEvent::*;
        if matches!(ev, Combine(_,_)) {
            spawn_se(
                &mut commands,
                sc_assets.h_se_combine.clone(),
                config.get_se_volume(),
            );
        }
    }
}

fn grow_ball_spawned(
    mut commands: Commands,
    mut q_ball: Query<(Entity, &mut BallGrowing, &Ball)>,
    time: Res<Time>,
    sc_asset: Res<GameAssets>,
) {
    for (entity, mut spacer, ball) in q_ball.iter_mut() {
        spacer.sec += time.delta_seconds();

        if spacer.sec > spacer.sec_max {
            commands.entity(entity)
                .try_insert(Collider::circle(
                        sc_asset.get_ball_r(*ball.get_level())
                    ))
                .remove::<BallGrowing>();
        } else {
            let level = *ball.get_level();
            let r_to = sc_asset.get_ball_r(level);
            let r_from = sc_asset.get_ball_start_r(level);
            let r = (spacer.sec / spacer.sec_max) * (r_to - r_from) + r_from;
            commands.entity(entity)
                .try_insert(Collider::circle(r));
        }
    }
}

#[allow(clippy::complexity)]
fn cleanup_ingame_entites(
    mut commands: Commands,
    q_entites: Query<Entity,
        Or<(
            With<Player>, // FIXME: Should add an InGameEntity component?
            With<PlayerPuppeteer>,
            With<DroppingBallGuide>,
            With<HoldingBallView>,
            With<Ball>,
            With<Bottle>,
            With<Background>,
            With<ScoreView>,
        )>>,
) {
    for e in q_entites.iter() {
        commands.entity(e)
            .despawn_recursive();
    }
}



fn start_play_bgm(
    mut commands: Commands,
    mut q_bgm: Query<&mut AudioSink, With<Bgm>>,
    sc_asset: Res<GameAssets>,
    config: Res<Config>,
) {
    if let Ok(sink) = q_bgm.get_single_mut() {
        sink.play(); // Already spawned: call play() to be sure
    } else {
        spawn_bgm(
            &mut commands,
            sc_asset.h_bgm.clone(),
            config.get_bgm_volume(),
        );
    }
}

fn pause_game(
    mut events: EventReader<PlayerInputEvent>,
    mut next_state: ResMut<NextState<GameScreenState>>,
) {
    for event in events.read() {
        if matches!(event, PlayerInputEvent::Pause) {
            next_state.set(GameScreenState::Pausing);
        }
    }
}
