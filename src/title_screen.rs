use crate::prelude::*;
use bevy::{
    prelude::*,
    asset::{
        embedded_asset, LoadState
    },
};

pub struct ScTitleScreenPlugin;

impl Plugin for ScTitleScreenPlugin {
    fn build(&self, app: &mut App) {

        embedded_asset!(app, "title_screen/title_bg_image.png");

        app.insert_state(TitleState::Loading);
        app.insert_resource(TitleAssets::default());
        app.add_systems(OnEnter(GameState::Title), (
            load_title_screen,
        ));

        app.add_systems(Update,
            (
                check_loading,
            ).run_if(in_state(TitleState::Loading))
        );

        app.add_systems(OnEnter(TitleState::Idle),
            (
                spawn_title_screen,
            )
        );

        app.add_systems(Update,
            (
                read_keyboard,
            ).run_if(in_state(TitleState::Idle))
        );

        app.add_systems(OnEnter(TitleState::End), 
            (
                end_title_screen,
            )
        );

    }
}

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
enum TitleState {
    #[default]
    Loading,
    Idle,
    End,
}

#[derive(Resource, Debug, Default)]
struct TitleAssets {
    h_bg_image: Handle<Image>,
}

impl TitleAssets {
    pub fn get_untyped_handles(&self) -> Vec<UntypedHandle> {
        let v = vec![
            self.h_bg_image.clone().untyped(),
        ];
        v
    }
}

#[derive(Component, Debug)]
struct InTitleScreen;


fn load_title_screen(
    mut asset: ResMut<TitleAssets>,
    asset_server: Res<AssetServer>,
) {
    asset.h_bg_image = asset_server.load("embedded://suika_clone/title_screen/title_bg_image.png");
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
enum LoadingState {
    Ok,
    Loading,
    Error,
}
fn summarise_assetpack_loadstate(
    asset_pack: &TitleAssets,
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

fn check_loading(
    asset_pack: Res<TitleAssets>,
    asset_server: Res<AssetServer>,
    mut next_state: ResMut<NextState<TitleState>>,
) {
    let state = summarise_assetpack_loadstate(&asset_pack, &asset_server);
    match state {
        LoadingState::Ok => {
            next_state.set(TitleState::Idle);
        }
        LoadingState::Loading => {
            // wait for next
        }
        LoadingState::Error => {
            panic!("load failed!");
        }
    }
}

fn spawn_title_screen(
    mut commands: Commands,
    asset: Res<TitleAssets>,
) {
    commands.spawn((
        InTitleScreen,
        SpriteBundle {
            texture: asset.h_bg_image.clone(),
            transform: Transform::from_translation(Vec2::new(0., 0.).extend(0.0)),
            ..default()
        },
    )).with_children(|b| {
        let text_style = TextStyle {
            font_size: 30.,
            ..default()
        };
        b.spawn((
            Text2dBundle {
                text: Text::from_section("Press [Space] to Start.", text_style),

                transform:
                    Transform::from_translation(
                        Vec2::new(0., -30.).extend(0.1)
                    ),
                ..default()
            },
        ));
    });
}


fn read_keyboard(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_title_state: ResMut<NextState<TitleState>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        next_title_state.set(TitleState::End);
    }
}

fn end_title_screen(
    mut commands: Commands,
    q_title_entities: Query<Entity, With<InTitleScreen>>,
    mut next_state: ResMut<NextState<GameState>>
) {
    for e in q_title_entities.iter() {
        commands.entity(e)
            .despawn_recursive();
    }
    next_state.set(GameState::Loading);
}
