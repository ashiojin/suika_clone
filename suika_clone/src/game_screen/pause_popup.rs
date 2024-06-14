use crate::prelude::*;
use bevy::prelude::*;

use super::common::*;
use super::camera::*;

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
const POPUP_CENTER: Vec2 = Vec2::new(0., 0.);
const POPUP_SIZE: Vec2 = Vec2::new(900., 700.);
const POPUP_STR_1_Y: f32 = 0. + 100.;
const POPUP_STR_2_Y: f32 = POPUP_STR_1_Y -  60. - 16.;
const POPUP_STR_3_Y: f32 = POPUP_STR_2_Y -  36. - 8.;
const POPUP_STR_4_Y: f32 = POPUP_STR_3_Y -  36. - 8.;

#[derive(Component, Debug)]
pub struct PausePopup;

#[derive(Component, Debug)]
pub struct ControllerPausePopup {
    input_suppresser: Timer,
    long_press: Option<Timer>,
}
#[derive(Component, Debug)]
pub struct PausePopupMessageDelay;

pub fn setup_pause_popup(
    mut commands: Commands,
    my_assets: Res<GameAssets>,
) {
    commands.spawn((
        PausePopup,
        PinnedToPlayingCamera(POPUP_CENTER),
        ControllerPausePopup{
            input_suppresser: Timer::from_seconds(1.5, TimerMode::Once),
            long_press: None,
        },
        SpriteBundle {
            texture: my_assets.ui.popup.h_bg_image.clone(),
            sprite: Sprite {
                custom_size: Some(POPUP_SIZE),
                ..default()
            },
            transform: Transform::from_translation(
                           POPUP_CENTER.extend(Z_POPUP)),
            ..default()
        },
        ImageScaleMode::Sliced(TextureSlicer {
            border: BorderRect::square(my_assets.ui.popup.border_width),
            center_scale_mode: SliceScaleMode::Tile { stretch_value: 1.0 },
            sides_scale_mode: SliceScaleMode::Tile { stretch_value: 1.0 },
            ..default()
        }),

    )).with_children(|b| {
        let text_style = TextStyle {
            font: my_assets.h_font.clone(),
            font_size: 60.0,
            color: my_assets.ui.popup.font_color,
        };
        b.spawn((
            Text2dBundle {
                text: Text::from_section("Paused", text_style),
                transform: Transform::from_translation(
                    Vec2::new(0., POPUP_STR_1_Y).extend(Z_POPUP + 0.01)
                ),
                text_anchor: bevy::sprite::Anchor::Center,
                ..default()
            },
        ));
        let text_style = TextStyle {
            font: my_assets.h_font.clone(),
            font_size: 36.0,
            color: my_assets.ui.popup.font_color_sub,
        };
        b.spawn((
            Text2dBundle {
                text: Text::from_section(
                    format!("Press [{}] to resume", GpKbInput::Start.get_str()), text_style),
                transform: Transform::from_translation(
                    Vec2::new(0., POPUP_STR_2_Y).extend(Z_POPUP + 0.01)
                ),
                text_anchor: bevy::sprite::Anchor::Center,
                ..default()
            },
        ));
        let text_style = TextStyle {
            font: my_assets.h_font.clone(),
            font_size: 36.0,
            color: my_assets.ui.popup.font_color_sub,
        };
        b.spawn((
            PausePopupMessageDelay,
            Text2dBundle {
                text: Text::from_section(
                    format!("Press [{}] to restart", GpKbInput::Select.get_str()), text_style),
                transform: Transform::from_translation(
                    Vec2::new(0., POPUP_STR_3_Y).extend(Z_POPUP + 0.01)
                ),
                visibility: Visibility::Hidden,
                text_anchor: bevy::sprite::Anchor::Center,
                ..default()
            },
        ));
        let text_style = TextStyle {
            font: my_assets.h_font.clone(),
            font_size: 36.0,
            color: my_assets.ui.popup.font_color_sub,
        };
        b.spawn((
            PausePopupMessageDelay,
            Text2dBundle {
                text: Text::from_section(
                    format!("Press [{}] for 3s to back to title", GpKbInput::Select.get_str()), text_style),
                transform: Transform::from_translation(
                    Vec2::new(0., POPUP_STR_4_Y).extend(Z_POPUP + 0.01)
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

pub fn update_pause_popup(
    mut q_controller: Query<&mut ControllerPausePopup>,
    mut q_popup_message: Query<&mut Visibility,
        (With<PausePopupMessageDelay>, Without<ControllerPausePopup>)>,
    time: Res<Time>,
) {
    if let Ok(mut controller) = q_controller.get_single_mut() {
        controller.input_suppresser.tick(time.delta());
        controller.long_press.iter_mut()
            .for_each(|p| {
                p.tick(time.delta());
            });
        if controller.input_suppresser.finished() {
            for mut vis_msg in q_popup_message.iter_mut() {
                *vis_msg = Visibility::Inherited;
            }
        }
    }
}

#[derive(Event, Debug)]
pub enum PausePopupInput {
    Resume,
    Restart,
    GoToTitle,
}

pub fn read_keyboard_for_pause_popup(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut ev_input: EventWriter<PausePopupInput>,
    mut q_controller: Query<&mut ControllerPausePopup>,
) {
    if let Ok(mut controller) = q_controller.get_single_mut() {
        if keyboard.any_just_pressed(KEYBOARD_KEYS_START) {
            ev_input.send(PausePopupInput::Resume);
        }

        if keyboard.any_just_pressed(KEYBOARD_KEYS_SELECT)
            && controller.long_press.is_none() {
            controller.long_press = Some(Timer::from_seconds(3.0, TimerMode::Once));
        }
        if keyboard.any_just_released(KEYBOARD_KEYS_SELECT) {
            ev_input.send(PausePopupInput::Restart);
        }
        if let Some(t) = controller.long_press.as_ref() {
            if t.finished() {
                ev_input.send(PausePopupInput::GoToTitle);
            }
        }
    }
}


pub fn read_gamepad_for_pause_popup(
    connected_gamepad: Option<Res<ConnectedGamePad>>,
    buttons: Res<ButtonInput<GamepadButton>>,
    mut ev_input: EventWriter<PausePopupInput>,
    mut q_controller: Query<&mut ControllerPausePopup>,
) {
    if let Some(&ConnectedGamePad(gamepad)) = connected_gamepad.as_deref() {
        if let Ok(mut controller) = q_controller.get_single_mut() {
            if buttons.any_just_pressed(to_gamepad_btn(gamepad, &GAMEPAD_BTNS_START)) {
                ev_input.send(PausePopupInput::Resume);
            }

            if buttons.any_just_pressed(to_gamepad_btn(gamepad, &GAMEPAD_BTNS_SELECT))
                && controller.long_press.is_none() {
                controller.long_press = Some(Timer::from_seconds(3.0, TimerMode::Once));
            }
            if buttons.any_just_released(to_gamepad_btn(gamepad, &GAMEPAD_BTNS_SELECT)) {
                ev_input.send(PausePopupInput::Restart);
            }
            if let Some(t) = controller.long_press.as_ref() {
                if t.finished() {
                    ev_input.send(PausePopupInput::GoToTitle);
                }
            }
        }
    }
}
pub fn act_pause_popup(
    mut next_screen_state: ResMut<NextState<GameScreenState>>,
    mut next_state: ResMut<NextState<GameState>>,
    q_controller: Query<&ControllerPausePopup>,
    mut ev_input: EventReader<PausePopupInput>,
) {
    if let Ok(controller) = q_controller.get_single() {
        for ev in ev_input.read() {
            match ev {
                PausePopupInput::Resume => {
                    next_screen_state.set(GameScreenState::Playing);
                },
                PausePopupInput::Restart => {
                    if controller.input_suppresser.finished() {
                        next_screen_state.set(GameScreenState::Restart);
                    }
                },
                PausePopupInput::GoToTitle => {
                    if controller.input_suppresser.finished() {
                        next_state.set(GameState::Title);
                    }
                },
            }
        }
    }

}
