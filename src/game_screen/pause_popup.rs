use crate::prelude::*;
use bevy::prelude::*;

use super::common::*;

//
//    +----------------+    
//    |                |    
//    |    Paused      |    
//    |                |    
//    |  press space.. |    
//    |  press esc..   |    
//    |                |    
//    +----------------+    
//
const GO_POPUP_CENTER: Vec2 = Vec2::new(0., 0.);
const GO_POPUP_SIZE: Vec2 = Vec2::new(700., 700.);
const GO_POPUP_STR_1_Y: f32 = 0. + 100.;
const GO_POPUP_STR_2_Y: f32 = 0. -  50.;
const GO_POPUP_STR_3_Y: f32 = 0. - 100.;

#[derive(Component, Debug)]
pub struct PausePopup;

#[derive(Component, Debug)]
pub struct ControllerPausePopup {
    input_suppresser: Timer,
}
#[derive(Component, Debug)]
pub struct PausePopupMessageToRestart;

pub fn setup_pause_popup(
    mut commands: Commands,
    my_assets: Res<GameAssets>,
) {
    commands.spawn((
        PausePopup,
        ControllerPausePopup{
            input_suppresser: Timer::from_seconds(1.5, TimerMode::Once)
        },
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgba(0.9, 0.9, 0.9, 0.9),
                custom_size: Some(GO_POPUP_SIZE),
                ..default()
            },
            transform: Transform::from_translation(
                           GO_POPUP_CENTER.extend(Z_POPUP_GAMEOVER)),
            ..default()
        },
    )).with_children(|b| {
        let text_style = TextStyle {
            font: my_assets.h_font.clone(),
            font_size: 60.0,
            color: Color::GREEN,
        };
        b.spawn((
            Text2dBundle {
                text: Text::from_section("Paused", text_style),
                transform: Transform::from_translation(
                    Vec2::new(0., GO_POPUP_STR_1_Y).extend(Z_POPUP_GAMEOVER + 0.01)
                ),
                text_anchor: bevy::sprite::Anchor::Center,
                ..default()
            },
        ));
        let text_style = TextStyle {
            font: my_assets.h_font.clone(),
            font_size: 30.0,
            color: Color::BLACK,
        };
        b.spawn((
            Text2dBundle {
                text: Text::from_section(
                    "Press [Space] to resume", text_style),
                transform: Transform::from_translation(
                    Vec2::new(0., GO_POPUP_STR_2_Y).extend(Z_POPUP_GAMEOVER + 0.01)
                ),
                text_anchor: bevy::sprite::Anchor::Center,
                ..default()
            },
        ));
        let text_style = TextStyle {
            font: my_assets.h_font.clone(),
            font_size: 30.0,
            color: Color::BLACK,
        };
        b.spawn((
            PausePopupMessageToRestart,
            Text2dBundle {
                text: Text::from_section(
                    "Press [Esc] to back to title", text_style),
                transform: Transform::from_translation(
                    Vec2::new(0., GO_POPUP_STR_3_Y).extend(Z_POPUP_GAMEOVER + 0.01)
                ),
                visibility: Visibility::Hidden,
                text_anchor: bevy::sprite::Anchor::Center,
                ..default()
            },
        ));
    });
}

#[allow(clippy::complexity)]
pub fn cleanup_pause_popup(
    mut commands: Commands,
    q_popup: Query<Entity,
        Or<(
            With<PausePopup>,
        )>
    >,
) {
    for p in q_popup.iter() {
        commands.entity(p)
            .despawn_recursive();
    }
}

pub fn read_keyboard_for_pause_popup(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_screen_state: ResMut<NextState<GameScreenState>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut q_controller: Query<&mut ControllerPausePopup>,
    mut q_popup_message: Query<&mut Visibility,
        (With<PausePopupMessageToRestart>, Without<ControllerPausePopup>)>,
    time: Res<Time>,
) {
    if let Ok(mut controller) = q_controller.get_single_mut() {
        controller.input_suppresser.tick(time.delta());
        if controller.input_suppresser.finished() {
            if let Ok(mut vis_msg) = q_popup_message.get_single_mut() {
                *vis_msg = Visibility::Inherited;
            }
            if keyboard.just_pressed(KeyCode::Escape) {
                next_state.set(GameState::Title);
            }
        }
        if keyboard.just_pressed(KeyCode::Space) {
            next_screen_state.set(GameScreenState::Playing);
        }
    }
}
