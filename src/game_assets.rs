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

#[derive(Resource, Debug)]
pub struct GameAssets {
    ball_level_settings: Vec<BallLevelDef>,
    pub drop_ball_level_max: BallLevel,
    pub player_settings: PlayerDef,
    pub bottle_settings: BottleDef,
    pub h_font: Handle<Font>,

    pub h_bgm: Handle<AudioSource>,
    pub h_se_combine: Handle<AudioSource>,
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

            self.h_bgm.clone().untyped(),
            self.h_se_combine.clone().untyped(),
        ];
        v.append(&mut v2);
        v
    }
}
pub const BALL_LEVEL_MIN: usize = 1;
impl GameAssets {
    pub fn new(
        ball_level_settings: Vec<BallLevelDef>,
        drop_ball_level_max: BallLevel,
        player_settings: PlayerDef,
        bottle_settings: BottleDef,
        h_font: Handle<Font>,
        h_bgm: Handle<AudioSource>,
        h_se_combine: Handle<AudioSource>,
    ) -> Self {
        Self {
            ball_level_settings,
            drop_ball_level_max,
            player_settings,
            bottle_settings,
            h_font,
            h_bgm,
            h_se_combine,
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
