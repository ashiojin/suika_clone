use crate::prelude::*;
use bevy::prelude::*;

use game_ron::*;

#[derive(Debug)]
pub struct BallLevelDef {
    pub physics_radius: f32,

    pub view_width: f32,
    pub view_height: f32,

    pub h_image: Handle<Image>,

    pub effect_index: Option<usize>,
}

impl BallLevelDef {
    pub fn create_with_loading(n: &BallLevelSettingRon, asset_server: &AssetServer,) -> Self {
        Self {
            physics_radius: n.physics_radius,
            view_width: n.view_width,
            view_height: n.view_height,
            h_image: asset_server.load(&n.image_asset_path),
            effect_index: n.effect_index,
        }
    }
}

pub mod effects {
    use bevy::prelude::*;
    use bevy_prng::ChaCha8Rng;
    use bevy_rand::component::EntropyComponent;
    use rand_core::RngCore;

    #[derive(Debug, Clone)]
    pub struct RandRange<T>(pub T, pub T);

    impl<T: Clone> RandRange<T> {
        pub fn from(ron: &game_ron::effects::RandRange<T>) -> Self {
            Self(ron.0.clone(), ron.1.clone())
        }
    }
    impl RandRange<f32> {
        pub fn rand(&self, rnd: &mut EntropyComponent<ChaCha8Rng>) -> f32 {
            let t = rnd.next_u32() as f32 / u32::MAX as f32;
            (self.1 - self.0) * t + self.0
        }
    }
    impl RandRange<Vec2> {
        pub fn rand(&self, rnd: &mut EntropyComponent<ChaCha8Rng>) -> Vec2 {
            let t = rnd.next_u32() as f32 / u32::MAX as f32;
            self.0.lerp(self.1, t)
        }
    }
    impl RandRange<usize> {
        pub fn rand(&self, rnd: &mut EntropyComponent<ChaCha8Rng>) -> usize {
            (rnd.next_u32() % (self.1 - self.0 + 1) as u32) as usize + self.0
        }
    }

    #[derive(Debug, Clone, Default)]
    pub struct Linear<T: Clone>(pub Vec<T>);
    impl<T:Clone> Linear<T> {
        pub fn from(ron: &game_ron::effects::Linear<T>) -> Self {
            Self(ron.0.clone())
        }
    }
    impl Linear<f32> {
        pub fn get(&self, fraction: f32) -> f32 {
            if self.0.len() == 1 {
                self.0[0]
            } else if fraction >= 1.0 {
                *self.0.last().unwrap()
            } else {
                let len = self.0.len();
                let idx_l = (fraction * (len-1) as f32).floor() as usize;
                let remain = (fraction - idx_l as f32 / (len-1) as f32) * (len-1) as f32;
                let l = self.0[idx_l];
                let r = self.0[idx_l + 1];

                l.lerp(r, remain)
            }
        }
    }


    #[derive(Debug, Clone)]
    pub struct Scattering {
        pub h_images: Vec<Handle<Image>>,
        pub image_scale: f32,
        pub alpha: Linear<f32>,
        pub red: Linear<f32>,
        pub green: Linear<f32>,
        pub blue: Linear<f32>,
        pub theta: RandRange<f32>,
        pub velocity: RandRange<f32>,
        pub rotation: RandRange<f32>,
        pub accelation: RandRange<Vec2>,
        pub num: RandRange<usize>,
        pub time: RandRange<f32>,
    }
}


#[derive(Debug)]
pub enum EffectDef {
    Scattering(effects::Scattering),
}

impl EffectDef {
    pub fn create_with_loading(ron: &EffectRon, asset_server: &AssetServer) -> Self {
        match ron {
            game_ron::EffectRon::Scattering(s) => {
                let h_images = s.image_asset_paths.iter().map(|p|
                    asset_server.load(p)
                ).collect();

                Self::Scattering(effects::Scattering{
                    h_images,
                    image_scale: s.image_scale,
                    alpha: effects::Linear::from(&s.alpha),
                    red: effects::Linear::from(&s.red),
                    green: effects::Linear::from(&s.green),
                    blue: effects::Linear::from(&s.blue),
                    theta: effects::RandRange::from(&s.theta),
                    velocity: effects::RandRange::from(&s.velocity),
                    rotation: effects::RandRange::from(&s.rotation),
                    accelation: effects::RandRange::from(&s.accelation),
                    num: effects::RandRange::from(&s.num),
                    time: effects::RandRange::from(&s.time),
                })
            }
        }
    }
    fn get_untyped_handles(&self) -> Vec<UntypedHandle> {
        match self {
            EffectDef::Scattering(s) => {
                s.h_images.iter().map(|h| h.clone().untyped()).collect()
            }
        }
    }
}

#[derive(Debug)]
pub struct PlayerDef {
    pub view_width: f32,
    pub view_height: f32,
    pub offset_x: f32,
    pub offset_y: f32,
    pub h_image: Handle<Image>,
    pub guide_color: Color,

    pub speed: f32,
}
impl PlayerDef {
    pub fn create_with_loading(ron: &PlayerRon, asset_server: &AssetServer) -> Self {
        Self {
            view_width: ron.view_width,
            view_height: ron.view_height,
            offset_x: ron.offset_x,
            offset_y: ron.offset_y,
            h_image: asset_server.load(&ron.image_asset_path),
            guide_color: ron.guide_color,
            speed: ron.speed,
        }
    }
}

#[derive(Debug)]
pub struct BottleDef {
    pub h_fg_image: Handle<Image>,
    pub h_bg_image: Handle<Image>,
    pub image_border: f32,

    pub inner_width: f32,
    pub inner_height: f32,
    pub thickness: f32,

    pub offset: Vec2,
}
impl BottleDef {
    pub fn create_with_loading(ron: &BottleRon, asset_server: &AssetServer) -> Self {
        Self {
            h_fg_image: asset_server.load(&ron.fg_image_asset_path),
            h_bg_image: asset_server.load(&ron.bg_image_asset_path),
            image_border: ron.image_border,
            inner_width: ron.inner_width,
            inner_height: ron.inner_height,
            thickness: ron.thickness,
            offset: ron.offset,
        }
    }

    pub fn outer_size(&self) -> Vec2 {
        Vec2::new(
            self.inner_width + self.thickness * 2.,
            self.inner_height + self.thickness,
        )
    }
    pub fn bottom_size(&self) -> Vec2 {
        let outer_size = self.outer_size();
        Vec2::new(
            outer_size.x,
            self.thickness,
        )
    }
    pub fn side_size(&self) -> Vec2 {
        let outer_size = self.outer_size();
        Vec2::new(
            self.thickness,
            outer_size.y,
        )
    }
    pub fn left_top(&self) -> Vec2 {
        let outer_size = self.outer_size();
        Vec2::new(
            -1. * outer_size.x * 0.5 + self.offset.x,
            -1. * -outer_size.y * 0.5 + self.offset.y,
        )
    }
    pub fn right_bottom(&self) -> Vec2 {
        let outer_size = self.outer_size();
        Vec2::new(
            outer_size.x * 0.5 + self.offset.x,
            -outer_size.y * 0.5 + self.offset.y,
        )
    }
}


#[derive(Debug)]
pub struct HoldViewDef {
    pub h_bg_image: Handle<Image>,
    pub border_width: f32,
    pub font_color: Color,
    pub width: f32,
    pub height: f32,
}
#[derive(Debug)]
pub struct ScoreViewDef {
    pub h_bg_image: Handle<Image>,
    pub border_width: f32,
    pub font_color: Color,
    pub width: f32,
    pub height: f32,
}
#[derive(Debug)]
pub struct ManualViewDef {
    pub h_bg_image: Handle<Image>,
    pub border_width: f32,
    pub font_color: Color,
    pub width: f32,
    pub height: f32,
}
#[derive(Debug)]
pub struct PopupDef {
    pub h_bg_image: Handle<Image>,
    pub border_width: f32,
    pub font_color: Color,
    pub font_color_sub: Color,
}
#[derive(Debug)]
pub struct UiDef {
    pub hold_view: HoldViewDef,
    pub score_view: ScoreViewDef,
    pub manual_view: ManualViewDef,
    pub view_margin_left: f32,
    pub view_margin_y: f32,
    pub popup: PopupDef,
}
impl UiDef {
    pub fn create_with_loading(ron: &UiRon, asset_server: &AssetServer) -> Self {
        Self {
            hold_view: HoldViewDef {
                h_bg_image: asset_server.load(&ron.hold_view.bg_image_asset_path),
                border_width: ron.hold_view.border_width,
                font_color: ron.hold_view.font_color,
                width: ron.hold_view.width,
                height: ron.hold_view.height,
            },
            score_view: ScoreViewDef {
                h_bg_image: asset_server.load(&ron.score_view.bg_image_asset_path),
                border_width: ron.score_view.border_width,
                font_color: ron.score_view.font_color,
                width: ron.score_view.width,
                height: ron.score_view.height,
            },
            manual_view: ManualViewDef {
                h_bg_image: asset_server.load(&ron.manual_view.bg_image_asset_path),
                border_width: ron.manual_view.border_width,
                font_color: ron.manual_view.font_color,
                width: ron.manual_view.width,
                height: ron.manual_view.height,
            },
            view_margin_left: ron.view_margin_left,
            view_margin_y: ron.view_margin_y,
            popup: PopupDef {
                h_bg_image: asset_server.load(&ron.popup.bg_image_asset_path),
                border_width: ron.popup.border_width,
                font_color: ron.popup.font_color,
                font_color_sub: ron.popup.font_color_sub,
            },
        }
    }

    fn get_untyped_handles(&self) -> Vec<UntypedHandle> {
        vec![
            self.hold_view.h_bg_image.clone().untyped(),
            self.score_view.h_bg_image.clone().untyped(),
            self.manual_view.h_bg_image.clone().untyped(),
            self.popup.h_bg_image.clone().untyped(),
        ]
    }
}

#[derive(Debug)]
pub struct BackgroundDef {
    pub h_bg_image: Handle<Image>,
    pub offset: Vec2,
}
impl BackgroundDef {
    pub fn create_with_loading(ron: &BackgroundRon, asset_server: &AssetServer) -> Self {
        Self {
            h_bg_image: asset_server.load(&ron.bg_image_asset_path),
            offset: ron.offset,
        }
    }

    fn get_untyped_handles(&self) -> Vec<UntypedHandle> {
        vec![
            self.h_bg_image.clone().untyped(),
        ]
    }
}


#[derive(Debug)]
pub struct SoundDef {
    pub h_bgm: Handle<AudioSource>,
    pub bgm_scale: f32,
    pub h_se_combine: Handle<AudioSource>,
    pub se_combine_scale: f32,
}
impl SoundDef {
    pub fn create_with_loading(ron: &SoundRon, asset_server: &AssetServer) -> Self {
        Self {
            h_bgm: asset_server.load(&ron.bgm_asset_path),
            bgm_scale: ron.bgm_scale,
            h_se_combine: asset_server.load(&ron.se_combine_asset_path),
            se_combine_scale: ron.se_combine_scale,
        }
    }

    fn get_untyped_handles(&self) -> Vec<UntypedHandle> {
        vec![
            self.h_bgm.clone().untyped(),
            self.h_se_combine.clone().untyped(),
        ]
    }
}

#[derive(Debug)]
pub struct FrictionDef {
    pub dynamic_coef: f32,
    pub static_coef: f32,
}

#[derive(Debug)]
pub struct RestitutionDef {
    pub coef: f32,
}

#[derive(Debug)]
pub struct RigitBodyDef {
    pub friction: FrictionDef,
    pub restitution: RestitutionDef,
}
impl RigitBodyDef {
    pub fn from_ron(ron: &RigitBodyRon) -> Self {
        Self {
            friction: FrictionDef {
                dynamic_coef: ron.friction.dynamic_coef,
                static_coef: ron.friction.static_coef
            },
            restitution: RestitutionDef {
                coef: ron.restitution.coef
            },
        }
    }
}
#[derive(Debug)]
pub struct OtherParamDef {
    pub gravity: f32,
    pub air_damping_coef: f32,
    pub ball_grow_time: f32,
    pub area: game_ron::Area,
    pub max_velocity: f32,
    pub shake_k: f32, // max move is about 0.4 * shake_k
    pub playing_cam_offset: Vec2,
}
impl OtherParamDef {
    pub fn from_ron(ron: &OtherParamRon) -> Self {
        Self {
            gravity: ron.gravity,
            air_damping_coef: ron.air_damping_coef,
            ball_grow_time: ron.ball_grow_time,
            area: ron.area.clone(),
            max_velocity: ron.max_velocity,
            shake_k: ron.shake_k,
            playing_cam_offset: ron.playing_cam_offset,
        }
    }
}


#[derive(Resource, Debug)]
pub struct GameAssets {
    ball_level_settings: Vec<BallLevelDef>,
    effects: Vec<EffectDef>,
    pub drop_ball_level_max: BallLevel,
    pub player_settings: PlayerDef,
    pub bottle_settings: BottleDef,
    pub h_font: Handle<Font>,

    pub background: BackgroundDef,

    pub ui: UiDef,

    pub sound: SoundDef,

    pub ball_physics: RigitBodyDef,
    pub bottle_physics: RigitBodyDef,
    pub physics: OtherParamDef,
}
impl Loadable for GameAssets {
    fn get_untyped_handles(&self) -> Vec<UntypedHandle> {
        let mut v: Vec<_> = self.ball_level_settings.iter()
            .map(|x| &x.h_image).cloned().map(|h| h.untyped()).collect();
        let mut v2 = vec![
            self.player_settings.h_image.clone().untyped(),
            self.bottle_settings.h_fg_image.clone().untyped(),
            self.bottle_settings.h_bg_image.clone().untyped(),
            self.h_font.clone().untyped(),
        ];
        let mut v3 = self.ui.get_untyped_handles();
        let mut v4 = self.background.get_untyped_handles();
        let mut v5 = self.sound.get_untyped_handles();
        let mut v6 = self.effects.iter()
            .flat_map(|x| x.get_untyped_handles())
            .collect();
        v.append(&mut v2);
        v.append(&mut v3);
        v.append(&mut v4);
        v.append(&mut v5);
        v.append(&mut v6);
        v
    }
}

pub const BALL_LEVEL_MIN: usize = 1;
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct BallLevel(pub usize);

impl Default for BallLevel {
    fn default() -> Self {
        Self(BALL_LEVEL_MIN)
    }
}
impl BallLevel {
    pub const fn new(lv: usize) -> Self {
        assert!(lv >= BALL_LEVEL_MIN);
        Self(lv)
    }
    pub fn from_rand_u32(rnd: u32, min: BallLevel, max: BallLevel) -> Self {
        Self::new(
            (rnd as usize % (max.0-min.0+1)) + min.0
        )
    }
}

impl GameAssets {
    #[allow(clippy::too_many_arguments)] // FIXME: nicer api (take extra care of ball_level_settings)
    pub fn new(
        ball_level_settings: Vec<BallLevelDef>,
        effects: Vec<EffectDef>,
        drop_ball_level_max: BallLevel,
        player_settings: PlayerDef,
        bottle_settings: BottleDef,
        background: BackgroundDef,
        ui: UiDef,
        h_font: Handle<Font>,
        sound: SoundDef,

        ball_physics: RigitBodyDef,
        bottle_physics: RigitBodyDef,
        physics: OtherParamDef,

    ) -> Self {
        Self {
            ball_level_settings,
            effects,
            drop_ball_level_max,
            player_settings,
            bottle_settings,
            h_font,
            background,
            ui,
            sound,
            ball_physics,
            bottle_physics,
            physics,
        }
    }
    pub fn get_ball_image(&self, level: BallLevel) -> &Handle<Image> {
        &self.get_ball_setting(level).h_image
    }

    #[inline]
    pub fn get_ball_max_level(&self) -> BallLevel {
        BallLevel (
            self.ball_level_settings.len()
        )
    }

    #[inline]
    pub fn get_ball_setting(&self, lv: BallLevel) -> &BallLevelDef {
        assert!(self.get_ball_max_level() >= lv);
        &self.ball_level_settings[lv.0 - BALL_LEVEL_MIN]
    }

    #[inline]
    pub fn get_ball_r(&self, lv: BallLevel) -> f32 {
        self.get_ball_setting(lv).physics_radius
    }

    pub fn get_ball_effect(&self, lv_combined: BallLevel) -> Option<&EffectDef> {
        if let Some(idx) = self.get_ball_setting(lv_combined).effect_index {
            Some(
                &self.effects[idx]
            )
        } else {
            None
        }
    }

    #[inline]
    pub fn get_ball_start_r(&self, lv: BallLevel) -> f32 {
        //
        //   -  *
        //   |  ***
        //   |  *  **  y+r      r: min(ball radius)
        //   y  *    **         y: combined ball radius
        //   |  *      **       x: max radius of new free space <- this!
        //   |  *        **
        //   |  *   x+r    **
        //   =  *------------*
        //   |  *          **
        //   |  *        **
        //   y  *      **
        //   |  *    **
        //   |  *  **
        //   |  ***
        //   _  *
        //
        let r = self.get_ball_r(BallLevel::new(BALL_LEVEL_MIN));
        let y = self.get_ball_r(BallLevel::new(lv.0 - 1));

        (2. * r * y + r * r).powf(1. / 2.) - r
    }

    #[inline]
    pub fn get_ball_mesh_wh(&self, lv: BallLevel) -> (f32, f32) {
        let s = self.get_ball_setting(lv);
        (s.view_width, s.view_height)
    }

    pub fn bottle_center(&self) -> Vec2 {
        self.bottle_settings.offset
    }
    pub fn bottle_outer_size(&self) -> Vec2 {
        self.bottle_settings.outer_size()
    }

    pub fn _bottle_left_top(&self) -> Vec2 {
        self.bottle_settings.left_top()
    }
    pub fn _bottle_right_bottom(&self) -> Vec2 {
        self.bottle_settings.right_bottom()
    }

    pub fn score_size(&self) -> Vec2 {
        Vec2::new(
            self.ui.score_view.width,
            self.ui.score_view.height,
        )
    }

    pub fn score_center(&self) -> Vec2 {
        Vec2::new(
            self.bottle_settings.right_bottom().x
                + self.ui.view_margin_left
                + self.ui.score_view.width * 0.5,
            self.bottle_settings.left_top().y
                - self.ui.score_view.height * 0.5,
        )
    }

    pub fn hold_view_size(&self) -> Vec2 {
        Vec2::new(
            self.ui.hold_view.width,
            self.ui.hold_view.height,
        )
    }

    pub fn hold_view_center(&self) -> Vec2 {
        Vec2::new(
            self.bottle_settings.right_bottom().x
                + self.ui.view_margin_left + self.ui.hold_view.width * 0.5,
            self.bottle_settings.left_top().y
                - self.ui.score_view.height
                - self.ui.view_margin_y
                - self.ui.hold_view.height * 0.5,
        )
    }

    pub fn manual_view_size(&self) -> Vec2 {
        Vec2::new(
            self.ui.manual_view.width,
            self.ui.manual_view.height,
        )
    }

    pub fn manual_view_center(&self) -> Vec2 {
        Vec2::new(
            self.bottle_settings.right_bottom().x
                + self.ui.view_margin_left
                + self.ui.manual_view.width * 0.5,
            self.bottle_settings.left_top().y
                - self.ui.score_view.height
                - self.ui.view_margin_y
                - self.ui.hold_view.height
                - self.ui.view_margin_y
                - self.ui.manual_view.height * 0.5,
        )
    }
}
