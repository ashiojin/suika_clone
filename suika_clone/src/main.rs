use bevy::{
    prelude::*, render::camera::ScalingMode, window::WindowResolution
};
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy_egui_kbgp::KbgpNavBindings;
use bevy_egui_kbgp::KbgpPlugin;
use bevy_egui_kbgp::KbgpSettings;
use bevy_pkv::PkvStore;
use bevy_xpbd_2d::prelude::*;

use bevy_rand::prelude::*;
use bevy_prng::ChaCha8Rng;

mod debug;
mod common;
#[cfg(target_arch = "wasm32")]
mod wasm;
mod embedded_assets;
mod game_assets;
mod game_ron;
mod resource_loader;
mod audios;
mod loading_screen;
mod title_screen;
mod game_screen;
mod physics_custom;

mod prelude {
    pub use crate::common::*;
    pub use crate::game_assets::*;
    pub use crate::resource_loader::*;
    pub use crate::audios::*;
    pub use crate::loading_screen::*;
    pub use crate::physics_custom::*;
    pub use crate::game_screen::*;

}
use crate::prelude::*;


use crate::title_screen::*;
use crate::embedded_assets::assets::ScEmbeddedAssetsPlugin;




// Window Settings
const TITLE: &str = "Suikx clone";


fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    run_app(None);
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn start(arg: &str) {
    run_app(Some(arg));
}

fn run_app(arg: Option<&str>) {
    #[cfg(target_family = "windows")]
    std::env::set_var("RUST_BACKTRACE", "1"); // Can't read env values when running on WSL

    let mut app = App::new();

    app.add_plugins((
        EntropyPlugin::<ChaCha8Rng>::default(),

        #[cfg(target_arch = "wasm32")]
        wasm::HttpWithVersionQueryStringWasmAssetReaderPlugin::new(
            option_env!("ASSETS_DIR_HASH").unwrap_or("---")), // before DefaultPlugins

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
        KbgpPlugin,

        #[cfg(debug_assertions)]
        debug::ScDebugPlugin::new(true, true),

        ScEmbeddedAssetsPlugin,

    ));

    app.insert_resource(KbgpSettings {
        bindings: KbgpNavBindings::empty()
            .with_wasd_navigation()
            .with_arrow_keys_navigation()
            .with_gamepad_dpad_navigation_and_south_button_activation(),
        disable_default_navigation: true,
        disable_default_activation: true,
        prevent_loss_of_focus: true,
        focus_on_mouse_movement: true,
        allow_keyboard: true,
        allow_gamepads: true,
        allow_mouse_wheel: true,
        allow_mouse_buttons: true,
        allow_mouse_wheel_sideways: true,
    });

    app.insert_resource(PkvStore::new("ashiojin.com", "suika_clone"));
    app.insert_resource(FixedConfig::default());
    app.insert_resource(Config::default());

    app.insert_resource(AppArgs {
        force_ron_file: arg.map(|x| x.to_string()),
    });

    app.init_state::<GameState>();
    app.add_systems(Startup, (
        setup_egui,
        setup_camera,
        load_config,
    ));

    app.add_systems(Update, (
        detect_gamepad,
        force_single_bgm,
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
    camera_bundle.projection.scaling_mode = ScalingMode::FixedVertical(LOGICAL_HEIGHT);
    camera_bundle.camera.order = CAM_ORDER_TITLE;
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
        .insert(0, "MaruMinyaM".to_owned());


    let fd = egui::FontData::from_static(include_bytes!("./embedded_assets/fonts/x12y12pxMaruMinyaM.ttf"));

    txt_font.font_data.insert("MaruMinyaM".to_owned(), fd);

    context.ctx_mut().set_fonts(txt_font);
}
