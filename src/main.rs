use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle, Material2d},
    reflect::Reflect,
};
use bevy_xpbd_2d::prelude::*;
use itertools::Itertools;

use bevy_console::{reply, AddConsoleCommand, ConsoleCommand, ConsoleConfiguration, ConsolePlugin};
use clap::Parser;

const GRAVITY_SCALE: f32 = 9.81 * 100.;
const XPBD_SUBSTEP: u32 = 24;

fn main() {
    #[cfg(target_family = "windows")]
    std::env::set_var("RUST_BACKTRACE", "1"); // Can't read env values when running on WSL



    let mut app = App::new();

    app.add_plugins((
        DefaultPlugins,
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
    app.add_console_command::<RpmkCommand, _>(command_rpmk);
    app.add_console_command::<RpkpCommand, _>(command_rpkp);

    app.insert_resource(Gravity(Vec2::NEG_Y * GRAVITY_SCALE));
    app.insert_resource(SubstepCount(XPBD_SUBSTEP));
    app.insert_resource(Config::default());

    app.add_event::<BallEvent>();
    app.add_event::<PlayerActionEvent>();
    app.add_event::<BallSpawnEvent>();

    app.add_systems(Startup, (
        setup_camera,
        setup_wall,

        spawn_player,
    ));

    app.add_systems(Update, (
        owner_set_to_repulsive,
        make_repulsive,

        limit_velocity_of_ball.after(make_repulsive),
        check_ball_collisions,
        action_player,
        combine_balls_touched
            .after(check_ball_collisions),
        spawn_ball
            .after(action_player)
            .after(combine_balls_touched),
    ));

    app.add_systems(FixedUpdate, (
        read_keyboard,
    ));

    app.run();
}


#[derive(Component, Debug)]
struct Player {
    speed: f32,
    next_ball_level: BallLevel,
    cooltime_remained: f32,
    cooltime: f32,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            speed: 1.5,
            next_ball_level: default(),
            cooltime_remained: 1.0,
            cooltime: 1.0,
        }
    }
}
impl Player {
    fn after_drop(&mut self, /* randam generator here? */) {
        let now = self.next_ball_level;
        self.next_ball_level = BallLevel::new(
            ((now.0 + 1731) % 101) % 4usize + BALL_LEVEL_MIN
            );
        self.cooltime_remained = self.cooltime;
    }
}

#[derive(Component, Debug)]
struct Wall;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct BallLevel(usize);
const BALL_LEVEL_MIN:usize = 1;
const BALL_LEVEL_MAX:usize = 11;
const R_FOR_BALL_LEVEL: [f32; BALL_LEVEL_MAX-BALL_LEVEL_MIN + 1]
    = [
        020. * 0.5,
        030. * 0.5,
        035. * 0.5,
        040. * 0.5,
        050. * 0.5,

        060. * 0.5,
        080. * 0.5,
        100. * 0.5,
        130. * 0.5,
        160. * 0.5,

        200. * 0.5,
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
    fn get_r(&self) -> f32 {
        R_FOR_BALL_LEVEL[self.0 - BALL_LEVEL_MIN]
    }
}

#[derive(Component, Debug, Default, PartialEq, Eq)]
struct Ball {
    pub level: BallLevel,
}

impl Ball {
    fn new(level: BallLevel) -> Self {
        Self { level }
    }
    fn get_r(&self) -> f32 {
        self.level.get_r()
    }
}


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
const WALL_WIDTH: f32 = 400.0;
const WALL_HEIGHT: f32 = 500.0;
const WALL_THICKNESS: f32 = 50.0;

const BOTTOM_SIZE: Vec2 = Vec2::new(WALL_WIDTH + 2.*WALL_THICKNESS, WALL_THICKNESS);
const SIDE_SIZE: Vec2 = Vec2::new(WALL_THICKNESS, WALL_HEIGHT);

const WALL_OUTER_LEFT_TOP: Vec2 = Vec2::new(
        -1. * BOTTOM_SIZE.x * 0.5,
        -1. * -SIDE_SIZE.y * 0.5,
    );

const PLAYER_GAP_WALL: f32 = 50.;
const PLAYER_Y: f32 = WALL_OUTER_LEFT_TOP.y + PLAYER_GAP_WALL;

const Z_BACK: f32 = -1.;
const Z_PLAYER: f32 = 0.;
const Z_BALL: f32 = 1.;
const Z_WALL: f32 = 2.;



fn setup_camera(
    mut commands: Commands,
) {
    commands.spawn((
        Camera2dBundle::default(),
    ));
}


fn setup_wall(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let outer_l_t = WALL_OUTER_LEFT_TOP;
    let bottom_l_t = Vec2::new(0., -WALL_HEIGHT) + outer_l_t;
    let left_wall_l_t = outer_l_t;
    let right_wall_l_t = Vec2::new(WALL_WIDTH + WALL_THICKNESS, 0.) + outer_l_t;

    fn inv_y(v: Vec2) -> Vec2 { Vec2::new(v.x, -v.y) }
    let bottom_c = bottom_l_t + 0.5 * inv_y(BOTTOM_SIZE);
    let left_wall_c = left_wall_l_t + 0.5 * inv_y(SIDE_SIZE);
    let right_wall_c = right_wall_l_t + 0.5 * inv_y(SIDE_SIZE);

    let wall_color = Color::RED;
    let wall_material = materials.add(wall_color);

    // Bottom
    commands.spawn((
        Wall,
        RigidBody::Static,
        Collider::rectangle(BOTTOM_SIZE.x, BOTTOM_SIZE.y),
        MaterialMesh2dBundle {
            mesh: meshes.add(Rectangle::from_size(BOTTOM_SIZE)).into(),
            transform: Transform::from_translation(bottom_c.extend(Z_WALL)),
            material: wall_material.clone(),
            ..default()
        },
    ));

    // Left wall
    commands.spawn((
        Wall,
        RigidBody::Static,
        Collider::rectangle(SIDE_SIZE.x, SIDE_SIZE.y),
        MaterialMesh2dBundle {
            mesh: meshes.add(Rectangle::from_size(SIDE_SIZE)).into(),
            transform: Transform::from_translation(left_wall_c.extend(Z_WALL)),
            material: wall_material.clone(),
            ..default()
        },
    ));

    // Right wall
    commands.spawn((
        Wall,
        RigidBody::Static,
        Collider::rectangle(SIDE_SIZE.x, SIDE_SIZE.y),
        MaterialMesh2dBundle {
            mesh: meshes.add(Rectangle::from_size(SIDE_SIZE)).into(),
            transform: Transform::from_translation(right_wall_c.extend(Z_WALL)),
            material: wall_material.clone(),
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
    Combine(Vec2, BallLevel),
}
impl BallSpawnEvent {
    fn get_position(&self) -> Vec2 {
        use BallSpawnEvent::*;
        match self {
            Drop(v, _) => *v,
            Combine(v, _) => *v,
        }
    }
    fn get_level(&self) -> BallLevel {
        use BallSpawnEvent::*;
        match self {
            Drop(_, l) => *l,
            Combine(_, l) => *l,
        }
    }
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
            if b1.1.level == b2.1.level {
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
                    let cur_lv = b1.level.0;

                    if cur_lv == BALL_LEVEL_MAX {
                        ev_ball_spawn.send(
                            BallSpawnEvent::Combine(pos, BallLevel(BALL_LEVEL_MIN)));
                    } else {
                        ev_ball_spawn.send(
                            BallSpawnEvent::Combine(pos, BallLevel(cur_lv + 1)));
                    }
                }

            }
        }
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
        Player::default(),
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
        player.cooltime_remained -= time.elapsed_seconds();
        player.cooltime_remained = if player.cooltime_remained < 0. {
            0.
        } else {
            player.cooltime_remained
        };

        for ev in ev_player_act.read() {
            match ev {
                PlayerActionEvent::TryDrop => {
                    if player.cooltime_remained <= 0. {
                        let pos = trans.translation.xy();
                        let lv = player.next_ball_level;

                        ev_ball_spawn.send(BallSpawnEvent::Drop(pos, lv));

                        player.after_drop();
                    }
                },
                PlayerActionEvent::TryMove(lr) => {
                    trans.translation.x += lr * player.speed;
                },
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

#[derive(Component, Debug)]
struct Repulsive {
    p : f32,
    t : f32,
}

#[derive(Component, Debug)]
struct RepulsiveOwner(Entity);

fn spawn_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,

    mut ev_ball_spawn: EventReader<BallSpawnEvent>,
) {
    for ev in ev_ball_spawn.read() {
        let ball_material = materials.add(Color::BLUE); // TODO: material
        let ball = Ball::new(ev.get_level());
        let ball_r = ball.get_r();
        let player_xy = ev.get_position();
        commands.spawn((
            ball,
            RigidBody::Dynamic,
            Collider::circle(ball_r),
            MaterialMesh2dBundle {
                mesh: meshes.add(Circle::new(ball_r)).into(),
                transform: Transform::from_translation(
                     player_xy.extend(Z_BALL)
                ),
                material: ball_material.clone(),
                ..default()
            },
        )).with_children(|b| {
            b.spawn((
                Sensor,
                Repulsive { p: 1.0, t: 10.0 },
                Collider::circle(ball_r + 10.0),
            ));
        });
    }
}

fn owner_set_to_repulsive(
    mut commands: Commands,
    q_repulsive: Query<(Entity, &Parent), Added<Repulsive>>,
){
    for (e, parent) in q_repulsive.iter() {
        commands.entity(parent.get())
            .try_insert(RepulsiveOwner(e));
    }
}

fn make_repulsive(
    mut ev_colls: EventReader<Collision>,
    mut q_balls: Query<(&mut LinearVelocity, &ColliderMassProperties, &RepulsiveOwner), With<Ball>>,
    q_repls: Query<(&Position, &Rotation, &Repulsive), Without<Ball>>,
    time: Res<Time>,
    config: Res<Config>,
) {
    for Collision(contact) in ev_colls.read() {
        let e1 = q_repls.get(contact.entity1);
        let e2 = q_repls.get(contact.entity2);

        if let (Err(_), Err(_)) = (e1, e2) {
            continue;
        } else if let (Ok(_), Ok(_)) = (e1, e2) {
            continue;
        } else {
            let (is_sensor_1,
                 (_repls_pos, repls_rot, Repulsive { p, t }),
                 _repls_entity,
                 ball_entity) = if let Ok(e1) = e1 {
                (true, e1, contact.entity1, contact.entity2)
            } else {
                (false, e2.unwrap(), contact.entity2, contact.entity1)
            };
            if let Ok((mut ball_vel, mass, RepulsiveOwner(_rep_e))) = q_balls.get_mut(ball_entity) {
                if let Some(max) = contact.manifolds.iter()
                    .filter_map(|x| x.contacts.iter()
                         .max_by(|l,r|
                             l.penetration.partial_cmp(&r.penetration)
                                .expect("Failed to `penetration` cmpare")))
                    .max_by(|l, r| l.penetration.partial_cmp(&r.penetration)
                        .expect("Failed to `penetration` cmpare"))
                {
                    let normal = if is_sensor_1 {
                        max.normal1
                    } else {
                        max.normal2
                    };

                    let mut k = max.penetration / t;
                    k = k.powf(config.repls_kp).clamp(0., 1.);
                    if k.is_nan() {
                        k = 0.;
                    }

                    let inv_m = config.repls_mk / mass.mass.0;

                    let magn = inv_m * k * *p;

                    let v = repls_rot.rotate(normal).normalize_or_zero();
                    ball_vel.0 += (magn * time.elapsed_seconds()) * v;
                }
            }
        }
    }
}


#[derive(Resource, Debug)]
#[derive(Reflect)]
struct Config {
    repls_mk: f32,
    repls_kp: f32,
}
impl Default for Config {
    fn default() -> Self {
        Self {
            repls_mk: 10.0,
            repls_kp: 0.9,
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
#[command(name = "rpmk")]
struct RpmkCommand {
    mk: f32,
}
fn command_rpmk(
    mut log: ConsoleCommand<RpmkCommand>,

    mut config: ResMut<Config>,
    ) {
    if let Some(Ok(RpmkCommand { mk })) = log.take() {
        config.repls_mk = mk;
    }
}


#[derive(Parser, ConsoleCommand)]
#[command(name = "rpkp")]
struct RpkpCommand {
    kp: f32,
}
fn command_rpkp(
    mut log: ConsoleCommand<RpkpCommand>,

    mut config: ResMut<Config>,
    ) {
    if let Some(Ok(RpkpCommand { kp })) = log.take() {
        config.repls_kp = kp;
    }
}
