use std::f32::consts::TAU;
use bevy::{
    asset::LoadState, prelude::*, reflect::Reflect, sprite::{Material2d, MaterialMesh2dBundle, Mesh2dHandle}, window::WindowResolution
};
use bevy_xpbd_2d::prelude::*;
use itertools::Itertools;

use bevy_console::{reply, AddConsoleCommand, ConsoleCommand, ConsoleConfiguration, ConsolePlugin};
use clap::Parser;

//
// ToDo Items
// - (ALWAYS) Refactoring!
// - [x] Remove Max Level Balls Combined.
// - [x] Scoring:
//   - [x] Combine Scores.
//   - [x] Drop Scores.
// - [ ] Player position:
//   - [ ] y-position should be higher than all of balls.
//   - [ ] x-position should be limited x positon to the inside of the bottle.
// - [ ] Use random generator.
// - [ ] GameOver.
// - [ ] Create and Load an external file (.ron or others)
//   for ball size, texture, and other data.
// - [ ] Sound.
//   - [ ] BGM.
//   - [ ] SE.
// - [ ] Title Screen.
// - [ ] Player texture.
// - [ ] Config Screen.
// - [ ] Player Actions.
//   - [ ] Holding a ball.
//   - [ ] Shaking the bottle.
//


// Window Settings
const TITLE: &str = "Suikx clone";
const LOGICAL_WIDTH: f32 = 1440.;
const LOGICAL_HEIGHT: f32 = 940.;
const WINDOW_MIN_WIDTH: f32 = LOGICAL_WIDTH;
const WINDOW_MIN_HEIGHT: f32 = LOGICAL_HEIGHT;
const WINDOW_MAX_WIDTH: f32 = 1920.;
const WINDOW_MAX_HEIGHT: f32 = 1080.;

// Physics Engine Settings
const GRAVITY_SCALE: f32 = 9.81 * 100.;
const XPBD_SUBSTEP: u32 = 24;

#[derive(States, Default, Hash, Clone, Copy, PartialEq, Eq, Debug)]
enum GameState {
    #[default]
    Loading,
    InGame,
}

fn main() {
    #[cfg(target_family = "windows")]
    std::env::set_var("RUST_BACKTRACE", "1"); // Can't read env values when running on WSL

    let mut app = App::new();

    app.add_plugins((
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: TITLE.into(),
                name: Some(TITLE.into()),
                resolution: WindowResolution::new(LOGICAL_WIDTH, LOGICAL_HEIGHT),
                resize_constraints: WindowResizeConstraints {
                    min_width: WINDOW_MIN_WIDTH,
                    min_height: WINDOW_MIN_HEIGHT,
                    max_width: WINDOW_MAX_WIDTH,
                    max_height: WINDOW_MAX_HEIGHT,
                },
                ..default()
            }),
            ..default()
        }),
        PhysicsPlugins::default(),

        PhysicsDebugPlugin::default(),

        ConsolePlugin,
    ));

    // Setup for bevy_console
    app.insert_resource(ConsoleConfiguration {
        keys: vec![
            KeyCode::F1,
        ],
        ..default()
    });
    app.add_console_command::<PrintConfigCommand, _>(command_print_config);
    app.add_console_command::<GrowCommand, _>(command_grow);

    app.init_state::<GameState>();

    app.init_gizmo_group::<MyLoadingScreenGizmos>();

    app.insert_resource(Gravity(Vec2::NEG_Y * GRAVITY_SCALE));
    app.insert_resource(SubstepCount(XPBD_SUBSTEP));
    app.insert_resource(Config::default());

    app.add_event::<BallEvent>();
    app.add_event::<PlayerActionEvent>();
    app.add_event::<BallSpawnEvent>();

    app.add_systems(Startup, (
        setup_camera,
    ));
    app.add_systems(OnEnter(GameState::Loading), (
        load_assets,
        setup_loading_screen,
    ));
    app.add_systems(Update, (
        check_loading,
        update_loading_screen,
    ).run_if(in_state(GameState::Loading)));
    app.add_systems(OnExit(GameState::Loading), (
        cleanup_loading_screen,
    ));

    app.add_systems(OnEnter(GameState::InGame), (
        setup_bottle,
        spawn_player,
        spawn_score_view,
    ));

    app.add_systems(Update, (
        grow_ball_spawned,
        check_ball_collisions,
        action_player,
        combine_balls_touched
            .after(check_ball_collisions),
        spawn_ball
            .after(action_player)
            .after(combine_balls_touched),
        update_player_view
            .after(action_player),
        limit_velocity_of_ball, // TODO: should exec after velocities are caluculated
        score_ball_events,
    ).run_if(in_state(GameState::InGame)));

    app.add_systems(FixedUpdate, (
        read_keyboard,
    ).run_if(in_state(GameState::InGame)));

    app.run();
}

#[derive(Resource, Debug)]
struct MyAssets {
    h_balls: Vec<Handle<Image>>,
    h_font: Handle<Font>,
}
impl MyAssets {
    fn get_untyped_handles(&self) -> Vec<UntypedHandle> {
        let mut v: Vec<_> = self.h_balls.iter().cloned().map(|h| h.untyped()).collect();
        let mut v2 = vec![
            self.h_font.clone().untyped(),
        ];
        v.append(&mut v2);
        v
    }

    fn get_ball_image(&self, level: BallLevel) -> &Handle<Image> {
        let idx = level.0 - BALL_LEVEL_MIN;
        &self.h_balls[idx]
    }
}


#[derive(Component, Debug)]
struct Player {
    speed: f32,
    next_ball_level: BallLevel,

    timer_cooltime: Timer,

    hand_offset: Vec2,

    score: u32,
}

const PLAYER_SPEED: f32 = 3.0;
const PLAYER_DROP_COOLTIME: f32 = 1.0;

impl Default for Player {
    fn default() -> Self {
        Self {
            speed: PLAYER_SPEED,
            next_ball_level: default(),

            timer_cooltime: Timer::from_seconds(PLAYER_DROP_COOLTIME, TimerMode::Once),
            hand_offset: Vec2::ZERO,

            score: 0,
        }
    }
}
impl Player {
    fn new(speed: f32, sec_cooltime: f32, hand_offset: Vec2, first_ball_level: BallLevel) -> Self {
        Self {
            speed,
            next_ball_level: first_ball_level,
            timer_cooltime: Timer::from_seconds(sec_cooltime, TimerMode::Once),
            hand_offset,
            ..default()
        }
    }
    fn set_next_ball_level(&mut self, /* randam generator here? */) {
        let now = self.next_ball_level;
        self.next_ball_level = BallLevel::new(
            ((now.0 + 1731) % 101) % 4usize + BALL_LEVEL_MIN

            );
    }
}

#[derive(Component, Debug)]
struct Wall;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct BallLevel(usize);
const BALL_LEVEL_MIN:usize = 1;
const BALL_LEVEL_MAX:usize = 11;
struct BallLevelSetting {
    radius: f32,
    _color: Color,
}
impl BallLevelSetting {
    const fn new(radius: f32, color: Color) -> Self {
        Self { radius, _color: color }
    }
}
const fn color(lv: BallLevel) -> Color {
    let idx = lv.0 - BALL_LEVEL_MIN;
    let x = (idx * 360) / (BALL_LEVEL_MAX-BALL_LEVEL_MIN+1);
    let h = x as f32;
    let s = 1.0;
    let l = 0.5;
    Color::hsl(h, s, l)
}
const BALL_LEVEL_SETTINGS: [BallLevelSetting; BALL_LEVEL_MAX-BALL_LEVEL_MIN+1] =
[
    BallLevelSetting::new(028.0, color(BallLevel(1))),
    BallLevelSetting::new(034.5, color(BallLevel(2))),
    BallLevelSetting::new(043.5, color(BallLevel(3))),
    BallLevelSetting::new(055.0, color(BallLevel(4))),
    BallLevelSetting::new(069.0, color(BallLevel(5))),
    BallLevelSetting::new(086.0, color(BallLevel(6))),
    BallLevelSetting::new(105.0, color(BallLevel(7))),
    BallLevelSetting::new(127.0, color(BallLevel(8))),
    BallLevelSetting::new(151.0, color(BallLevel(9))),
    BallLevelSetting::new(177.5, color(BallLevel(10))),
    BallLevelSetting::new(207.0, color(BallLevel(11))),
];
impl Default for BallLevel {
    fn default() -> Self {
        Self(BALL_LEVEL_MIN)
    }
}
impl BallLevel {
    fn new(lv: usize) -> Self {
        assert!(lv >= BALL_LEVEL_MIN);
        assert!(lv <= BALL_LEVEL_MAX);
        Self(lv)
    }
    fn get_settings(&self) -> &'static BallLevelSetting {
        &BALL_LEVEL_SETTINGS[self.0 - BALL_LEVEL_MIN]
    }

    fn get_r(&self) -> f32 {
        self.get_settings().radius
    }
    fn _get_color(&self) -> Color {
        self.get_settings()._color
    }

    fn get_growstart_r(&self) -> f32 {
        //                               
        //   -  *                        
        //   |  ***                      
        //   |  *  **  y+r      r: min(ball radius)
        //   y  *    **         y: combined ball radius
        //   |  *      **       x: max radius of new free space <- this!
        //   |  *        **              
        //   |  *   x+r    **            
        //   =  *------------*           
        //   |  *          **            
        //   |  *        **              
        //   y  *      **                
        //   |  *    **                  
        //   |  *  **                    
        //   |  ***                      
        //   _  *                        
        //                               
        let r = BallLevel::new(BALL_LEVEL_MIN).get_r();
        let y = BallLevel::new(self.0 - 1).get_r();

        (2. * r * y + r * r).powf(1. / 2.) - r
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

// # Screen Layout
// +: (0.0, 0.0)
//                                             
//                      P                      
//                               SCORE: xx     
//                *         *                  
//                *         *                  
//                *         *    sample+       
//                *    +    *    |     |       
//                *         *    | Lv1 |       
//                *         *    | Lv2 |       
//                *         *    | ... |       
//                **bottle***    +-----+       
//                                             
//                                             
//                                             
const BOTTOLE_MARGIN_RIGHT: f32 = 60.;
const SCORE_TEXT_WEIGHT: f32 = 30.;
const SCORE_WIDTH: f32 = 360.; // "Score: 12345" (12) 12 * 30.
const SCORE_HEIGHT: f32 = 40.;


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
const BOTTLE_WIDTH: f32 = 740.0;
const BOTTLE_HEIGHT: f32 = 740.0;
const BOTTLE_THICKNESS: f32 = 30.0;

const BOTTOM_SIZE: Vec2 = Vec2::new(BOTTLE_WIDTH + 2.*BOTTLE_THICKNESS, BOTTLE_THICKNESS);
const SIDE_SIZE: Vec2 = Vec2::new(BOTTLE_THICKNESS, BOTTLE_HEIGHT);

const WALL_OUTER_LEFT_TOP: Vec2 = Vec2::new(
        -1. * BOTTOM_SIZE.x * 0.5,
        -1. * -SIDE_SIZE.y * 0.5,
    );
const WALL_OUTER_RIGHT_BOTTOM: Vec2 = Vec2::new(
        BOTTOM_SIZE.x * 0.5,
        -SIDE_SIZE.y * 0.5,
    );

const PLAYER_GAP_WALL: f32 = 50.;
const PLAYER_Y: f32 = WALL_OUTER_LEFT_TOP.y + PLAYER_GAP_WALL;

// Z-Order
//   These are layers. each layer can freely use +[0.0, 1.0) Z-Order for any purpose.
const Z_BACK: f32 = -20.;
const Z_SCORE: f32 = -10.;
const Z_WALL: f32 = 00.;
const Z_PLAYER: f32 = 10.;
const Z_BALL: f32 = 20.;

const Z_BALL_D_BY_LEVEL: f32 = 0.01;

fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {

    let mut h_balls = vec![];
    for i in BALL_LEVEL_MIN..=BALL_LEVEL_MAX {
        h_balls.push(
            asset_server.load(format!("images/kao/kao_{:>02}.png", i))
        );
    }
    commands.insert_resource(
        MyAssets {
            h_balls,
            h_font: asset_server.load("fonts/GL-CurulMinamoto.ttf"),
        }
    );
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
enum LoadingState {
    Ok,
    Loading,
    Error,
}
fn summarise_assetpack_loadstate(
    asset_pack: &MyAssets,
    asset_server: &AssetServer,
) -> LoadingState {
    asset_pack.get_untyped_handles()
        .iter()
        .map(|h| asset_server.get_load_states(h.id()))
        .filter_map(|s| s.map(|(s, _, _)| s))
        .fold(LoadingState::Ok, |a, s| {
            let s = match s {
                LoadState::Loaded => LoadingState::Ok,
                LoadState::Failed => LoadingState::Error,
                _ => LoadingState::Loading,
            };
            LoadingState::max(a, s)
        })
}

fn check_loading(
    asset_pack: Res<MyAssets>,
    asset_server: Res<AssetServer>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let state = summarise_assetpack_loadstate(&asset_pack, &asset_server);
    match state {
        LoadingState::Ok => {
            next_state.set(GameState::InGame);
        }
        LoadingState::Loading => {
            // wait for next
        }
        LoadingState::Error => {
            panic!("load failed!");
        }
    }
}

#[derive(Component, Debug)]
struct ForLoadingScreen;
#[derive(GizmoConfigGroup, Default, Reflect)]
struct MyLoadingScreenGizmos {}

fn setup_loading_screen(
    mut commands: Commands,
    mut config_gizmos: ResMut<GizmoConfigStore>,
) {
    commands.spawn((
        ForLoadingScreen,
        SpriteBundle {
            sprite: Sprite {
                color: Color::BLACK,
                custom_size: Some(Vec2::new(600., 300.)),
                ..default()
            },
            transform: Transform::from_translation(Vec2::new(0., 0.).extend(0.0)),
            ..default()
        },
    )).with_children(|b| {
        let text_style = TextStyle {
            font_size: 60.0,
            color: Color::WHITE,
            ..default()
        };
        b.spawn((
            Text2dBundle {
                text: Text::from_section("Now Loading...", text_style),
                transform: Transform::from_translation(Vec2::new(0., 0.).extend(0.1)),
                text_anchor: bevy::sprite::Anchor::BottomRight,
                ..default()
            },
        ));
    });

    let (config, ..) = config_gizmos.config_mut::<MyLoadingScreenGizmos>();
    config.line_width = 5.;
}

fn update_loading_screen(
    mut gizmos: Gizmos<MyLoadingScreenGizmos>,
    time: Res<Time>,
) {
    let second_hand = (time.elapsed_seconds() % 1.0) * TAU;
    gizmos.arrow_2d(
        Vec2::ZERO,
        Vec2::from_angle(second_hand) * 100.,
        Color::YELLOW,
    );
}

fn cleanup_loading_screen(
    mut commands: Commands,
    q_screen_items: Query<Entity, With<ForLoadingScreen>>,
) {
    for e in q_screen_items.iter() {
        commands.entity(e)
            .despawn_recursive();
    }
}

fn setup_camera(
    mut commands: Commands,
) {
    commands.spawn((
        Camera2dBundle::default(),
    ));
}


fn setup_bottle(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let outer_l_t = WALL_OUTER_LEFT_TOP;
    let bottom_l_t = Vec2::new(0., -BOTTLE_HEIGHT) + outer_l_t;
    let left_bottle_l_t = outer_l_t;
    let right_bottle_l_t = Vec2::new(BOTTLE_WIDTH + BOTTLE_THICKNESS, 0.) + outer_l_t;

    fn inv_y(v: Vec2) -> Vec2 { Vec2::new(v.x, -v.y) }
    let bottom_c = bottom_l_t + 0.5 * inv_y(BOTTOM_SIZE);
    let left_bottle_c = left_bottle_l_t + 0.5 * inv_y(SIDE_SIZE);
    let right_bottle_c = right_bottle_l_t + 0.5 * inv_y(SIDE_SIZE);

    let bottle_color = Color::RED;
    let bottle_material = materials.add(bottle_color);

    // Bottom
    commands.spawn((
        Wall,
        RigidBody::Static,
        Collider::rectangle(BOTTOM_SIZE.x, BOTTOM_SIZE.y),
        MaterialMesh2dBundle {
            mesh: meshes.add(Rectangle::from_size(BOTTOM_SIZE)).into(),
            transform: Transform::from_translation(bottom_c.extend(Z_WALL)),
            material: bottle_material.clone(),
            ..default()
        },
    ));

    // Left bottle
    commands.spawn((
        Wall,
        RigidBody::Static,
        Collider::rectangle(SIDE_SIZE.x, SIDE_SIZE.y),
        MaterialMesh2dBundle {
            mesh: meshes.add(Rectangle::from_size(SIDE_SIZE)).into(),
            transform: Transform::from_translation(left_bottle_c.extend(Z_WALL)),
            material: bottle_material.clone(),
            ..default()
        },
    ));

    // Right bottle
    commands.spawn((
        Wall,
        RigidBody::Static,
        Collider::rectangle(SIDE_SIZE.x, SIDE_SIZE.y),
        MaterialMesh2dBundle {
            mesh: meshes.add(Rectangle::from_size(SIDE_SIZE)).into(),
            transform: Transform::from_translation(right_bottle_c.extend(Z_WALL)),
            material: bottle_material.clone(),
            ..default()
        },
    ));

    // BACK
    commands.spawn((
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
#[inline]
fn damping(x: f32) -> f32 {
    let k = 0.00025;
    let c = 1.0 / k;

    c - (1.0 / (k * std::f32::consts::E.powf(k * x)))
}

fn limit_velocity_of_ball(
    mut q_ball: Query<(Entity, &mut LinearVelocity), With<Ball>>,
) {
    for (_, mut vel) in q_ball.iter_mut() {
        let l = vel.length();
        if l > 0.1 {
            *vel = ((damping(l) / l) * vel.0).into();
        }
    }
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
                    let cur_lv = b1.get_level().0;

                    if cur_lv == BALL_LEVEL_MAX {
                        ev_ball_spawn.send(
                            BallSpawnEvent::Combine(pos, None));
                    } else {
                        ev_ball_spawn.send(
                            BallSpawnEvent::Combine(pos, Some(BallLevel(cur_lv + 1))));
                    }
                }

            }
        }
    }
}

fn score_ball_events(
    mut q_player: Query<&mut Player>,
    mut ev_ball: EventReader<BallSpawnEvent>,
) {
    if let Ok(mut player) = q_player.get_single_mut() {
        let score: u32 = ev_ball.read()
            .map(|ev| match ev {
                BallSpawnEvent::Drop(_, _level) => {
                    //level.0 as u32 * 1
                    0
                },
                BallSpawnEvent::Combine(_, level) => {
                    let level_combined = level.map(|l| l.0-1).unwrap_or(BALL_LEVEL_MAX);
                    level_combined.pow(2) as u32 * 5
                },
            })
            .sum();
        player.score += score;
    }
}



#[derive(Event, Debug, Clone, Copy, PartialEq)]
enum PlayerActionEvent {
    TryDrop,
    TryMove(f32), // [-1, 1]
}

fn read_keyboard(
    q_player: Query<&Player>,
    keyboard: Res<ButtonInput<KeyCode>>,

    mut ev_player_act: EventWriter<PlayerActionEvent>,
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
            ev_player_act.send(PlayerActionEvent::TryDrop);
        }

        if lr != 0. {
            ev_player_act.send(PlayerActionEvent::TryMove(lr));
        }
    }
}

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let player_tri = Triangle2d::new(
            Vec2::Y * -30.,
            Vec2::new(-30., 30.),
            Vec2::new(30., 30.),
        );

    let player_y = PLAYER_Y;

    commands.spawn((
        Player::new(PLAYER_SPEED, PLAYER_DROP_COOLTIME, Vec2::new(0., -20.0), BallLevel::new(1)),
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(player_tri)),
            material: materials.add(Color::GREEN),
            transform: Transform::from_translation(
                Vec2::new(0., player_y).extend(Z_PLAYER)),
            ..default()
        },
    ));
}

fn action_player(
    mut q_player: Query<(&mut Transform, &mut Player)>,
    mut ev_player_act: EventReader<PlayerActionEvent>,
    mut ev_ball_spawn: EventWriter<BallSpawnEvent>,

    time: Res<Time>,
) {
    if let Ok((mut trans, mut player)) = q_player.get_single_mut() {
        player.timer_cooltime.tick(time.delta());

        for ev in ev_player_act.read() {
            match ev {
                PlayerActionEvent::TryDrop => {
                    if player.timer_cooltime.finished() {
                        let pos = trans.translation.xy() + player.hand_offset;
                        let lv = player.next_ball_level;

                        ev_ball_spawn.send(BallSpawnEvent::Drop(pos, lv));

                        player.set_next_ball_level();
                        player.timer_cooltime.reset();
                    }
                },
                PlayerActionEvent::TryMove(lr) => {
                    trans.translation.x += lr * player.speed;
                },
            }
        }
    }
}

#[derive(Component, Debug)]
struct FakeBall;

#[derive(Component, Debug)]
struct ScoreView;

#[derive(Component, Debug)]
struct ScoreText;

fn spawn_score_view(
    mut commands: Commands,
    my_assets: Res<MyAssets>,
) {
    let score_size = Vec2::new(SCORE_WIDTH, SCORE_HEIGHT);
    let bottom_rt = Vec2::new(
        WALL_OUTER_RIGHT_BOTTOM.x,
        WALL_OUTER_LEFT_TOP.y,
    );
    let score_lt =
        bottom_rt + Vec2::new(BOTTOLE_MARGIN_RIGHT, 0.)
            + (Vec2::new(score_size.x, -score_size.y) / 2.);
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
                               score_lt.extend(Z_SCORE)),
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
                    text: Text::from_section("", text_style.clone()),
                    transform: Transform::from_translation(Vec3::Z * 0.01),
                    ..default()
                },
            ));
        });
}

fn update_player_view(
    q_player: Query<(Entity, &Player)>,

    q_fakeball: Query<Entity, With<FakeBall>>,

    mut q_score_text: Query<&mut Text, With<ScoreText>>,

    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    my_assets: Res<MyAssets>,
) {
    if let Ok((plyer_entity, player)) = q_player.get_single() {
        // Fake ball
        let fakeball = q_fakeball.get_single();

        if player.timer_cooltime.finished() {
            // need fake ball
            if fakeball.is_err() {
                commands.entity(plyer_entity)
                    .with_children(|b| {
                        let ball_view = create_ball_view(&mut meshes, &mut materials, player.next_ball_level, player.hand_offset, &my_assets);
                        b.spawn((
                            FakeBall,
                            ball_view,
                        ));
                    });
            }
        } else {
            // don't need fake ball
            if let Ok(fakeball) = fakeball {
                commands.entity(fakeball).despawn_recursive();
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

    my_assets: &Res<MyAssets>,
) -> impl Bundle {

    let ball_material = materials.add(my_assets.get_ball_image(level).clone());
    let ball_r = level.get_r();
    let tex_k = 512. / 420.;
    MaterialMesh2dBundle {
        mesh: meshes.add(Rectangle::new(ball_r*2.*tex_k, ball_r*2.*tex_k)).into(),
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
    my_assets: Res<MyAssets>,
    config: Res<Config>,
) {
    for ev in ev_ball_spawn.read() {
        use BallSpawnEvent::*;
        match *ev {
            Drop(pos, level) => {
                let ball_view = create_ball_view(&mut meshes, &mut materials, level, pos, &my_assets);
                commands.spawn((
                    Ball::new(level),
                    RigidBody::Dynamic,
                    Collider::circle(level.get_r()),
                    ball_view,
                ));
            },
            Combine(pos, Some(level)) => {
                let ball_r_start = level.get_growstart_r();
                let ball_view = create_ball_view(&mut meshes, &mut materials, level, pos, &my_assets);
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

fn grow_ball_spawned(
    mut commands: Commands,
    mut q_ball: Query<(Entity, &mut BallGrowing, &Ball)>,
    time: Res<Time>,
) {
    for (entity, mut spacer, ball) in q_ball.iter_mut() {
        spacer.sec += time.delta_seconds();

        if spacer.sec > spacer.sec_max {
            commands.entity(entity)
                .try_insert(Collider::circle(ball.get_level().get_r()))
                .remove::<BallGrowing>();
        } else {
            let r_to = ball.get_level().get_r();
            let r_from = ball.get_level().get_growstart_r();
            let r = (spacer.sec / spacer.sec_max) * (r_to - r_from) + r_from;
            commands.entity(entity)
                .try_insert(Collider::circle(r));
        }
    }
}

#[derive(Resource, Debug)]
#[derive(Reflect)]
struct Config {
    grow_time: f32,
}
impl Default for Config {
    fn default() -> Self {
        Self {
            grow_time: 0.2,
        }
    }
}

#[derive(Parser, ConsoleCommand)]
#[command(name = "print_config")]
struct PrintConfigCommand {
}
fn command_print_config(
    mut log: ConsoleCommand<PrintConfigCommand>,
    config: Res<Config>,
) {
    if let Some(Ok(_)) = log.take() {
        reply!(log, "{:?}", *config);
    }
}

#[derive(Parser, ConsoleCommand)]
#[command(name = "grow")]
struct GrowCommand {
    tm: f32,
}
fn command_grow(
    mut log: ConsoleCommand<GrowCommand>,

    mut config: ResMut<Config>,
    ) {
    if let Some(Ok(GrowCommand { tm })) = log.take() {
        config.grow_time = tm;
        reply!(log, "{:?}", *config);
    }
}
