use std::f32::consts::TAU;
use crate::prelude::*;
use bevy::prelude::*;

use crate::game_ron::*;

use bevy_common_assets::ron::RonAssetPlugin;

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
impl GameAssets {
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
        app.add_plugins((
            RonAssetPlugin::<GameRon>::new(&["game.ron"]),
        ));
        app.init_gizmo_group::<MyLoadingScreenGizmos>();

        app.init_state::<LoadingScreenState>();

        app.add_systems(OnEnter(GameState::Loading), (
            kick_loading,
            setup_loading_screen,
        ));
        app.add_systems(Update, (
            update_loading_screen,
        ).run_if(in_state(GameState::Loading)));
        app.add_systems(OnExit(GameState::Loading), (
            cleanup_loading_screen,
        ));

        app.add_systems(OnEnter(LoadingScreenState::LoadingGameRon),
            (
                start_loading_game_ron,
            )
        );

        app.add_systems(Update,
            (
                wait_to_complete_loading_game_ron,
            ).run_if(in_state(LoadingScreenState::LoadingGameRon))
        );

        app.add_systems(OnEnter(LoadingScreenState::LoadingGameAssets),
            (
                load_assets_game_assets,
            )
        );

        app.add_systems(Update,
            (
                wait_to_complete_loading_game_assets,
            ).run_if(in_state(LoadingScreenState::LoadingGameAssets))
        );

        app.add_systems(OnEnter(LoadingScreenState::Completed),
            (
                complete_loading,
            )
        );

    }
}

#[derive(States, Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
enum LoadingScreenState {
    #[default]
    NotLoaded,
    LoadingGameRon,
    LoadingGameAssets,
    Completed,
}


fn kick_loading(
    mut next_state: ResMut<NextState<LoadingScreenState>>,
) {
    next_state.set(LoadingScreenState::LoadingGameRon);
}

fn start_loading_game_ron(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    config: Res<Config>,
) {
    let game_ron_name = &config.game_ron_file_name;

    commands.insert_resource(
        CurrentGameRon(
            asset_server.load(format!("ron/{}", game_ron_name)),
        )
    );
}

fn wait_to_complete_loading_game_ron(
    game_ron: Res<CurrentGameRon>,
    asset_server: Res<AssetServer>,
    mut next_state: ResMut<NextState<LoadingScreenState>>,
) {
    match game_ron.get_loading_state(&asset_server) {
        LoadingState::Completed => {
            next_state.set(LoadingScreenState::LoadingGameAssets);
        }
        LoadingState::Loading => {
            // wait for next
        }
        LoadingState::Error => {
            panic!("load failed!");
        }
    }
}

fn load_assets_game_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    current_game_ron: Res<CurrentGameRon>,
    game_ron: Res<Assets<GameRon>>,
) {
    let from_ron = game_ron.get(current_game_ron.0.id())
        .expect("game.ron is not yet loaded.");
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


fn wait_to_complete_loading_game_assets(
    asset_pack: Res<GameAssets>,
    asset_server: Res<AssetServer>,
    mut next_state: ResMut<NextState<LoadingScreenState>>,
) {
    let state = asset_pack.get_loading_state(&asset_server);
    match state {
        LoadingState::Completed => {
            next_state.set(LoadingScreenState::Completed);
        }
        LoadingState::Loading => {
            // wait for next
        }
        LoadingState::Error => {
            panic!("load failed!");
        }
    }
}

fn complete_loading(
    mut next_state: ResMut<NextState<GameState>>,
) {
    next_state.set(GameState::InGame);
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

fn update_loading_screen(
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

fn cleanup_loading_screen(
    mut commands: Commands,
    q_screen_items: Query<Entity, With<ForLoadingScreen>>,
) {
    for e in q_screen_items.iter() {
        commands.entity(e)
            .despawn_recursive();
    }
}

