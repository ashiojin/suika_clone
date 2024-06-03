use bevy::{
    prelude::*, render::camera::ScalingMode, window::WindowResolution
};
use bevy_egui::egui;
use bevy_egui::EguiContexts;
use bevy_egui::EguiPlugin;
use bevy_xpbd_2d::prelude::*;

use bevy_rand::prelude::*;
use bevy_prng::ChaCha8Rng;

mod debug;
mod common;
mod asset;
mod title_screen;
mod game_screen;
mod physics_custom;

mod prelude {
    pub use crate::debug::*;
    pub use crate::common::*;
    pub use crate::asset::*;
    pub use crate::physics_custom::*;
    pub use crate::game_screen::*;
}
use crate::prelude::*;


use crate::title_screen::*;


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
// - [x] Embedded an external file (.ron or others) as settings
//   for ball size, texture infos.
// - [x] Sound.
//   - [x] BGM.
//   - [x] SE.
// - [x] Title Screen.
//   - Use embedded assets(title image)
// - [x] Config Screen. (or Popup on title screen)
//   - [x] List and Load a .ron file
// - [ ] Separate Asset Loading Logic into Plugin or Use bevy_asset_loader
// - [ ] Refine config screen & title screen.
//   - [ ] Loading state needed to read 'list.ron' and selected game ron
//   - [ ] Use bevy_egui_kbgp
// - [x] Change timing of spawing fake ball to after previous ball is touched to other
// - [ ] Create PlayerBundle.
// - [ ] Player texture.
// - [ ] Player Actions.
//   - [ ] Holding a ball.
//   - [ ] Shaking the bottle.
// - [ ] Extend .ron
//   - [ ] player settings
//   - [ ] bottle settings
//   - [ ] background image
//   - [ ] popup/messages
// - [ ] Pause in playing game
//   - [ ] Return to Title
//   - [ ] Restart
// - [ ] Save config.
// - [ ] Set config from program args.
// - [ ] New game mode: ex) Mode where the objective is to flood a lot of balls.
// - [ ] Separate game states to 
//       application state (pre-load/title/config/loading/in-game) and
//       in-game state (playing/pausing/gameover)
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
        PhysicsPlugins::default()
            .build()
            .add(LimitVelocityPlugin),

        EguiPlugin,
        ScDebugPlugin::new(true, true),
    ));

    app.insert_resource(Config::default());
    app.init_state::<GameState>();
    app.add_systems(Startup, (
        setup_egui,
        setup_camera,
    ));

    app.add_plugins((
        ScTitleScreenPlugin,
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

fn setup_egui(
    mut context: EguiContexts,
) {
    // see https://qiita.com/8bitTD/items/c3f1a9421615c3db1879
    let mut txt_font = egui::FontDefinitions::default();
    txt_font.families.get_mut(&egui::FontFamily::Proportional)
        .unwrap()
        .insert(0, "GL-CurulMinamo".to_owned());


    let fd = egui::FontData::from_static(include_bytes!("../assets/fonts/GL-CurulMinamoto.ttf"));

    txt_font.font_data.insert("GL-CurulMinamo".to_owned(), fd);

    context.ctx_mut().set_fonts(txt_font);
}
