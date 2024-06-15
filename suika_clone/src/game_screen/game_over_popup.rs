use crate::prelude::*;
use bevy::prelude::*;

use super::common::*;
use super::camera::*;

//
//    +----------------+    
//    |                |    
//    |    Game Over   |    
//    |                |    
//    |  Score: XXXXXX |    
//    |  press space.. |    
//    |  press esc..   |    
//    |       ver:xx   |    
//    |       mode:xx  |    
//    +----------------+    
//
const POPUP_CENTER: Vec2 = Vec2::new(0., 0.);
const POPUP_SIZE: Vec2 = Vec2::new(700., 700.);
const POPUP_STR_LABEL: f32 = 0. + 100.;
const POPUP_STR_SCORE_Y: f32 = POPUP_STR_LABEL - 60. - 8.;
const POPUP_STR_HIGH_SCORE_Y: f32 = POPUP_STR_SCORE_Y - 48. - 8.;
const POPUP_STR_RESTART: f32 = POPUP_STR_HIGH_SCORE_Y - 24. - 32.;
const POPUP_STR_GOTO_TITLE: f32 = POPUP_STR_RESTART - 36. - 8.;

const POPUP_STR_5_1_Y: f32 = POPUP_STR_GOTO_TITLE - 36. - 24.;

#[derive(Component, Debug)]
pub struct GameOverPopup;

#[derive(Component, Debug)]
pub struct ControllerGameOverPopup {
    input_suppresser: Timer,
}
#[derive(Component, Debug)]
pub struct GameOverPopupMessageDelay;

pub fn setup_gameover_popup(
    mut commands: Commands,
    q_player: Query<&Player>,
    my_assets: Res<GameAssets>,
    config: Res<Config>,
    scores: Res<Scores>,
) {
    if let Ok(player) = q_player.get_single() {
        let game_cnd = GameCond::new(config.game_ron_name.as_str());
        let highscore = scores.get_highest(&game_cnd);
        let score = Score::new(player.score);
        let score_is_highest = if let Some(highscore) = highscore {
            score > *highscore
        } else {
            true
        };

        let highscore = highscore.map(|x| x.score).unwrap_or(0);
        let high_score_txt = format!("high score: {}", highscore);

        let score_txt = if score_is_highest {
            format!("New High Score:{:>6}", score.score)
        } else {
            format!("Score:{:>6}", score.score)
        };

        commands.spawn((
            GameOverPopup,
            PinnedToPlayingCamera(POPUP_CENTER),
            ControllerGameOverPopup{
                input_suppresser: Timer::from_seconds(1.5, TimerMode::Once)
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
                    text: Text::from_section("GAME OVER", text_style),
                    transform: Transform::from_translation(
                        Vec2::new(0., POPUP_STR_LABEL).extend(Z_POPUP + 0.01)
                    ),
                    text_anchor: bevy::sprite::Anchor::Center,
                    ..default()
                },
            ));
            let text_style = TextStyle {
                font: my_assets.h_font.clone(),
                font_size: 48.0,
                color: my_assets.ui.popup.font_color,
            };
            b.spawn((
                Text2dBundle {
                    text: Text::from_section(
                        score_txt, text_style),
                    transform: Transform::from_translation(
                        Vec2::new(0., POPUP_STR_SCORE_Y).extend(Z_POPUP + 0.01)
                    ),
                    text_anchor: bevy::sprite::Anchor::Center,
                    ..default()
                },
            ));
            let text_style = TextStyle {
                font: my_assets.h_font.clone(),
                font_size: 24.0,
                color: my_assets.ui.popup.font_color,
            };
            b.spawn((
                Text2dBundle {
                    text: Text::from_section(
                        high_score_txt, text_style),
                    transform: Transform::from_translation(
                        Vec2::new(0., POPUP_STR_HIGH_SCORE_Y).extend(Z_POPUP + 0.01)
                    ),
                    text_anchor: bevy::sprite::Anchor::CenterLeft,
                    ..default()
                },
            ));
            let text_style = TextStyle {
                font: my_assets.h_font.clone(),
                font_size: 36.0,
                color: my_assets.ui.popup.font_color_sub,
            };
            b.spawn((
                GameOverPopupMessageDelay,
                Text2dBundle {
                    text: Text::from_section(
                        format!("Press [{}] to restart", GpKbInput::Start.get_str()), text_style),
                    transform: Transform::from_translation(
                        Vec2::new(0., POPUP_STR_RESTART).extend(Z_POPUP + 0.01)
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
                GameOverPopupMessageDelay,
                Text2dBundle {
                    text: Text::from_section(
                        format!("Press [{}] to back to title.", GpKbInput::Select.get_str()), text_style),
                    transform: Transform::from_translation(
                        Vec2::new(0., POPUP_STR_GOTO_TITLE).extend(Z_POPUP + 0.01)
                    ),
                    visibility: Visibility::Hidden,
                    text_anchor: bevy::sprite::Anchor::Center,
                    ..default()
                },
            ));
            let text_style = TextStyle {
                font: my_assets.h_font.clone(),
                font_size: 24.0,
                color: my_assets.ui.popup.font_color_sub,
            };
            b.spawn((
                GameOverPopupMessageDelay,
                Text2dBundle {
                    text: Text::from_section(
                        format!("v{}, mode:{}", game_cnd.app_ver, game_cnd.mode), text_style),
                    transform: Transform::from_translation(
                        Vec2::new(0., POPUP_STR_5_1_Y).extend(Z_POPUP + 0.01)
                    ),
                    visibility: Visibility::Hidden,
                    text_anchor: bevy::sprite::Anchor::Center,
                    ..default()
                },
            ));
        });
    }
}

fn offset_to_go_to_inside(pos_cur: f32, half_w: f32, ball_pos: f32, ball_r: f32) -> f32 {
    if pos_cur + half_w < ball_pos + ball_r {
        pos_cur + (ball_pos + ball_r) - (pos_cur + half_w)
    } else if pos_cur - half_w > ball_pos - ball_r {
        pos_cur + (ball_pos - ball_r) - (pos_cur - half_w)
    } else {
        pos_cur
    }
}

const MARGIN_CAM_INSIDE: f32 = 30.;
pub fn move_camera_to_ball_protruded(
    mut commands: Commands,
    q_protruded: Query<(&Transform, &Ball), With<AreaProtruded>>,
    q_cam: Query<(Entity, &Transform), With<PlayCamera>>,
    window: Query<&Window>,
    asset: Res<GameAssets>,
) {
    let window = window.get_single()
        .expect("many window?");
    if let Ok((cam_entity, cam_trans)) = q_cam.get_single() {
        if let Ok((ball_trans, ball)) = q_protruded.get_single() {
            let ball_r = asset.get_ball_r(*ball.get_level());

            let x = offset_to_go_to_inside(
                cam_trans.translation.x,
                window.resolution.width() /2. - MARGIN_CAM_INSIDE,
                ball_trans.translation.x,
                ball_r,
            );
            let y = offset_to_go_to_inside(
                cam_trans.translation.y,
                window.resolution.height() /2. - MARGIN_CAM_INSIDE,
                ball_trans.translation.y,
                ball_r,
            );

            commands.entity(cam_entity)
                .insert(CameraMoving::move_to(Vec2::new(x, y), 1.0));

        } else {
            warn!("a ball has AreaProtruded is not found or is duplicated");
        }
    }
}

pub fn move_camera_to_default(
    mut commands: Commands,
    q_cam: Query<Entity, With<PlayCamera>>,
) {
    if let Ok(cam_entity) = q_cam.get_single() {
        commands.entity(cam_entity)
            .insert(CameraMoving::back_to_default_immediately());
    }
}


#[allow(clippy::complexity)]
pub fn cleanup_gameover_popup(
    mut commands: Commands,
    q_popup: Query<Entity,
        Or<(
            With<GameOverPopup>,
        )>
    >,
) {
    for p in q_popup.iter() {
        commands.entity(p)
            .despawn_recursive();
    }
}

pub fn update_gameover_popup(
    mut q_controller: Query<&mut ControllerGameOverPopup>,
    mut q_popup_message: Query<&mut Visibility,
        (With<GameOverPopupMessageDelay>, Without<ControllerGameOverPopup>)>,
    time: Res<Time>,
) {
    if let Ok(mut controller) = q_controller.get_single_mut() {
        controller.input_suppresser.tick(time.delta());
        if controller.input_suppresser.finished() {
            for mut vis_msg in q_popup_message.iter_mut() {
                *vis_msg = Visibility::Inherited;
            }
        }
    }
}

#[derive(Event, Debug)]
pub enum GameOverPopupInput {
    Restart,
    GoToTitle,
}

pub fn read_keyboard_for_gameover_popup(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut ev_input: EventWriter<GameOverPopupInput>,
) {
    if keyboard.any_just_pressed(KEYBOARD_KEYS_START) {
        ev_input.send(GameOverPopupInput::Restart);
    }
    if keyboard.any_just_pressed(KEYBOARD_KEYS_SELECT) {
        ev_input.send(GameOverPopupInput::GoToTitle);
    }
}

pub fn read_gamepad_for_gameover_popup(
    connected_gamepad: Option<Res<ConnectedGamePad>>,
    buttons: Res<ButtonInput<GamepadButton>>,
    mut ev_input: EventWriter<GameOverPopupInput>,
) {
    if let Some(&ConnectedGamePad(gamepad)) = connected_gamepad.as_deref() {

        if buttons.any_just_pressed(to_gamepad_btn(gamepad, &GAMEPAD_BTNS_START)) {
            ev_input.send(GameOverPopupInput::Restart);
        }
        if buttons.any_just_pressed(to_gamepad_btn(gamepad, &GAMEPAD_BTNS_SELECT)) {
            ev_input.send(GameOverPopupInput::GoToTitle);
        }
    }
}

pub fn act_gameover_popup(
    mut next_screen_state: ResMut<NextState<GameScreenState>>,
    mut next_state: ResMut<NextState<GameState>>,
    q_controller: Query<&ControllerGameOverPopup>,
    mut ev_input: EventReader<GameOverPopupInput>,
) {
    if let Ok(controller) = q_controller.get_single() {
        for ev in ev_input.read() {
            match ev {
                GameOverPopupInput::Restart => {
                    if controller.input_suppresser.finished() {
                        next_screen_state.set(GameScreenState::Init);
                        // FIXME: Should return?
                    }
                },
                GameOverPopupInput::GoToTitle => {
                    if controller.input_suppresser.finished() {
                        next_state.set(GameState::Title);
                    }
                },
            }
        }
    }
}
