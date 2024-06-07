use crate::prelude::*;
use bevy::{
    prelude::*,
    asset::embedded_asset,
};

use bevy_common_assets::ron::RonAssetPlugin;

mod common;
use self::common::*;
mod config_popup;
mod list_ron;
use list_ron::*;


pub struct ScTitleScreenPlugin;

impl Plugin for ScTitleScreenPlugin {
    fn build(&self, app: &mut App) {

        embedded_asset!(app, "title_screen/title_bg_image.png");

        app.add_plugins((
            RonAssetPlugin::<ListRon>::new(&["list.ron"]),
        ));

        app.insert_state(TitleScreenState::Inactive);
        app.insert_resource(TitleAssets::default());
        app.insert_resource(config_popup::ConfigData::default());
        app.add_systems(OnEnter(GameState::Title), (
            activate_title_screen,
        ));
        app.add_systems(OnExit(GameState::Title), (
            inactivate_title_screen,
        ));

        app.add_systems(OnEnter(TitleScreenState::Loading), (
            start_loading,
            spawn_loading_screen,
        ));

        app.add_systems(Update,
            (
                update_loading_screen,
                check_loading,
            ).run_if(in_state(TitleScreenState::Loading))
        );

        app.add_systems(OnEnter(TitleScreenState::Idle),
            (
                spawn_title_screen,
            )
        );

        app.add_systems(Update,
            (
                read_keyboard,
            ).run_if(in_state(TitleScreenState::Idle))
        );

        app.add_systems(OnEnter(TitleScreenState::Config),
            (
                config_popup::prepare,
            )
        );
        app.add_systems(Update,
            (
                config_popup::ui_popup,
            ).run_if(in_state(TitleScreenState::Config))
        );
        app.add_systems(OnExit(TitleScreenState::Config),
            (
                save_config,
            )
        );

        app.add_systems(OnEnter(TitleScreenState::End), 
            (
                end_title_screen,
            )
        );
    }
}

fn activate_title_screen(
    mut next_state: ResMut<NextState<TitleScreenState>>,
) {
    next_state.set(TitleScreenState::Loading);
}
fn inactivate_title_screen(
    mut next_state: ResMut<NextState<TitleScreenState>>,
) {
    next_state.set(TitleScreenState::Inactive);
}


#[derive(Resource, Debug, Default)]
struct TitleAssets {
    h_bg_image: Handle<Image>,
    h_list_ron: Handle<ListRon>,
}

impl Loadable for TitleAssets {
    fn get_untyped_handles(&self) -> Vec<UntypedHandle> {
        let v = vec![
            self.h_bg_image.clone().untyped(),
            self.h_list_ron.clone().untyped(),
        ];
        v
    }
}


#[derive(Component, Debug)]
struct PreLoadingText;

fn start_loading(
    mut asset: ResMut<TitleAssets>,
    asset_server: Res<AssetServer>,
) {
    asset.h_bg_image = asset_server.load("embedded://suika_clone/title_screen/title_bg_image.png");
    asset.h_list_ron = asset_server.load("ron/index.list.ron");
}

fn spawn_loading_screen(
    mut commands: Commands,
) {
    let text_style = TextStyle {
        font_size: 30.,
        ..default()
    };
    commands.spawn((
        InTitleScreen,
        PreLoadingText,
        Text2dBundle {
            text: Text::from_section("ashiojin.com", text_style),
            transform: Transform::from_translation(Vec2::new(0., 0.).extend(-1.0)),
            ..default()
        },
    ));
}

fn update_loading_screen(
    mut q_text: Query<&mut Text, With<PreLoadingText>>,
    time: Res<Time>,
) {
    let marks = [
        '/', '-', '\\', '|'
    ];
    let one = time.elapsed_seconds() % 1.;
    let idx = one * marks.len() as f32;
    let idx = idx.floor() as usize;
    let mark = marks[idx];

    if let Ok(mut text) = q_text.get_single_mut() {
        let text_style = text.sections[0].style.clone();
        if 1 == text.sections.len() {
            text.sections.push(
                TextSection::new("", text_style));
        }
        text.sections[1].value = format!("{}", mark);
    }
}


fn check_loading(
    asset_pack: Res<TitleAssets>,
    asset_server: Res<AssetServer>,
    mut next_state: ResMut<NextState<TitleScreenState>>,
) {
    match asset_pack.get_loading_state(&asset_server) {
        LoadingState::Completed => {
            next_state.set(TitleScreenState::Idle);
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
                text: Text::from_section("[Space] : Start, [c] : Config", text_style),

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
    mut next_title_state: ResMut<NextState<TitleScreenState>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        next_title_state.set(TitleScreenState::End);
    }

    if keyboard.just_pressed(KeyCode::KeyC) {
        next_title_state.set(TitleScreenState::Config);
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

