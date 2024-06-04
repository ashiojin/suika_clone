use bevy::prelude::*;

use crate::game_ron::gat_default_game_ron_name_and_file_name;

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


#[derive(Resource, Debug, Clone)]
#[derive(Reflect)]
pub struct Config {
    pub grow_time: f32,
    pub area: Area,
    pub max_velocity: f32,

    pub bgm_volume: i32, // 0..=100

    pub game_ron_name: String,
    pub game_ron_file_name: String,

}
impl Default for Config {
    fn default() -> Self {
        let (game_ron_name, game_ron_file_name) = gat_default_game_ron_name_and_file_name();
        Self {
            grow_time: 0.2,
            area: Area::new(AREA_X_MIN, AREA_X_MAX, AREA_Y_MIN, AREA_Y_MAX,),
            max_velocity: 60. * 32. * (30./2.),

            bgm_volume: 50,

            game_ron_name: game_ron_name.to_string(),
            game_ron_file_name: game_ron_file_name.to_string(),
        }
    }
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
