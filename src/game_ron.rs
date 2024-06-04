use crate::prelude::*;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};


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
#[derive(Asset)]
pub struct GameRon {
    pub balls: Vec<BallLevelSettingRon>,
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
const DEFAULT_GAME_RON_FILE_NAME: &str = "kao.game.ron";
pub fn get_default_game_ron_name_and_file_name() -> (&'static str, &'static str) {
    (DEFAULT_GAME_RON_NAME, DEFAULT_GAME_RON_FILE_NAME)
}
