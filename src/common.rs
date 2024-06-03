use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(States, Default, Hash, Clone, Copy, PartialEq, Eq, Debug)]
pub enum GameState {
    #[default]
    Title,
    Loading,
    InGame,
    GameOver,
}

#[derive(Reflect, Debug, Clone)]
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
const AREA_Y_MAX: f32 =  500.0 + 200.0;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[derive(Reflect)]
pub struct BallLevelSettingRon {
    pub physics_radius: f32,

    pub view_width: f32,
    pub view_height: f32,

    pub image_asset_path: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[derive(Reflect)]
pub struct SoundRon {
    pub bgm_asset_path: String,
    pub se_combine_asset_path: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[derive(Reflect)]
pub struct GameRon {
    pub balls: Vec<BallLevelSettingRon>,
    pub sounds: SoundRon,
}

const DEFAULT_GAME_RON: &str = include_str!("../assets/ron/kao.ron");
const DEFAULT_GAME_RON_NAME: &str = "(default)";

#[derive(Resource, Debug, Clone)]
#[derive(Reflect)]
pub struct Config {
    pub grow_time: f32,
    pub area: Area,
    pub max_velocity: f32,

    pub bgm_volume: i32, // 0..=100

    pub game_ron: GameRon,
    pub game_ron_name: String,

}
impl Default for Config {
    fn default() -> Self {
        let game_ron: GameRon = ron::from_str(DEFAULT_GAME_RON)
            .expect("Failed to deserialize DEFAULT_GAME_RON");

        //info!("----------------------------------------");
        //info!("{}", ron::ser::to_string_pretty(&game_ron, ron::ser::PrettyConfig::default()).unwrap());
        //info!("----------------------------------------");
        Self {
            grow_time: 0.2,
            area: Area::new(AREA_X_MIN, AREA_X_MAX, AREA_Y_MIN, AREA_Y_MAX,),
            max_velocity: 60. * 32. * (30./2.),

            bgm_volume: 50,

            game_ron,
            game_ron_name: "(default)".to_string(),
        }
    }
}

pub fn read_default_game_ron() -> (GameRon, &'static str) {
    let game_ron: GameRon = ron::from_str(DEFAULT_GAME_RON)
        .expect("Failed to deserialize DEFAULT_GAME_RON");
    (game_ron, DEFAULT_GAME_RON_NAME)
}

// Z-Order
//   These are layers. each layer can freely use +[0.0, 1.0) Z-Order for any purpose.
pub const Z_BACK: f32 = -20.;
pub const Z_SCORE: f32 = -10.;
pub const Z_WALL: f32 = 00.;
pub const Z_PLAYER: f32 = 10.;
pub const Z_BALL: f32 = 20.;
pub const Z_POPUP_GAMEOVER: f32 = 30.;

pub const Z_BALL_D_BY_LEVEL: f32 = 0.01;
