use std::f32::consts::TAU;
use crate::prelude::*;
use bevy::prelude::*;

use game_ron::*;

use bevy_common_assets::ron::RonAssetPlugin;


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
    let game_ron_name = &config.game_ron_asset_path;

    commands.insert_resource(
        CurrentGameRon(
            asset_server.load(game_ron_name),
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
        .map(|n| BallLevelDef::create_with_loading(n, &asset_server))
        .collect();
    let effects = from_ron.effects.iter()
        .map(|r| EffectDef::create_with_loading(r, &asset_server))
        .collect();
    let player = PlayerDef::create_with_loading(&from_ron.player, &asset_server);
    let bottle = BottleDef::create_with_loading(&from_ron.bottle, &asset_server);
    let background = BackgroundDef::create_with_loading(&from_ron.background, &asset_server);
    let ui = UiDef::create_with_loading(&from_ron.ui, &asset_server);
    let sound = SoundDef::create_with_loading(&from_ron.sounds, &asset_server);
    let ball_physics = PhysicsDef::from_ron(&from_ron.ball_physics);
    let bottle_physics = PhysicsDef::from_ron(&from_ron.bottle_physics);

    commands.insert_resource(
        GameAssets::new(
            balls,
            effects,
            BallLevel(from_ron.drop_ball_level_max),
            player,
            bottle,
            background,
            ui,
            asset_server.load("embedded://suika_clone/embedded_assets/fonts/x12y12pxMaruMinyaM.ttf"),
            sound,
            ball_physics,
            bottle_physics,
        )
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
    let second_hand = -1. * (time.elapsed_seconds() % 1.0) * TAU;
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
