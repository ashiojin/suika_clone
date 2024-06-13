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
pub struct BottleRon {
    pub fg_image_asset_path: String,
    pub bg_image_asset_path: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[derive(Reflect)]
pub struct BackgroundRon {
    pub bg_image_asset_path: String,
    pub offset: Vec2,
}


#[derive(Debug, Clone, Deserialize, Serialize)]
#[derive(Reflect)]
pub struct ScoreViewRon {
    pub bg_image_asset_path: String,
    pub border_width: f32,
    pub font_color: Color,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
#[derive(Reflect)]
pub struct HoldViewRon {
    pub bg_image_asset_path: String,
    pub border_width: f32,
    pub font_color: Color,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
#[derive(Reflect)]
pub struct ManualViewRon {
    pub bg_image_asset_path: String,
    pub border_width: f32,
    pub font_color: Color,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
#[derive(Reflect)]
pub struct PopupViewRon {
    pub bg_image_asset_path: String,
    pub border_width: f32,
    pub font_color: Color,
    pub font_color_sub: Color,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
#[derive(Reflect)]
pub struct UiRon {
    pub hold_view: HoldViewRon,
    pub score_view: ScoreViewRon,
    pub manual_view: ManualViewRon,
    pub popup: PopupViewRon,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[derive(Reflect)]
pub struct SoundRon {
    pub bgm_asset_path: String,
    pub bgm_scale: f32,
    pub se_combine_asset_path: String,
    pub se_combine_scale: f32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[derive(Reflect)]
pub struct FrictionRon {
    pub dynamic_coef: f32,
    pub static_coef: f32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[derive(Reflect)]
pub struct RestitutionRon {
    pub coef: f32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[derive(Reflect)]
pub struct PhysicsRon {
    pub friction: FrictionRon,
    pub restitution: RestitutionRon,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[derive(Reflect)]
#[derive(Asset)]
pub struct GameRon {
    pub balls: Vec<BallLevelSettingRon>,
    pub drop_ball_level_max: usize,
    pub player: PlayerRon,
    pub bottle: BottleRon,
    pub background: BackgroundRon,
    pub sounds: SoundRon,
    pub ui: UiRon,
    pub ball_physics: PhysicsRon,
    pub bottle_physics: PhysicsRon,
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


