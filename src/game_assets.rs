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

#[derive(Resource, Debug)]
pub struct GameAssets {
    ball_level_settings: Vec<BallLevelDef>,
    pub h_font: Handle<Font>,

    pub h_bgm: Handle<AudioSource>,
    pub h_se_combine: Handle<AudioSource>,
}
impl Loadable for GameAssets {
    fn get_untyped_handles(&self) -> Vec<UntypedHandle> {
        let mut v: Vec<_> = self.ball_level_settings.iter()
            .map(|x| &x.h_image).cloned().map(|h| h.untyped()).collect();
        let mut v2 = vec![
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
        h_font: Handle<Font>,
        h_bgm: Handle<AudioSource>,
        h_se_combine: Handle<AudioSource>,
    ) -> Self {
        Self {
            ball_level_settings,
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
