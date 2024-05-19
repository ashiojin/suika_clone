use bevy::{prelude::*, sprite::{MaterialMesh2dBundle, Mesh2dHandle}};
use bevy_xpbd_2d::prelude::*;

const GRAVITY_SCALE: f32 = 9.81 * 100.;

fn main() {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins,
        PhysicsPlugins::default(),
        PhysicsDebugPlugin::default(),
    ));

    app.insert_resource(Gravity(Vec2::NEG_Y * GRAVITY_SCALE));

    app.add_systems(Startup, (
        setup_camera,
        setup_wall,

        spawn_player,
    ));

    app.add_systems(Update, (
        report_collisions_of_balls,
    ));

    app.add_systems(FixedUpdate, (
        move_player,
        spawn_ball,
    ));

    app.run();
}


#[derive(Component, Debug)]
struct Player {
    speed: f32,
    next_ball_level: BallLevel,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            speed: 1.5,
            next_ball_level: default(),
        }
    }
}
impl Player {
    fn change_next_ball_level(&mut self, /* randam generator here? */) {
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
const BALL_LEVEL_MAX:usize = 9;
const R_FOR_BALL_LEVEL: [f32; BALL_LEVEL_MAX-BALL_LEVEL_MIN]
    = [
        020. * 0.5,
        040. * 0.5,
        080. * 0.5,
        120. * 0.5,
        200. * 0.5,
        240. * 0.5,
        300. * 0.5,
        360. * 0.5,
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
const WALL_THICKNESS: f32 = 6.0;

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

fn report_collisions_of_balls(
    mut ev_colls: EventReader<Collision>,
    q_balls: Query<(Entity, &Ball)>,
) {
    for Collision(contacts) in ev_colls.read() {
        let b1 = q_balls.get(contacts.entity1);
        let b2 = q_balls.get(contacts.entity2);
        if let (Ok(b1), Ok(b2)) = (b1, b2) {
            if b1.1.level == b2.1.level {
                info!(
                    "{:?} and {:?} are colliding",
                    b1,
                    b2,
                );
            }
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

fn move_player(
    mut q_player: Query<(&mut Transform, &Player)>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if let Ok((mut trans, player)) = q_player.get_single_mut() {
        let mut lr = 0.;
        if keyboard.pressed(KeyCode::ArrowLeft) {
            lr += -player.speed;
        }
        if keyboard.pressed(KeyCode::ArrowRight) {
            lr += player.speed;
        }

        // TODO: Check x renge

        trans.translation.x += lr;
    }
}

fn spawn_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,

    mut q_player: Query<(&Transform, &mut Player)>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        if let Ok((trans, mut player)) = q_player.get_single_mut() {
            let ball_material = materials.add(Color::BLUE);
            let ball = Ball::new(player.next_ball_level);
            let ball_r = ball.get_r();
            let player_xy = trans.translation.xy();
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
            ));

            player.change_next_ball_level();
        }
    }
}


