use crate::prelude::*;
use bevy::prelude::*;

use crate::game_ron::*;

#[derive(Debug)]
pub struct BallLevelDef {
    pub physics_radius: f32,

    pub view_width: f32,
    pub view_height: f32,

    pub h_image: Handle<Image>,
}

impl BallLevelDef {
    pub fn create_with_loading(n: &BallLevelSettingRon, asset_server: &AssetServer,) -> Self {
        Self {
            physics_radius: n.physics_radius,
            view_width: n.view_width,
            view_height: n.view_height,
            h_image: asset_server.load(&n.image_asset_path),
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
}
impl BottleDef {
    pub fn create_with_loading(ron: &BottleRon, asset_server: &AssetServer) -> Self {
        Self {
            h_fg_image: asset_server.load(&ron.fg_image_asset_path),
            h_bg_image: asset_server.load(&ron.bg_image_asset_path),
        }
    }
}

#[derive(Debug)]
pub struct HoldViewDef {
    pub h_bg_image: Handle<Image>,
    pub border_width: f32,
    pub font_color: Color,
}
#[derive(Debug)]
pub struct ScoreViewDef {
    pub h_bg_image: Handle<Image>,
    pub border_width: f32,
    pub font_color: Color,
}
#[derive(Debug)]
pub struct ManualViewDef {
    pub h_bg_image: Handle<Image>,
    pub border_width: f32,
    pub font_color: Color,
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
    pub popup: PopupDef,
}
impl UiDef {
    pub fn create_with_loading(ron: &UiRon, asset_server: &AssetServer) -> Self {
        Self {
            hold_view: HoldViewDef {
                h_bg_image: asset_server.load(&ron.hold_view.bg_image_asset_path),
                border_width: ron.hold_view.border_width,
                font_color: ron.hold_view.font_color,
            },
            score_view: ScoreViewDef {
                h_bg_image: asset_server.load(&ron.score_view.bg_image_asset_path),
                border_width: ron.score_view.border_width,
                font_color: ron.score_view.font_color,
            },
            manual_view: ManualViewDef {
                h_bg_image: asset_server.load(&ron.manual_view.bg_image_asset_path),
                border_width: ron.manual_view.border_width,
                font_color: ron.manual_view.font_color,
            },
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
pub struct PhysicsDef {
    pub friction: FrictionDef,
    pub restitution: RestitutionDef,
}
impl PhysicsDef {
    pub fn from_ron(ron: &PhysicsRon) -> Self {
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


#[derive(Resource, Debug)]
pub struct GameAssets {
    ball_level_settings: Vec<BallLevelDef>,
    pub drop_ball_level_max: BallLevel,
    pub player_settings: PlayerDef,
    pub bottle_settings: BottleDef,
    pub h_font: Handle<Font>,

    pub background: BackgroundDef,

    pub ui: UiDef,

    pub sound: SoundDef,

    pub ball_physics: PhysicsDef,
    pub bottle_physics: PhysicsDef,
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
        v.append(&mut v2);
        v.append(&mut v3);
        v.append(&mut v4);
        v.append(&mut v5);
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
        drop_ball_level_max: BallLevel,
        player_settings: PlayerDef,
        bottle_settings: BottleDef,
        background: BackgroundDef,
        ui: UiDef,
        h_font: Handle<Font>,
        sound: SoundDef,

        ball_physics: PhysicsDef,
        bottle_physics: PhysicsDef,

    ) -> Self {
        Self {
            ball_level_settings,
            drop_ball_level_max,
            player_settings,
            bottle_settings,
            h_font,
            background,
            ui,
            sound,
            ball_physics,
            bottle_physics,
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
}
