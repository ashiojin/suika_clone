use crate::prelude::*;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::embedded_assets::assets::DEFAULT_GAME_RON_PATH;

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
pub struct PlayerRon {
    pub view_width: f32,
    pub view_height: f32,
    pub offset_x: f32,
    pub offset_y: f32,
    pub image_asset_path: String,

    pub guide_color: Color,
    pub speed: f32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[derive(Reflect)]
pub struct SoundRon {
    pub bgm_asset_path: String,
    pub se_combine_asset_path: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[derive(Reflect)]
#[derive(Asset)]
pub struct GameRon {
    pub balls: Vec<BallLevelSettingRon>,
    pub drop_ball_level_max: usize,
    pub player: PlayerRon,
    pub sounds: SoundRon,
}


#[derive(Resource, Debug, Clone)]
#[derive(Reflect)]
pub struct CurrentGameRon(pub Handle<GameRon>);

impl Loadable for CurrentGameRon {
    fn get_untyped_handles(&self) -> Vec<UntypedHandle> {
        vec![self.0.clone().untyped()]
    }
}

const DEFAULT_GAME_RON_NAME: &str = "(default)";
pub fn get_default_game_ron_name_and_asset_path() -> (&'static str, &'static str) {
    (DEFAULT_GAME_RON_NAME, DEFAULT_GAME_RON_PATH)
}


