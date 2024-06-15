use crate::prelude::*;
use bevy::prelude::*;
use game_ron::GameRon;
use crate::embedded_assets::assets::DEFAULT_GAME_RON_PATH;

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
