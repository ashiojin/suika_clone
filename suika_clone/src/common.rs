use bevy::{
    audio::Volume,
    input::gamepad::{GamepadConnection, GamepadEvent},
    prelude::*
};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use bevy_pkv::PkvStore;

use crate::game_ron::get_default_game_ron_name_and_asset_path;


pub const CAM_ORDER_PLAYING: isize = 10;
pub const CAM_ORDER_TITLE: isize = 0;



#[derive(States, Default, Hash, Clone, Copy, PartialEq, Eq, Debug)]
pub enum GameState {
    #[default]
    Title,
    Loading,
    InGame,
}

#[derive(Reflect, Debug, Clone)]
#[derive(Deserialize, Serialize)]
pub struct Area {
    pub min_x: f32,
    pub max_x: f32,
    pub min_y: f32,
    pub max_y: f32,
}
impl Area {
    fn new(min_x: f32, max_x: f32, min_y: f32, max_y: f32) -> Self {
        Self { min_x, max_x, min_y, max_y, }
    }
}
const AREA_X_MIN: f32 = -500.0;
const AREA_X_MAX: f32 =  500.0;
const AREA_Y_MIN: f32 = -500.0;
const AREA_Y_MAX: f32 =  500.0 + 9999.0;

#[derive(Resource, Debug, Clone)]
pub struct FixedConfig {
    pub grow_time: f32,
    pub area: Area,
    pub max_velocity: f32,
    pub shake_k: f32, // max move is about 0.4 * shake_k
    pub playing_cam_offset: Vec2,
    pub ball_restitution_coef: f32,
    pub bottle_restitution_coef: f32,
}
impl Default for FixedConfig {
    fn default() -> Self {
        Self {
            grow_time: 0.5,
            area: Area::new(AREA_X_MIN, AREA_X_MAX, AREA_Y_MIN, AREA_Y_MAX,),
            max_velocity: 60. * 32. * (30./(2. + 1.)),
            shake_k: 24. / 0.4,
            playing_cam_offset: Vec2::new(100., 0.),
            ball_restitution_coef: 0.15,
            bottle_restitution_coef: 0.15,
        }
    }
}

#[derive(Resource, Debug, Clone)]
#[derive(Reflect)]
#[derive(Deserialize, Serialize)]
pub struct Config {

    // configurable
    pub bgm_volume: i32, // 0..=100
    pub se_volume: i32, // 0..=100

    pub game_ron_name: String,
    pub game_ron_asset_path: String,

}
impl Default for Config {
    fn default() -> Self {
        let (game_ron_name, asset_path) = get_default_game_ron_name_and_asset_path();
        Self {
            bgm_volume: 50,
            se_volume: 50,

            game_ron_name: game_ron_name.to_string(),
            game_ron_asset_path: asset_path.to_string(),
        }
    }
}
fn volume(v: i32, scale: f32) -> Volume {
    Volume::new(scale * v  as f32 / 100.)
}
const STORE_NAME_CONFIG: &str = "config";
impl Config {
    pub fn get_se_volume(&self, scale:f32) -> Volume {
        volume(self.se_volume, scale)
    }
    pub fn get_bgm_volume(&self, scale:f32) -> Volume {
        volume(self.bgm_volume, scale)
    }
}

pub fn load_config(
    mut config: ResMut<Config>,
    pkv: Res<PkvStore>,
) {
    if let Ok(saved_config) = pkv.get::<Config>(STORE_NAME_CONFIG) {
        *config = saved_config;
    } else {
        let def_config = default();
        *config = def_config;
    }
}
pub fn save_config(
    config: Res<Config>,
    mut pkv: ResMut<PkvStore>,
) {
    pkv.set(STORE_NAME_CONFIG, config.into_inner())
        .expect("Failed to store `config`.");
}


// Z-Order
//   These are layers. each layer can freely use +[0.0, 1.0) Z-Order for any purpose.
pub const Z_BACK: f32 = -20.;
pub const Z_UI: f32 = -10.;
pub const Z_GUIDE: f32 = 00.;
pub const Z_WALL: f32 = 10.;
pub const Z_PLAYER: f32 = 20.;
pub const Z_BALL: f32 = 30.;
pub const Z_POPUP: f32 = 40.;

pub const Z_BALL_D_BY_LEVEL: f32 = 0.01;

#[derive(Resource, Debug)]
pub struct ConnectedGamePad(pub Gamepad);

pub fn detect_gamepad(
    mut commands: Commands,
    connected_gamepad: Option<Res<ConnectedGamePad>>,
    mut ev_gamepad: EventReader<GamepadEvent>,
) {
    for ev in ev_gamepad.read() {
        if let GamepadEvent::Connection(con_ev) = ev {
            match con_ev.connection {
                GamepadConnection::Connected(_) => {
                    if connected_gamepad.is_none() {
                        commands.insert_resource(ConnectedGamePad(con_ev.gamepad));
                    }
                },
                GamepadConnection::Disconnected => {
                    if let Some(ConnectedGamePad(old_id)) = connected_gamepad.as_deref() {

                        if *old_id == con_ev.gamepad {
                            commands.remove_resource::<ConnectedGamePad>();
                        }
                    }
                }
            }
        }
    }
}


pub const KEYBOARD_KEYS_LEFT: [KeyCode; 2] = [KeyCode::ArrowLeft, KeyCode::KeyA];
pub const KEYBOARD_KEYS_RIGHT: [KeyCode; 2] = [KeyCode::ArrowRight, KeyCode::KeyD];
pub const KEYBOARD_KEYS_MAIN: [KeyCode; 2] = [KeyCode::Space, KeyCode::KeyZ];
pub const KEYBOARD_KEYS_SUB1: [KeyCode; 3] = [KeyCode::ArrowUp, KeyCode::KeyW, KeyCode::KeyX];
pub const KEYBOARD_KEYS_SUB2: [KeyCode; 2] = [KeyCode::KeyU, KeyCode::KeyC];
pub const KEYBOARD_KEYS_START: [KeyCode; 1] = [KeyCode::KeyP];
pub const KEYBOARD_KEYS_SELECT: [KeyCode; 1] = [KeyCode::Escape];

pub const GAMEPAD_BTNS_LEFT: [GamepadButtonType; 1] = [GamepadButtonType::DPadLeft];
pub const GAMEPAD_BTNS_RIGHT: [GamepadButtonType; 1] = [GamepadButtonType::DPadRight];
pub const GAMEPAD_BTNS_MAIN: [GamepadButtonType; 1] = [GamepadButtonType::East];
pub const GAMEPAD_BTNS_SUB1: [GamepadButtonType; 1] = [GamepadButtonType::North];
pub const GAMEPAD_BTNS_SUB2: [GamepadButtonType; 2] = [GamepadButtonType::RightTrigger, GamepadButtonType::LeftTrigger];
pub const GAMEPAD_BTNS_START: [GamepadButtonType; 1] = [GamepadButtonType::Start];
pub const GAMEPAD_BTNS_SELECT: [GamepadButtonType; 1] = [GamepadButtonType::Select];

pub fn to_gamepad_btn(gamepad: Gamepad, btn_types: &[GamepadButtonType]) -> Vec<GamepadButton> {
    btn_types.iter().map(|btn|
        GamepadButton::new(gamepad, *btn)
    ).collect_vec()
}


/// MaruMinyaMフォントの特殊文字（ゲームパッド系）
#[allow(dead_code)]
pub mod maru_minya_m {
    pub const GP_LSTICK: &str = "\u{E014}";
    pub const GP_DP_LEFT_RIGHT: &str = "\u{E006}";
    pub const GP_BTN_S: &str = "\u{E010}";
    pub const GP_BTN_E: &str = "\u{E011}";
    pub const GP_BTN_W: &str = "\u{E012}";
    pub const GP_BTN_N: &str = "\u{E013}";


    // xb
    pub const GP_BTN_LB: &str = "\u{E024}";
    pub const GP_BTN_RB: &str = "\u{E025}";
    pub const GP_BTN_START: &str = "\u{E02D}";
    pub const GP_BTN_SELECT: &str = "\u{E02C}";
}


#[allow(dead_code)]
pub enum GpKbInput {
    MoveLeftRight,
    Main,
    Sub1,
    Sub2,
    Start,
    Select,
}

impl GpKbInput {
    pub fn get_str(&self) -> String {
        use maru_minya_m::*;
        match *self {
            GpKbInput::MoveLeftRight => format!("{}/{}/{}/{}", GP_LSTICK, GP_DP_LEFT_RIGHT, "\u{21E6}\u{21E8}", "AD"),
            GpKbInput::Main => format!("{}/{}/{}", GP_BTN_E, "Space", "Z"),
            GpKbInput::Sub1 => format!("{}/{}/{}/{}", GP_BTN_N, "\u{21E7}", "W", "X"),
            GpKbInput::Sub2 => format!("{}/{}/{}/{}", GP_BTN_LB, GP_BTN_RB, "U", "C"),
            GpKbInput::Start => format!("{}/{}", GP_BTN_START, "P"),
            GpKbInput::Select => format!("{}/{}", GP_BTN_SELECT, "Esc"),
        }
    }
}
