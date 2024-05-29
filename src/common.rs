use bevy::prelude::*;

#[derive(States, Default, Hash, Clone, Copy, PartialEq, Eq, Debug)]
pub enum GameState {
    #[default]
    Loading,
    InGame,
    GameOver,
}

#[derive(Reflect, Debug)]
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

#[derive(Debug, Clone)]
#[derive(Reflect)]
pub struct BallLevelSettingRon {
    pub physics_radius: f32,

    pub view_width: f32,
    pub view_height: f32,

    pub image_asset_path: String,
}

impl BallLevelSettingRon {
    fn new(radius: f32, idx: usize) -> Self {
        let tex_k = 512. / 420.;
        Self {
            physics_radius: radius,
            view_width: radius * tex_k * 2.,
            view_height: radius * tex_k * 2.,
            image_asset_path: format!("images/kao/kao_{:>02}.png", idx),
        }
    }
}

#[derive(Resource, Debug)]
#[derive(Reflect)]
pub struct Config {
    pub grow_time: f32,
    pub area: Area,

    pub game_ron: Option<Vec<BallLevelSettingRon>>,

}
impl Default for Config {
    fn default() -> Self {
        let from_ron = vec![ // TODO: read from ron
            BallLevelSettingRon::new(028.0 , 1),
            BallLevelSettingRon::new(034.5 , 2),
            BallLevelSettingRon::new(043.5 , 3),
            BallLevelSettingRon::new(055.0 , 4),
            BallLevelSettingRon::new(069.0 , 5),
            BallLevelSettingRon::new(086.0 , 6),
            BallLevelSettingRon::new(105.0 , 7),
            BallLevelSettingRon::new(127.0 , 8),
            BallLevelSettingRon::new(151.0 , 9),
            BallLevelSettingRon::new(177.5 , 10),
            BallLevelSettingRon::new(207.0 , 11),
        ];
        Self {
            grow_time: 0.2,
            area: Area::new(AREA_X_MIN, AREA_X_MAX, AREA_Y_MIN, AREA_Y_MAX,),
            game_ron: Some(from_ron),
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
