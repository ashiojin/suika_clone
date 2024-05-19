use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_xpbd_2d::prelude::*;

fn main() {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins,
        PhysicsPlugins::default(),
        PhysicsDebugPlugin::default(),
    ));

    app.add_systems(Startup, (
        setup_camera,
        setup_wall,

        create_test_balls,
    ));

    app.run();
}

#[derive(Component, Debug)]
struct WALL;

#[derive(Component, Debug)]
struct BALL;

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
const WALL_HEIGHT: f32 = 600.0;
const WALL_THICKNESS: f32 = 6.0;

const Z_BACK: f32 = -1.;
const Z_BALL: f32 = 0.;
const Z_WALL: f32 = 1.;


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
    let bottom_wh = Vec2::new(WALL_WIDTH + 2.*WALL_THICKNESS, WALL_THICKNESS);
    let wall_wh = Vec2::new(WALL_THICKNESS, WALL_HEIGHT);

    let outer_l_t = Vec2::new(
        -1. * bottom_wh.x * 0.5,
        -1. * -wall_wh.y * 0.5,
    );
    let bottom_l_t = Vec2::new(0., -WALL_HEIGHT) + outer_l_t;
    let left_wall_l_t = outer_l_t;
    let right_wall_l_t = Vec2::new(WALL_WIDTH + WALL_THICKNESS, 0.) + outer_l_t;

    fn inv_y(v: Vec2) -> Vec2 { Vec2::new(v.x, -v.y) }
    let bottom_c = bottom_l_t + 0.5 * inv_y(bottom_wh);
    let left_wall_c = left_wall_l_t + 0.5 * inv_y(wall_wh);
    let right_wall_c = right_wall_l_t + 0.5 * inv_y(wall_wh);


    let wall_color = Color::RED;
    let wall_material = materials.add(wall_color);

    // Bottom
    commands.spawn((
        WALL,
        MaterialMesh2dBundle {
            mesh: meshes.add(Rectangle::from_size(bottom_wh)).into(),
            transform: Transform::from_translation(bottom_c.extend(Z_WALL)),
            material: wall_material.clone(),
            ..default()
        },
    ));

    // Left wall
    commands.spawn((
        WALL,
        MaterialMesh2dBundle {
            mesh: meshes.add(Rectangle::from_size(wall_wh)).into(),
            transform: Transform::from_translation(left_wall_c.extend(Z_WALL)),
            material: wall_material.clone(),
            ..default()
        },
    ));

    // Right wall
    commands.spawn((
        WALL,
        MaterialMesh2dBundle {
            mesh: meshes.add(Rectangle::from_size(wall_wh)).into(),
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

fn create_test_balls(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let xs = [-100.0_f32, 100., 101.];
    let ys = [   0.0_f32,   0., 100.];
    let r = 40.;

    let ball_material = materials.add(Color::BLUE);

    for (&x, &y) in xs.iter().zip(ys.iter()) {
        commands.spawn((
            BALL,
            MaterialMesh2dBundle {
                mesh: meshes.add(Circle::new(r)).into(),
                transform: Transform::from_translation(
                    (Vec2::new(x, y))
                        .extend(Z_BALL)
                ),
                material: ball_material.clone(),
                ..default()
            },
        ));
    }
}



