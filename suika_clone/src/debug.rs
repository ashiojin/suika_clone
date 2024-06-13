#![cfg_attr(not(debug_assertions), allow(dead_code))] // only for debugging

use crate::prelude::*;

use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;
use bevy_console::{reply, AddConsoleCommand, ConsoleCommand, ConsoleConfiguration, ConsolePlugin};
use clap::Parser;

use bevy_xpbd_2d::prelude::PhysicsDebugPlugin;

pub struct ScDebugPlugin {
    console: bool,
    physics: bool,
}

impl ScDebugPlugin {
    pub fn new(console: bool, physics: bool) -> Self {
        Self { console, physics, }
    }
}

#[derive(Resource, Debug)]
struct DebugConfig {
    display_area: bool,
}
#[allow(clippy::derivable_impls)]
impl Default for DebugConfig {
    fn default() -> Self {
        Self {
            display_area: false,
        }
    }
}


impl Plugin for ScDebugPlugin {
    fn build(&self, app: &mut App) {
        if self.physics {
            app.add_plugins((
                PhysicsDebugPlugin::default(),
            ));
        }
        if self.console {
            app.add_plugins((
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
            app.add_console_command::<MaxVelCommand, _>(command_max_vel);
            app.add_console_command::<DispAreaCommand, _>(command_disp_area);
            app.add_console_command::<RestitutionCommand, _>(command_restitution);
            app.add_console_command::<FrictionDynamicCommand, _>(command_friction_dynamic);
            app.add_console_command::<FrictionStaticCommand, _>(command_friction_static);

            app.insert_resource(DebugConfig {
                display_area: true,
            });
            app.add_systems(Update, (
                display_area,
            ).run_if(in_state(GameState::InGame))
                .run_if(run_condition_for_display_area));

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

    mut config: ResMut<FixedConfig>,
) {
    if let Some(Ok(GrowCommand { tm })) = log.take() {
        config.grow_time = tm;
        reply!(log, "{:?}", config.grow_time);
    }
}


#[derive(Parser, ConsoleCommand)]
#[command(name = "max_vel")]
struct MaxVelCommand {
    max: f32,
}
fn command_max_vel(
    mut log: ConsoleCommand<MaxVelCommand>,

    mut config: ResMut<FixedConfig>,
) {
    if let Some(Ok(MaxVelCommand { max })) = log.take() {
        config.max_velocity = max;
        reply!(log, "{:?}", config.max_velocity);
    }
}

#[derive(Parser, ConsoleCommand, Default)]
#[command(name = "disp_area")]
struct DispAreaCommand {
    display: Option<bool>, // FIXME: the type should be `bool`. if doing so, an assertion will be caused...
}
fn command_disp_area(
    mut log: ConsoleCommand<DispAreaCommand>,

    mut config: ResMut<DebugConfig>,
) {
    if let Some(Ok(DispAreaCommand { display })) = log.take() {
        config.display_area = display.unwrap_or(false);
        reply!(log, "{:?}", config.display_area);
    }
}


fn run_condition_for_display_area(
    debug_config: Res<DebugConfig>,
) -> bool {
    debug_config.display_area
}

fn display_area(
    mut gizmos: Gizmos<DefaultGizmoConfigGroup>,
    config: Res<FixedConfig>,
) {
    let Area { min_x, max_x, min_y, max_y } = config.area;

    gizmos.rect_2d(
        Vec2::new((max_x+min_x)/2., (max_y+min_y)/2.),
        0.,
        Vec2::new(max_x-min_x, max_y-min_y),
        Color::RED);
}

#[derive(Parser, ConsoleCommand, Default)]
#[command(name = "restitution")]
struct RestitutionCommand {
    coefficient: f32,
}
fn command_restitution(
    mut log: ConsoleCommand<RestitutionCommand>,
    mut q_col: Query<&mut Restitution, With<Collider>>,
    mut asset: Option<ResMut<GameAssets>>,
) {
    if let Some(Ok(RestitutionCommand { coefficient })) = log.take() {
        if let Some(assets) = asset.as_deref_mut() {
            assets.ball_physics.restitution.coef = coefficient;
            assets.bottle_physics.restitution.coef = coefficient;
            for mut rest in q_col.iter_mut() {
                rest.coefficient = coefficient;
            }
            reply!(log, "set {} to {} colliders", coefficient, q_col.iter().len());
        } else {
            reply!(log, "ERROR: This command is available only during playing a game");
        }
    }
}

#[derive(Parser, ConsoleCommand, Default)]
#[command(name = "friction_dynamic")]
struct FrictionDynamicCommand {
    coefficient: f32,
}
fn command_friction_dynamic(
    mut log: ConsoleCommand<FrictionDynamicCommand>,
    mut q_col: Query<&mut Friction, With<Collider>>,
    mut asset: Option<ResMut<GameAssets>>,
) {
    if let Some(Ok(FrictionDynamicCommand { coefficient })) = log.take() {
        if let Some(assets) = asset.as_deref_mut() {
            assets.ball_physics.friction.dynamic_coef = coefficient;
            assets.bottle_physics.friction.dynamic_coef = coefficient;
            for mut rest in q_col.iter_mut() {
                rest.dynamic_coefficient = coefficient;
            }
            reply!(log, "set {} to {} colliders", coefficient, q_col.iter().len());
        } else {
            reply!(log, "ERROR: This command is available only during playing a game");
        }
    }
}

#[derive(Parser, ConsoleCommand, Default)]
#[command(name = "friction_static")]
struct FrictionStaticCommand {
    coefficient: f32,
}
fn command_friction_static(
    mut log: ConsoleCommand<FrictionDynamicCommand>,
    mut q_col: Query<&mut Friction, With<Collider>>,
    mut asset: Option<ResMut<GameAssets>>,
) {
    if let Some(Ok(FrictionDynamicCommand { coefficient })) = log.take() {
        if let Some(assets) = asset.as_deref_mut() {
            assets.ball_physics.friction.static_coef = coefficient;
            assets.bottle_physics.friction.static_coef = coefficient;
            for mut rest in q_col.iter_mut() {
                rest.static_coefficient = coefficient;
            }
            reply!(log, "set {} to {} colliders", coefficient, q_col.iter().len());
        } else {
            reply!(log, "ERROR: This command is available only during playing a game");
        }
    }
}
