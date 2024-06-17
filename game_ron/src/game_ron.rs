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
        /// num of spawn
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
pub struct RigitBodyRon {
    pub friction: FrictionRon,
    pub restitution: RestitutionRon,
}

#[derive(Reflect, Debug, Clone)]
#[derive(Deserialize, Serialize)]
pub struct Area {
    pub min_x: f32,
    pub max_x: f32,
    pub min_y: f32,
    pub max_y: f32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[derive(Reflect)]
pub struct OtherParamRon {
    #[serde(default = "OtherParamRon::get_default_graviry")]
    pub gravity: f32,
    #[serde(default = "OtherParamRon::get_default_air_damping_coef")]
    pub air_damping_coef: f32,
    #[serde(default = "OtherParamRon::get_default_ball_grow_time")]
    pub ball_grow_time: f32,
    #[serde(default = "OtherParamRon::get_default_area")]
    pub area: Area,
    #[serde(default = "OtherParamRon::get_default_max_velocity")]
    pub max_velocity: f32,
    #[serde(default = "OtherParamRon::get_default_shake_k")]
    pub shake_k: f32, // max move is about 0.4 * shake_k
    #[serde(default = "OtherParamRon::get_default_playing_cam_offset")]
    pub playing_cam_offset: Vec2,
}
impl Default for OtherParamRon {
    fn default() -> Self {
        Self {
            gravity: 9.81 * 200.,
            air_damping_coef: 0.000005,
            ball_grow_time: 0.8,
            area: Area {
                min_x: -700.0,
                max_x: 700.0,
                min_y: -500.0,
                max_y: 10000.0,
            },
            max_velocity: 3000.,
            shake_k: 24. / 0.4,
            playing_cam_offset: Vec2::new(100., 0.),
        }
    }
}
impl OtherParamRon {
    fn get_default_graviry() -> f32 { 9.81 * 200. }
    fn get_default_air_damping_coef() -> f32 { 0.000005 }
    fn get_default_ball_grow_time() -> f32 { 0.8 }
    fn get_default_area() -> Area {
        Area {
                min_x: -700.0,
                max_x: 700.0,
                min_y: -500.0,
                max_y: 10000.0,
        }
    }
    fn get_default_max_velocity() -> f32 { 3000. }
    fn get_default_shake_k() -> f32 { 24. / 0.4 }
    fn get_default_playing_cam_offset() -> Vec2 { Vec2::new(100., 0.) }
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
    pub ball_physics: RigitBodyRon,
    pub bottle_physics: RigitBodyRon,
    #[serde(default)]
    pub physics: OtherParamRon,
}
