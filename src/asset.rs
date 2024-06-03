use std::f32::consts::TAU;
use crate::prelude::*;
use bevy::{
    prelude::*,
    asset::LoadState,
};



#[derive(Debug)]
pub struct BallLevelDef {
    pub physics_radius: f32,

    pub view_width: f32,
    pub view_height: f32,

    pub h_image: Handle<Image>,
}

impl BallLevelDef {
    fn from(n: &BallLevelSettingRon, asset_server: &AssetServer,) -> Self {
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
    pub ball_level_settings: Vec<BallLevelDef>,
    pub h_font: Handle<Font>,

    pub h_bgm: Handle<AudioSource>,
    pub h_se_combine: Handle<AudioSource>,
}
impl GameAssets {
    pub fn get_untyped_handles(&self) -> Vec<UntypedHandle> {
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

    pub fn get_ball_image(&self, level: BallLevel) -> &Handle<Image> {
        let idx = level.0 - BALL_LEVEL_MIN;
        &self.ball_level_settings[idx].h_image
    }

    #[inline]
    pub fn get_ball_max_level(&self) -> BallLevel {
        BallLevel (
            self.ball_level_settings.len() + 1
        )
    }

    #[inline]
    pub fn get_ball_setting(&self, lv: BallLevel) -> &BallLevelDef {
        assert!(self.get_ball_max_level() >= lv);
        &self.ball_level_settings[lv.0]
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
pub const BALL_LEVEL_MIN: usize = 1;


pub struct ScLoadingScreenPlugin;

impl Plugin for ScLoadingScreenPlugin {
    fn build(&self, app: &mut App) {
        app.init_gizmo_group::<MyLoadingScreenGizmos>();

        app.add_systems(OnEnter(GameState::Loading), (
            load_assets,
            setup_loading_screen,
        ));
        app.add_systems(Update, (
            check_loading,
            update_loading_screen,
        ).run_if(in_state(GameState::Loading)));
        app.add_systems(OnExit(GameState::Loading), (
            cleanup_loading_screen,
        ));
    }
}

pub fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    config: Res<Config>,
) {
    let from_ron = &config.game_ron;
    let balls = from_ron.balls.iter()
        .map(|n| BallLevelDef::from(n, &asset_server))
        .collect();
    let h_bgm =asset_server.load(&from_ron.sounds.bgm_asset_path);
    let h_se_combine = asset_server.load(&from_ron.sounds.se_combine_asset_path);

    commands.insert_resource(
        GameAssets {
            ball_level_settings: balls,
            h_font: asset_server.load("fonts/GL-CurulMinamoto.ttf"),
            h_bgm,
            h_se_combine,
        }
    );
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum LoadingState {
    Ok,
    Loading,
    Error,
}
fn summarise_assetpack_loadstate(
    asset_pack: &GameAssets,
    asset_server: &AssetServer,
) -> LoadingState {
    asset_pack.get_untyped_handles()
        .iter()
        .map(|h| asset_server.get_load_states(h.id()))
        .filter_map(|s| s.map(|(s, _, _)| s))
        .fold(LoadingState::Ok, |a, s| {
            let s = match s {
                LoadState::Loaded => LoadingState::Ok,
                LoadState::Failed => LoadingState::Error,
                _ => LoadingState::Loading,
            };
            LoadingState::max(a, s)
        })
}

pub fn check_loading(
    asset_pack: Res<GameAssets>,
    asset_server: Res<AssetServer>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let state = summarise_assetpack_loadstate(&asset_pack, &asset_server);
    match state {
        LoadingState::Ok => {
            next_state.set(GameState::InGame);
        }
        LoadingState::Loading => {
            // wait for next
        }
        LoadingState::Error => {
            panic!("load failed!");
        }
    }
}

#[derive(Component, Debug)]
pub struct ForLoadingScreen;
#[derive(GizmoConfigGroup, Default, Reflect)]
pub struct MyLoadingScreenGizmos {}

pub fn setup_loading_screen(
    mut commands: Commands,
    mut config_gizmos: ResMut<GizmoConfigStore>,
) {
    commands.spawn((
        ForLoadingScreen,
        SpriteBundle {
            sprite: Sprite {
                color: Color::BLACK,
                custom_size: Some(Vec2::new(600., 300.)),
                ..default()
            },
            transform: Transform::from_translation(Vec2::new(0., 0.).extend(0.0)),
            ..default()
        },
    )).with_children(|b| {
        let text_style = TextStyle {
            font_size: 60.0,
            color: Color::WHITE,
            ..default()
        };
        b.spawn((
            Text2dBundle {
                text: Text::from_section("Now Loading...", text_style),
                transform: Transform::from_translation(Vec2::new(300., -150.).extend(0.1)),
                text_anchor: bevy::sprite::Anchor::BottomRight,
                ..default()
            },
        ));
    });

    let (config, ..) = config_gizmos.config_mut::<MyLoadingScreenGizmos>();
    config.line_width = 5.;
}

pub fn update_loading_screen(
    mut gizmos: Gizmos<MyLoadingScreenGizmos>,
    time: Res<Time>,
) {
    let second_hand = (time.elapsed_seconds() % 1.0) * TAU;
    gizmos.arrow_2d(
        Vec2::ZERO,
        Vec2::from_angle(second_hand) * 100.,
        Color::YELLOW,
    );
}

pub fn cleanup_loading_screen(
    mut commands: Commands,
    q_screen_items: Query<Entity, With<ForLoadingScreen>>,
) {
    for e in q_screen_items.iter() {
        commands.entity(e)
            .despawn_recursive();
    }
}

