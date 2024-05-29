use bevy::{
    prelude::*, render::camera::ScalingMode, window::WindowResolution
};
use bevy_xpbd_2d::prelude::*;

use bevy_rand::prelude::*;
use bevy_prng::ChaCha8Rng;

mod debug;
mod common;
mod asset;
mod game_screen;

mod prelude {
    pub use crate::debug::*;
    pub use crate::common::*;
    pub use crate::asset::*;
    pub use crate::game_screen::*;
}
use crate::prelude::*;

//
// ToDo Items
// - (ALWAYS) Refactoring!
// - [x] Remove Max Level Balls Combined.
// - [x] Scoring:
//   - [x] Combine Scores.
//   - [x] Drop Scores.
// - [x] Player position:
//   - [x] y-position should be higher than all of balls.
//   - [x] x-position should be limited x positon to the inside of the bottle.
// - [x] Use random generator.
// - [x] GameOver.
// - [x] Reset game.
// - [ ] Create and Load an external file (.ron or others)
//   for ball size, texture, and other data.
// - [ ] Sound.
//   - [ ] BGM.
//   - [ ] SE.
// - [ ] Title Screen.
// - [ ] Create PlayerBundle
// - [ ] Player texture.
// - [ ] Config Screen.
// - [ ] Player Actions.
//   - [ ] Holding a ball.
//   - [ ] Shaking the bottle.
// - [ ] New game mode: ex) Mode where the objective is to flood a lot of balls.
//


// Window Settings
const TITLE: &str = "Suikx clone";
const LOGICAL_WIDTH: f32 = 1440.;
const LOGICAL_HEIGHT: f32 = 1080. - 120./* for browser ui spaces*/;
const WINDOW_MIN_WIDTH: f32 = LOGICAL_WIDTH / 2.;
const WINDOW_MIN_HEIGHT: f32 = LOGICAL_HEIGHT / 2.;
const WINDOW_MAX_WIDTH: f32 = 1920.;
const WINDOW_MAX_HEIGHT: f32 = 1080.;



fn main() {
    #[cfg(target_family = "windows")]
    std::env::set_var("RUST_BACKTRACE", "1"); // Can't read env values when running on WSL

    let mut app = App::new();

    app.add_plugins((
        EntropyPlugin::<ChaCha8Rng>::default(),
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

        ScDebugPlugin::new(true, true),
    ));

    app.init_state::<GameState>();
    app.add_systems(Startup, (
        setup_camera,
    ));

    app.add_plugins((
        ScLoadingScreenPlugin,
        ScGameScreenPlugin,
    ));

    app.run();
}

fn setup_camera(
    mut commands: Commands,
) {
    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.projection.scaling_mode = ScalingMode::FixedVertical(1040.);
    commands.spawn((
        camera_bundle,
    ));
}

