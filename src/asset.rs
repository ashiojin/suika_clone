use std::f32::consts::TAU;
use crate::prelude::*;
use bevy::{
    prelude::*,
    asset::LoadState,
};

#[derive(Resource, Debug)]
pub struct ScAssets {
    pub h_balls: Vec<Handle<Image>>,
    pub h_font: Handle<Font>,
}
impl ScAssets {
    pub fn get_untyped_handles(&self) -> Vec<UntypedHandle> {
        let mut v: Vec<_> = self.h_balls.iter().cloned().map(|h| h.untyped()).collect();
        let mut v2 = vec![
            self.h_font.clone().untyped(),
        ];
        v.append(&mut v2);
        v
    }

    pub fn get_ball_image(&self, level: BallLevel) -> &Handle<Image> {
        let idx = level.0 - BALL_LEVEL_MIN;
        &self.h_balls[idx]
    }
}


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
) {

    let mut h_balls = vec![];
    for i in BALL_LEVEL_MIN..=BALL_LEVEL_MAX {
        h_balls.push(
            asset_server.load(format!("images/kao/kao_{:>02}.png", i))
        );
    }
    commands.insert_resource(
        ScAssets {
            h_balls,
            h_font: asset_server.load("fonts/GL-CurulMinamoto.ttf"),
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
    asset_pack: &ScAssets,
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
    asset_pack: Res<ScAssets>,
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

