use crate::prelude::*;

use bevy::prelude::*;
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
            app.add_console_command::<DispAreaCommand, _>(command_disp_area);

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

    mut config: ResMut<Config>,
) {
    if let Some(Ok(GrowCommand { tm })) = log.take() {
        config.grow_time = tm;
        reply!(log, "{:?}", *config);
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
        reply!(log, "{:?}", *config);
    }
}


fn run_condition_for_display_area(
    debug_config: Res<DebugConfig>,
) -> bool {
    debug_config.display_area
}

fn display_area(
    mut gizmos: Gizmos<DefaultGizmoConfigGroup>,
    config: Res<Config>,
) {
    let Area { min_x, max_x, min_y, max_y } = config.area;

    gizmos.rect_2d(
        Vec2::new((max_x+min_x)/2., (max_y+min_y)/2.),
        0.,
        Vec2::new(max_x-min_x, max_y-min_y),
        Color::RED);
}
