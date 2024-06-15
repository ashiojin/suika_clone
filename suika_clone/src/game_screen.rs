use std::f32::consts::PI;
use crate::prelude::*;
use bevy::{
    prelude::*, sprite::{Material2d, MaterialMesh2dBundle, Mesh2dHandle},
};
use bevy_xpbd_2d::prelude::*;
use itertools::Itertools;

use bevy_rand::prelude::*;
use bevy_rand::resource::GlobalEntropy;
use bevy_prng::ChaCha8Rng;

mod common;
use common::*;
mod camera;
mod game_over_popup;
use game_over_popup::*;
mod pause_popup;
use pause_popup::*;
use rand_core::RngCore;

pub struct ScGameScreenPlugin;

// Physics Engine Settings
const GRAVITY_SCALE: f32 = 9.81 * 100.;
const XPBD_SUBSTEP: u32 = 32;

impl Plugin for ScGameScreenPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Gravity(Vec2::NEG_Y * GRAVITY_SCALE));
        app.insert_resource(SubstepCount(XPBD_SUBSTEP));

        app.insert_state(GameScreenState::Inactive);

        app.add_event::<BallEvent>();
        app.add_event::<PlayerInputEvent>();
        app.add_event::<BallSpawnEvent>();

        // GameState :: InGame
        app.add_systems(OnEnter(GameState::InGame), (
            camera::spawn_camera,
            camera::update_camera
                .after(camera::spawn_camera), // FIXME: can i put it only in Update?
            activate_game_screen,
        ));
        app.add_systems(OnExit(GameState::InGame), (
            camera::despawn_camera,
            cleanup_ingame_entites,
            cleanup_gameover_popup,
            stop_bgm,
            inactivate_game_screen,
        ));
        app.add_systems(Update, (
            camera::update_camera,
            camera::update_pinned_to_camera,
        ).run_if(in_state(GameState::InGame)));

        // GameScreenState :: Init
        app.add_systems(OnEnter(GameScreenState::Init), (
            spawn_background,
            spawn_bottle,
            spawn_player,
            spawn_score_view,
            spwan_holding_ball_view,
            spawn_manual_view,
            start_play_bgm,

            start_playing,
        ));

        // GameScreenState :: Playing
        app.add_systems(Update, (
            read_keyboard_for_player_actions,
            read_gamepad_for_player_actions,
            grow_ball_spawned,
            check_ball_collisions,
            check_dropping_ball,
            move_puppeteer
                .after(read_gamepad_for_player_actions)
                .after(read_keyboard_for_player_actions),
            puppet_player_pos.after(move_puppeteer),
            sync_guide.after(puppet_player_pos),
            sync_puppetter_shape_caster
                .after(sync_guide),
            pause_game
                .after(read_gamepad_for_player_actions)
                .after(read_keyboard_for_player_actions),
            action_player
                .after(read_gamepad_for_player_actions)
                .after(read_keyboard_for_player_actions)
                .after(check_dropping_ball),
            shake_bottle
                .after(read_gamepad_for_player_actions)
                .after(read_keyboard_for_player_actions),
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

        // GameScreenState :: GameOver
        app.add_event::<GameOverPopupInput>();
        app.add_systems(OnEnter(GameScreenState::GameOver), (
            physics_pause,
            setup_gameover_popup,
            record_score
                .after(setup_gameover_popup),
            save_scores
                .after(record_score),
            move_camera_to_ball_protruded,
        ));

        app.add_systems(Update, (
            update_gameover_popup,
            read_keyboard_for_gameover_popup,
            read_gamepad_for_gameover_popup,
            act_gameover_popup
                .after(read_keyboard_for_gameover_popup)
                .after(read_gamepad_for_gameover_popup),
        ).run_if(in_state(GameScreenState::GameOver)));

        app.add_systems(OnExit(GameScreenState::GameOver), (
            physics_restart,
            cleanup_gameover_popup,
            cleanup_ingame_entites,
            move_camera_to_default,
        ));

        // GameScreenState :: Paused
        app.add_event::<PausePopupInput>();
        app.add_systems(OnEnter(GameScreenState::Paused), (
            physics_pause,
            setup_pause_popup,
        ));

        app.add_systems(Update, (
            update_pause_popup,
            read_keyboard_for_pause_popup,
            read_gamepad_for_pause_popup,
            act_pause_popup
                .after(read_keyboard_for_pause_popup)
                .after(read_gamepad_for_pause_popup),
        ).run_if(in_state(GameScreenState::Paused)));

        app.add_systems(OnExit(GameScreenState::Paused), (
            physics_restart,
            cleanup_pause_popup,
        ));

        // GameScreenState :: Restart
        app.add_systems(OnEnter(GameScreenState::Restart), (
            physics_restart,
            cleanup_ingame_entites,
            |mut next: ResMut<NextState<GameScreenState>>| { next.set(GameScreenState::Init); }, // FIXME: Re-design states
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

fn record_score(
    q_player: Query<&Player>,
    config: Res<Config>,
    mut scores: ResMut<Scores>,
) {
    if let Ok(player) = q_player.get_single() {
        let game_cnd = GameCond::new(&config.game_ron_name);
        scores.push(&game_cnd, Score::new(player.score));
    }
}


#[derive(Component, Debug)]
struct Bottle {
    origin: Vec2,
}

#[derive(Component, Debug)]
struct BottleWall;

#[derive(Component, Debug)]
struct Background;

// # Screen Layout
// +: (0.0, 0.0)
//                                             
//                      S                       
//                      v                      
//--------------------------------------------- (height: 1080-120=960)
//                                             
//                               +-------+     
//                      P        |SCORE  |     
//                               +-------+     
//                *         *    +-------+     
//                *         *    |HOLD   |     
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

const FONT_WEIGHT_L: f32 = 32.;
const FONT_WEIGHT_M: f32 = 24.;
const FONT_WEIGHT_S: f32 = 16.;



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

const PLAYER_GAP_WALL: f32 = 50.;
const PLAYER_GAP_TO_MAX: f32 = 9999.;



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

/// Spawn bottle at the origin (x:0,y:0)
fn spawn_bottle(
    mut commands: Commands,
    assets: Res<GameAssets>,
) {
    let bottle_center = assets.bottle_center();
    let bottle_outer_size = assets.bottle_outer_size();
    // Spawn Bottle
    commands.spawn((
        Bottle {
            origin: bottle_center,
        },
        RigidBody::Kinematic,
        SpatialBundle {
            transform: Transform::from_translation(bottle_center.extend(Z_WALL)),
            ..default()
        },
    ))
    .with_children(|b| {
        // fg
        b.spawn((
            SpriteBundle {
                texture: assets.bottle_settings.h_fg_image.clone(),
                sprite: Sprite {
                    custom_size: Some(bottle_outer_size),
                    ..default()
                },
                transform: Transform::from_translation(Vec2::ZERO.extend(0.02)),
                ..default()
            },
            ImageScaleMode::Sliced(TextureSlicer {
                border: BorderRect::square(30.),
                center_scale_mode: SliceScaleMode::Stretch,
                sides_scale_mode: SliceScaleMode::Stretch,
                ..default()
            }),
        ));

        // bg
        b.spawn((
            SpriteBundle {
                texture: assets.bottle_settings.h_bg_image.clone(),
                sprite: Sprite {
                    custom_size: Some(bottle_outer_size),
                    ..default()
                },
                transform: Transform::from_translation(Vec2::ZERO.extend(Z_BACK-Z_WALL+0.01)),
                ..default()
            },
            ImageScaleMode::Sliced(TextureSlicer {
                border: BorderRect::square(30.),
                center_scale_mode: SliceScaleMode::Stretch,
                sides_scale_mode: SliceScaleMode::Stretch,
                ..default()
            }),
        ));

        let bottom_c = Vec2::Y * -(0.5 * bottle_outer_size.y - assets.bottle_settings.thickness/2.);
        let left_bottle_c = Vec2::X * -(0.5 * bottle_outer_size.x - assets.bottle_settings.thickness/2.);
        let right_bottle_c = Vec2::X * (0.5 * bottle_outer_size.x - assets.bottle_settings.thickness/2.);

        let bottom_size = assets.bottle_settings.bottom_size();
        let len_bottom = bottom_size.x - bottom_size.y;
        let r_bottom = bottom_size.y/2.;
        let side_size = assets.bottle_settings.side_size();
        let len_side = side_size.y - side_size.x;
        let r_side = side_size.x/2.;

        let physics_param = (
            Restitution {
                coefficient: assets.bottle_physics.restitution.coef,
                ..default()
            },
            Friction {
                dynamic_coefficient: assets.bottle_physics.friction.dynamic_coef,
                static_coefficient: assets.bottle_physics.friction.static_coef,
                ..default()
            },
        );

        // Bottom
        b.spawn((
            BottleWall,
            Collider::capsule(len_bottom, r_bottom),
            physics_param,
            TransformBundle {
                local: Transform::from_translation(bottom_c.extend(0.))
                    .with_rotation(Quat::from_rotation_z(PI/2.)),
                ..default()
            },
        ));

        // Left bottle
        b.spawn((
            BottleWall,
            Collider::capsule(len_side, r_side),
            physics_param,
            TransformBundle {
                local: Transform::from_translation(left_bottle_c.extend(0.)),
                ..default()
            },
        ));

        // Right bottle
        b.spawn((
            BottleWall,
            Collider::capsule(len_side, r_side),
            physics_param,
            TransformBundle {
                local: Transform::from_translation(right_bottle_c.extend(0.)),
                ..default()
            },
        ));
    });
}

/// Spawn background image at the center of the default camera position with offset.
fn spawn_background(
    mut commands: Commands,
    assets: Res<GameAssets>,
    config: Res<FixedConfig>,
) {
    let center_xy = config.playing_cam_offset;
    let offset = center_xy + assets.background.offset;
    // BACK
    commands.spawn((
        Background,
        SpriteBundle {
            texture: assets.background.h_bg_image.clone(),
            sprite: Sprite {
                custom_size: None,
                ..default()
            },
            transform: Transform::from_translation(offset.extend(Z_BACK)),
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
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameScreenState>>,
    q_balls: Query<(Entity, &Transform), With<Ball>>,
    config: Res<FixedConfig>,
) {
    let Area { min_x, max_x, min_y, max_y } = config.area;
    if let Some((entity, ball)) = q_balls.iter().find(|(_, t)| {
        let t = t.translation;
        let x = t.x;
        let y = t.y;
        !(min_x..=max_x).contains(&x) ||
            !(min_y..=max_y).contains(&y)
    }) {
        info!("Game over: {:?} / {:?}", ball, config.area);
        commands.entity(entity)
            .insert(AreaProtruded);
        next_state.set(GameScreenState::GameOver);
    }
}


/// Player inputs
#[derive(Event, Debug, Clone, Copy, PartialEq)]
enum PlayerInputEvent {
    Drop,
    Move(f32), // [-1, 1]
    Hold,
    Shake(Vec2),
    Pause,
}

fn read_keyboard_for_player_actions(
    q_player: Query<&Player>,
    keyboard: Res<ButtonInput<KeyCode>>,

    mut ev_player_act: EventWriter<PlayerInputEvent>,
) {
    if q_player.get_single().is_ok() {
        let mut lr = 0.;
        if keyboard.any_pressed(KEYBOARD_KEYS_LEFT) {
            lr += -1.;
        }
        if keyboard.any_pressed(KEYBOARD_KEYS_RIGHT) {
            lr += 1.;
        }

        if lr != 0. {
            ev_player_act.send(PlayerInputEvent::Move(lr));
        }

        if keyboard.any_just_pressed(KEYBOARD_KEYS_MAIN) {
            ev_player_act.send(PlayerInputEvent::Drop);
        }

        if keyboard.any_just_pressed(KEYBOARD_KEYS_SUB1) {
            ev_player_act.send(PlayerInputEvent::Hold);
        }

        if keyboard.any_just_pressed(KEYBOARD_KEYS_SUB2) {
            ev_player_act.send(PlayerInputEvent::Shake(Vec2::new(0., 1.)));
        }

        if keyboard.any_just_pressed(KEYBOARD_KEYS_START) {
            ev_player_act.send(PlayerInputEvent::Pause);
        }
    }
}



fn read_gamepad_for_player_actions(
    q_player: Query<&Player>,
    connected_gamepad: Option<Res<ConnectedGamePad>>,
    axes: Res<Axis<GamepadAxis>>,
    buttons: Res<ButtonInput<GamepadButton>>,

    mut ev_player_act: EventWriter<PlayerInputEvent>,
) {
    if q_player.get_single().is_ok() {
        if let Some(&ConnectedGamePad(gamepad)) = connected_gamepad.as_deref() {
            let axis_lx = GamepadAxis {
                gamepad,
                axis_type: GamepadAxisType::LeftStickX,
            };

            let button = |btns: &[GamepadButtonType]| {
                btns.iter().map(|btn|
                    GamepadButton::new(gamepad, *btn)
                ).collect_vec()
            };

            let mut lr = if let Some(lx) = axes.get(axis_lx) {
                lx
            } else {
                0.
            };

            if buttons.any_pressed(button(&GAMEPAD_BTNS_LEFT)) {
                lr -= 1.;
            }
            if buttons.any_pressed(button(&GAMEPAD_BTNS_RIGHT)) {
                lr += 1.;
            }

            if lr != 0. {
                ev_player_act.send(PlayerInputEvent::Move(lr.clamp(-1., 1.)));
            }

            if buttons.any_just_pressed(button(&GAMEPAD_BTNS_MAIN)) {
                ev_player_act.send(PlayerInputEvent::Drop);
            }

            if buttons.any_just_pressed(button(&GAMEPAD_BTNS_SUB1)) {
                ev_player_act.send(PlayerInputEvent::Hold);
            }

            if buttons.any_just_pressed(button(&GAMEPAD_BTNS_SUB2)) {
                ev_player_act.send(PlayerInputEvent::Shake(Vec2::new(0., 1.)));
            }

            if buttons.any_just_pressed(button(&GAMEPAD_BTNS_START)) {
                ev_player_act.send(PlayerInputEvent::Pause);
            }
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
    let player_y_max = assets.bottle_settings.left_top().y + PLAYER_GAP_WALL + PLAYER_GAP_TO_MAX;
    // puppetter
    commands.spawn((
        PlayerPuppeteer{},
        TransformBundle::from_transform(
            Transform::from_translation(Vec2::new(0., player_y_max).extend(Z_PLAYER))
        ),
        ShapeCaster::new(
            Collider::circle(10.),
            Vec2::ZERO,
            0.,
            Direction2d::NEG_Y
        ),
    ));

    // player
    let player_y = assets.bottle_settings.left_top().y + PLAYER_GAP_WALL;
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
    assets: Res<GameAssets>,
) {
    if let Ok((mut trans, _)) = q_puppeteer.get_single_mut() {
        if let Ok(player) = q_player.get_single() {
            for ev in ev_player_act.read() {
                if let PlayerInputEvent::Move(lr) = ev {
                    let bottle_width = assets.bottle_settings.inner_width;
                    trans.translation.x =
                        (trans.translation.x + lr * player.speed)
                            .clamp(-bottle_width/2., bottle_width/2.);
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
                    sc_asset.bottle_settings.left_top().y + PLAYER_GAP_WALL,
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

fn sync_puppetter_shape_caster(
    mut q_shape_caster: Query<&mut ShapeCaster, With<PlayerPuppeteer>>,
    q_player: Query<&Player, Without<PlayerPuppeteer>>,
    assets: Res<GameAssets>,
) {
    if let Ok(player) = q_player.get_single() {
        if let Ok(mut shape_caster) = q_shape_caster.get_single_mut() {
            let r = assets.get_ball_r(player.next_ball_level);
            shape_caster.shape = Collider::circle(r);
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
    assets: Res<GameAssets>,
) {
    // 1st: Origin/Visibility of Guide
    let player_trans = set.p2().get_single().map(|(o, p)| (o.translation.x, o.translation.y, p.is_fakeball_exists(), assets.get_ball_r(p.next_ball_level)));
    if let Ok((player_x, player_y, has_fake_ball, _)) = player_trans {
        if let Ok((mut trans, mut vis)) = set.p0().get_single_mut() {
            trans.translation.x = player_x;
            trans.translation.y = player_y;
            *vis = if has_fake_ball { Visibility::Visible } else { Visibility::Hidden };
        }
    }

    // 2nd: Guide Body Length
    let puppetter_data = set.p3().get_single().map(|(t, _sc, hits)| (t.translation.y, get_shortest_hit(hits).cloned()));
    if let (Ok((puppetter_y, Some(hit))), Ok((_, player_y, _, r))) = (puppetter_data, player_trans) {
        if let Ok(mut trans) = set.p1().get_single_mut() {
            let len = hit.time_of_impact - (puppetter_y - player_y) + r;
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

                        // Dropping balls in the same position makes them a "totem".
                        // Therefore, we add a small random value to the drop position x.
                        let jitter = -0.5 + rng.next_u32() as f32 / std::u32::MAX as f32;

                        ev_ball_spawn.send(BallSpawnEvent::Drop(pos + Vec2::X * jitter, lv));

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
                PlayerInputEvent::Shake(_) => {
                },
                PlayerInputEvent::Pause => {
                },
            }
        }
    }
}

#[derive(Component, Debug)]
struct Shaking(Vec<(Vec2, Timer)>);

fn shake_y(s: f32) -> f32 {
    let x = s*8. - 4.;
    (1. / (2.*PI).sqrt()) * std::f32::consts::E.powf(- x*x / 2.)
}

fn shake_bottle(
    mut commands: Commands,
    mut q_bottle: Query<(Entity, &Bottle, &mut Transform, Option<&mut Shaking>)>,
    mut ev_player_act: EventReader<PlayerInputEvent>,
    time: Res<Time>,
    config: Res<FixedConfig>,
) {
    let delta = time.delta();
    let new_shakes = ev_player_act.read()
        .filter_map(|ev| {
            if let PlayerInputEvent::Shake(v) = ev {
                Some(v)
            } else {
                None
            }
        })
        .map(|&v| (v, Timer::from_seconds(1.0, TimerMode::Once)));
    if let Ok((bottle_entity, bottle, mut bottle_trans, shaking)) = q_bottle.get_single_mut() {
        let iter = if let Some(mut shaking) = shaking {
            shaking.0.iter_mut().for_each(|t| {t.1.tick(delta);});
            shaking.0.retain(|(_,t)| !t.finished());
            shaking.0.append(&mut new_shakes.collect());
            shaking.0.clone()
        } else {
            let shakes = new_shakes.collect_vec();
            commands.entity(bottle_entity)
                .insert(Shaking(shakes.clone()));
            shakes
        };

        let max_y = config.shake_k * iter.iter().map(|(v,t)| shake_y(v.y * t.elapsed_secs())).reduce(f32::max).unwrap_or(0.);

        bottle_trans.translation.y = bottle.origin.y - max_y;
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
    config: Res<Config>,
    scores: Res<Scores>,
) {
    let game_cnd = GameCond::new(&config.game_ron_name);
    let highscore = scores.get_highest(&game_cnd);
    let high_score_txt = format!("high score:{:>8}", highscore.unwrap_or(&default()).score);


    let border_width = my_assets.ui.score_view.border_width;
    let inner_margin = 4.;
    let label_weight = FONT_WEIGHT_L;
    let score_weight = FONT_WEIGHT_L;
    let high_score_weight = FONT_WEIGHT_S;
    let score_size = my_assets.score_size();
    let score_center = my_assets.score_center();
    commands
        .spawn((
            ScoreView,
            SpriteBundle { // as frame
                texture: my_assets.ui.score_view.h_bg_image.clone(),
                sprite: Sprite {
                    custom_size: Some(score_size),
                    ..default()
                },
                transform: Transform::from_translation(
                               score_center.extend(Z_UI)),
                ..default()
            },
            ImageScaleMode::Sliced(TextureSlicer {
                border: BorderRect::square(border_width),
                center_scale_mode: SliceScaleMode::Tile { stretch_value: 1.0 },
                sides_scale_mode: SliceScaleMode::Tile { stretch_value: 1.0 },
                ..default()
            }),
        ))
        .with_children(|b| {
            let label_pos =
                Vec2::new(
                    0.,
                    score_size.y/2.- label_weight/2. - border_width - inner_margin
                );
            let score_pos =
                label_pos +
                Vec2::new(
                    0.,
                    -label_weight - inner_margin,
                );
            let high_score_pos =
                score_pos +
                Vec2::new(
                    0.,
                    - score_weight - inner_margin,
                );

            let text_style = TextStyle {
                font: my_assets.h_font.clone(),
                font_size: label_weight,
                color: my_assets.ui.score_view.font_color,
            };
            b.spawn((
                Text2dBundle {
                    text: Text::from_section("SCORE", text_style.clone()),
                    transform: Transform::from_translation(label_pos.extend(0.01)),
                    ..default()
                },
            ));
            let text_style = TextStyle {
                font: my_assets.h_font.clone(),
                font_size: score_weight,
                color: my_assets.ui.score_view.font_color,
            };
            b.spawn((
                ScoreText,
                Text2dBundle {
                    text: Text::from_section("0", text_style.clone()),
                    transform: Transform::from_translation(score_pos.extend(0.01)),
                    ..default()
                },
            ));
            let text_style = TextStyle {
                font: my_assets.h_font.clone(),
                font_size: high_score_weight,
                color: my_assets.ui.score_view.font_color,
            };
            b.spawn((
                Text2dBundle {
                    text: Text::from_section(high_score_txt, text_style.clone()),
                    transform: Transform::from_translation(high_score_pos.extend(0.01)),
                    ..default()
                },
            ));
        });
}

#[derive(Component, Debug)]
struct ManualView;
#[derive(Component, Debug)]
struct ManualViewText;

fn spawn_manual_view(
    mut commands: Commands,
    my_assets: Res<GameAssets>,
) {
    let border_width = my_assets.ui.manual_view.border_width;
    let inner_margin = 4.;
    let font_weight = FONT_WEIGHT_M;
    let font_weight_p = FONT_WEIGHT_S;
    let manual_size = my_assets.manual_view_size();
    let manual_center = my_assets.manual_view_center();
    commands
        .spawn((
            ManualView,
            SpriteBundle { // as frame
                texture: my_assets.ui.manual_view.h_bg_image.clone(),
                sprite: Sprite {
                    custom_size: Some(manual_size),
                    ..default()
                },
                transform: Transform::from_translation(
                               manual_center.extend(Z_UI)),
                ..default()
            },
            ImageScaleMode::Sliced(TextureSlicer {
                border: BorderRect::square(border_width),
                center_scale_mode: SliceScaleMode::Tile { stretch_value: 1.0 },
                sides_scale_mode: SliceScaleMode::Tile { stretch_value: 1.0 },
                ..default()
            }),
        ))
        .with_children(|b| {
            let pos1 =
                Vec2::new(
                    0.,
                    manual_size.y/2.- font_weight/2. - border_width - inner_margin
                );
            let pos2 =
                pos1 +
                Vec2::new(
                    0.,
                    -font_weight - inner_margin,
                );
            let pos3 =
                pos2 +
                Vec2::new(
                    0.,
                    -font_weight - inner_margin,
                );

            let text_style = TextStyle {
                font: my_assets.h_font.clone(),
                font_size: font_weight,
                color: my_assets.ui.manual_view.font_color,
            };
            let text_style_p = TextStyle {
                font: my_assets.h_font.clone(),
                font_size: font_weight_p,
                color: my_assets.ui.manual_view.font_color,
            };
            b.spawn((
                ManualViewText,
                Text2dBundle {
                    text: Text::from_sections([
                        TextSection::new("Move", text_style.clone()),
                        TextSection::new(format!("[{}]", GpKbInput::MoveLeftRight.get_str()), text_style_p.clone()),
                    ]),
                    transform: Transform::from_translation(pos1.extend(0.01)),
                    ..default()
                },
            ));
            b.spawn((
                ManualViewText,
                Text2dBundle {
                    text: Text::from_sections([
                        TextSection::new("Shake", text_style.clone()),
                        TextSection::new(format!("[{}]", GpKbInput::Sub2.get_str()), text_style_p.clone()),
                    ]),
                    transform: Transform::from_translation(pos2.extend(0.01)),
                    ..default()
                },
            ));
            b.spawn((
                ManualViewText,
                Text2dBundle {
                    text: Text::from_sections([
                        TextSection::new("Pause", text_style.clone()),
                        TextSection::new(format!("[{}]", GpKbInput::Start.get_str()), text_style_p.clone()),
                    ]),
                    transform: Transform::from_translation(pos3.extend(0.01)),
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
    let border_width = my_assets.ui.hold_view.border_width;
    let inner_margin = 4.;
    let label_weight = FONT_WEIGHT_L;
    let plus_weight = FONT_WEIGHT_S;
    let size = my_assets.hold_view_size();
    commands.spawn((
        HoldingBallView,
        SpriteBundle {
            texture: my_assets.ui.hold_view.h_bg_image.clone(),
            sprite: Sprite {
                custom_size: Some(size),
                ..default()
            },
            transform: Transform::from_translation(
                           my_assets.hold_view_center().extend(Z_UI)),
            ..default()
        },
        ImageScaleMode::Sliced(TextureSlicer {
            border: BorderRect::square(border_width),
            center_scale_mode: SliceScaleMode::Tile { stretch_value: 1.0 },
            sides_scale_mode: SliceScaleMode::Tile { stretch_value: 1.0 },
            ..default()
        }),
    ))
    .with_children(|b| {
        let label_pos =
            Vec2::new(0., size.y/2.- label_weight/2. - border_width - inner_margin)
            ;
        let image_pos =
            Vec2::new(0., -label_weight/2.);
        let text_style = TextStyle {
            font: my_assets.h_font.clone(),
            font_size: label_weight,
            color: my_assets.ui.hold_view.font_color,
        };
        let text_style_p = TextStyle {
            font: my_assets.h_font.clone(),
            font_size: plus_weight,
            color: my_assets.ui.hold_view.font_color,
        };
        b.spawn((
            Text2dBundle {
                text: Text::from_sections([
                    TextSection::new("Hold", text_style.clone()),
                    TextSection::new(format!("[{}]", GpKbInput::Sub1.get_str()), text_style_p.clone()),
                ]),
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
                if *fakeball_level != player.next_ball_level { // Holding a ball causes this.
                    // update
                    let ball_view = create_ball_view_for_fake(
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
                        let ball_view = create_ball_view_for_fake(
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
                score_text.value = format!("{:>8}", player.score);
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

fn create_ball_view_base(
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,

    level: BallLevel,
    pos: Vec2,

    my_assets: &Res<GameAssets>,
) -> MaterialMesh2dBundle<ColorMaterial> {

    let ball_material = materials.add(my_assets.get_ball_image(level).clone());
    let (mesh_w, mesh_h) = my_assets.get_ball_mesh_wh(level);
    MaterialMesh2dBundle {
        mesh: meshes.add(Rectangle::new(mesh_w, mesh_h)).into(),
        transform: Transform::from_translation(
             pos.extend(0.0)
        ),
        material: ball_material,
        ..default()
    }
}

fn create_ball_view_for_fake(
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,

    level: BallLevel,
    pos: Vec2,

    my_assets: &Res<GameAssets>,
) -> impl Bundle {
    let mut b = create_ball_view_base(meshes, materials, level, pos, my_assets);
    b.transform.translation.z = 0.1;
    b
}

fn create_ball_view(
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,

    level: BallLevel,
    pos: Vec2,

    my_assets: &Res<GameAssets>,
) -> impl Bundle {
    let mut b = create_ball_view_base(meshes, materials, level, pos, my_assets);
    b.transform.translation.z = Z_BALL + Z_BALL_D_BY_LEVEL * level.0 as f32;
    b
}

fn spawn_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,

    mut ev_ball_spawn: EventReader<BallSpawnEvent>,
    my_assets: Res<GameAssets>,
    config: Res<FixedConfig>,
) {
    for ev in ev_ball_spawn.read() {
        use BallSpawnEvent::*;
        let physics_param = (
            Restitution {
                coefficient: my_assets.ball_physics.restitution.coef,
                ..default()
            },
            Friction {
                dynamic_coefficient: my_assets.ball_physics.friction.dynamic_coef,
                static_coefficient: my_assets.ball_physics.friction.static_coef,
                ..default()
            },
        );
        match *ev {
            Drop(pos, level) => {
                let ball_view = create_ball_view(&mut meshes, &mut materials,
                                                 level, pos, &my_assets);
                commands.spawn((
                    DroppingBall,
                    Ball::new(level),
                    RigidBody::Dynamic,
                    Collider::circle(my_assets.get_ball_r(level)),
                    physics_param,
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
                    physics_param,
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
                sc_assets.sound.h_se_combine.clone(),
                config.get_se_volume(sc_assets.sound.se_combine_scale),
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
            With<ManualView>,
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
            sc_asset.sound.h_bgm.clone(),
            config.get_bgm_volume(sc_asset.sound.bgm_scale),
        );
    }
}

fn pause_game(
    mut events: EventReader<PlayerInputEvent>,
    mut next_state: ResMut<NextState<GameScreenState>>,
) {
    for event in events.read() {
        if matches!(event, PlayerInputEvent::Pause) {
            next_state.set(GameScreenState::Paused);
        }
    }
}
