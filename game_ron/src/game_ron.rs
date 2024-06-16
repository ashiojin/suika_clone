use bevy::prelude::*;
use serde::{Deserialize, Serialize};


pub mod effects {
    use bevy::prelude::*;
    use serde::{Deserialize, Serialize};


    #[derive(Debug, Clone, Deserialize, Serialize)]
    #[derive(Reflect)]
    pub struct RandRange<T: Clone>(pub T, pub T);

    #[derive(Debug, Clone, Deserialize, Serialize)]
    #[derive(Reflect)]
    pub struct Linear<T: Clone>(pub Vec<T>);
    impl Default for Linear<f32> {
        fn default() -> Self {
            Self(vec![1.])
        }
    }

    #[derive(Debug, Clone, Deserialize, Serialize)]
    #[derive(Reflect)]
    pub struct Scattering {
        pub image_asset_paths: Vec<String>,
        pub image_scale: f32,

        /// list of alpha [0.0, 1.0]. at least 1 element.
        #[serde(default)]
        pub alpha: Linear<f32>,
        /// list of red [0.0, 1.0]. at least 1 element.
        #[serde(default)]
        pub red: Linear<f32>,
        /// list of green [0.0, 1.0]. at least 1 element.
        #[serde(default)]
        pub green: Linear<f32>,
        /// list of blue [0.0, 1.0]. at least 1 element.
        #[serde(default)]
        pub blue: Linear<f32>,

        /// degree
        pub theta: RandRange<f32>,
        /// px/sec
        pub velocity: RandRange<f32>,
        /// rotation times/sec
        pub rotation: RandRange<f32>,
        /// px/sec * 2
        pub accelation: RandRange<Vec2>,
        ///
        pub num: RandRange<usize>,
        /// sec
        pub time: RandRange<f32>,
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[derive(Reflect)]
pub enum EffectRon {
    Scattering(effects::Scattering),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[derive(Reflect)]
pub struct BallLevelSettingRon {
    pub physics_radius: f32,

    pub view_width: f32,
    pub view_height: f32,

    pub image_asset_path: String,

    /// Index of `effects`.
    #[serde(default)]
    pub effect_index: Option<usize>,
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
    pub image_border: f32,

    pub inner_width: f32,
    pub inner_height: f32,
    pub thickness: f32,
    pub offset: Vec2,
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
    pub width: f32,
    pub height: f32,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
#[derive(Reflect)]
pub struct HoldViewRon {
    pub bg_image_asset_path: String,
    pub border_width: f32,
    pub font_color: Color,
    pub width: f32,
    pub height: f32,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
#[derive(Reflect)]
pub struct ManualViewRon {
    pub bg_image_asset_path: String,
    pub border_width: f32,
    pub font_color: Color,
    pub width: f32,
    pub height: f32,
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
    pub view_margin_left: f32,
    pub view_margin_y: f32,
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
    #[serde(default)]
    pub effects: Vec<EffectRon>,
    pub drop_ball_level_max: usize,
    pub player: PlayerRon,
    pub bottle: BottleRon,
    pub background: BackgroundRon,
    pub sounds: SoundRon,
    pub ui: UiRon,
    pub ball_physics: PhysicsRon,
    pub bottle_physics: PhysicsRon,
}
