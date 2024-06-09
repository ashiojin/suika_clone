use bevy::prelude::*;
use bevy_egui::{
    egui,
    EguiContexts,
    EguiPlugin,
};
use bevy_file_dialog::prelude::*;

struct FontFileContents;

fn main() {
    #[cfg(target_family = "windows")]
    std::env::set_var("RUST_BACKTRACE", "1"); // Can't read env values when running on WSL

    let mut app = App::new();

    app.add_plugins((
        DefaultPlugins,
        EguiPlugin,
        FileDialogPlugin::new()
            .with_load_file::<FontFileContents>(),
    ));

    app.add_event::<ViewEvent>();
    app.insert_state(ViewerState::Idle);
    app.add_systems(Startup, (
        setup,
    ));

    app.add_systems(Update, (
        file_loaded,
        spawn_font_table,
        ui,
        read_key,
    ));

    app.run();
}

fn read_key(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut view: ResMut<View>,
    mut ev_view: EventWriter<ViewEvent>,
) {
    let mut addr_d = 0i32;
    if keyboard.just_pressed(KeyCode::ArrowUp) {
        addr_d -= 0x10;
    }
    if keyboard.just_pressed(KeyCode::ArrowDown) {
        addr_d += 0x10;
    }
    if keyboard.just_pressed(KeyCode::ArrowLeft) {
        addr_d -= 0x10 * view.len_x16 as i32;
    }
    if keyboard.just_pressed(KeyCode::ArrowRight) {
        addr_d += 0x10 * view.len_x16 as i32;
    }
    if keyboard.just_pressed(KeyCode::KeyU) {
        addr_d -= 0x100 * view.len_x16 as i32;
    }
    if keyboard.just_pressed(KeyCode::KeyD) {
        addr_d += 0x100 * view.len_x16 as i32;
    }

    if addr_d != 0 {
        view.start_addr =
            (view.start_addr as i32 + addr_d).clamp(0x0, 0x110000) as u32;
        ev_view.send(ViewEvent::Reflesh);
    }
}

fn setup(
    mut commands: Commands,
    mut ev_view: EventWriter<ViewEvent>,
) {
    commands.insert_resource(
        View {
            h_font: None,
            start_addr: 0xE000,
            len_x16: 5,
            weight: 32.0,
        }
    );

    commands.spawn(
        Camera2dBundle::default(),
    );

    ev_view.send(ViewEvent::Reflesh);
}

#[derive(Event, Debug)]
enum ViewEvent {
    Reflesh,
}

#[derive(States, Default, Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum ViewerState {
    #[default]
    Idle,
    _Reset,
}

#[derive(Component, Debug)]
struct ViewTag;

fn spawn_font_table(
    mut commands: Commands,
    mut ev_view: EventReader<ViewEvent>,
    q_view: Query<Entity, With<ViewTag>>,
    view: Res<View>,
) {
    if !ev_view.read().any(|ev| matches!(ev, ViewEvent::Reflesh)) {
        return;
    }
    let weight = view.weight;
    let boxsize = Vec2::new(weight, weight);
    let margin = 4.;
    let char_x_num = 0x10;
    let label_size = weight * 4.;
    let background_size = Vec2::new(
        boxsize.x + (char_x_num as f32 -1.) * (boxsize.x + margin),
        boxsize.y + (view.len_x16 as f32 -1.) * (boxsize.y + margin)
    );
    let background_offset = Vec2::new(
        background_size.x / 2. + label_size,
        -(background_size.y / 2. + (boxsize.y + margin)/*label*/),
    );
    let whole_size = background_size + Vec2::X * label_size;
    let text_style = if let Some(h) = view.h_font.clone() {
        TextStyle {
            font: h.clone(),
            font_size: weight,
            ..default()
        }
    } else {
        TextStyle {
            font_size: weight,
            ..default()
        }
    };

    for e in q_view.iter() {
        commands.entity(e)
            .despawn_recursive();
    }

    commands.spawn((
        ViewTag,
        SpatialBundle {
            transform: Transform::from_translation(
               Vec2::new(
                   -whole_size.x / 2.,
                   whole_size.y / 2.,
               ).extend(0.)
            ),

            ..default()
        },
    ))
    .with_children(|b| {


        let text_style_label = TextStyle {
            font_size: weight,
            ..default()
        };

        for (idx, ch) in (0..0x10).enumerate() {

            let x = label_size + (boxsize.x + margin) * idx as f32 +weight/2.;

            b.spawn((
                Text2dBundle {
                    text: Text::from_section(format!("{:x}", ch), text_style.clone()),
                    transform: Transform::from_translation(
                        Vec2::new(x, 0.).extend(0.1)),
                    ..default()
                },
            ));
        }

        for idx_x16 in 0..view.len_x16 {
            let y = 0. - (boxsize.y + margin) * (idx_x16 + 1/*label*/) as f32 -weight/2.;
            let addr = view.start_addr + idx_x16 as u32 * char_x_num;

            let mut chars = vec![];
            for addr in addr..(addr+char_x_num) {
                chars.push(char::from_u32(addr));
            }

            b.spawn((
                Text2dBundle {
                    text: Text::from_section(format!("U+{:>X}", addr), text_style_label.clone()),
                    transform: Transform::from_translation(
                        Vec2::new(0., y).extend(0.1)),
                    text_anchor: bevy::sprite::Anchor::CenterLeft,
                    ..default()
                },
            ));
            for (idx, ch) in chars.iter().enumerate() {

                let x = label_size + (boxsize.x + margin) * idx as f32 +weight/2.;

                if let Some(ch) = ch {
                    b.spawn((
                        SpriteBundle {
                            sprite: Sprite {
                                color: Color::BLACK,
                                custom_size: Some(boxsize),
                                ..default()
                            },
                            transform: Transform::from_translation(Vec2::new(x, y).extend(0.1)),
                            ..default()
                        },
                    ))
                    .with_children(|b| {
                        b.spawn((
                            Text2dBundle {
                                text: Text::from_section(ch.to_string(), text_style.clone()),
                                transform: Transform::from_translation(
                                    Vec2::new(0., 0.).extend(0.1)),
                                ..default()
                            },
                        ));
                    });
                }
            }
        }

        b.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::GRAY,
                    custom_size: Some(background_size),
                    ..default()
                },
                transform: Transform::from_translation(background_offset.extend(0.0)),
                ..default()
            },
        ));

    });
}

#[derive(Resource, Debug)]
struct View {
    h_font: Option<Handle<Font>>,
    start_addr: u32,
    len_x16: usize,
    weight: f32,
}

fn ui(
    mut commands: Commands,
    mut contexts: EguiContexts,
    mut view: ResMut<View>,
    mut ev_view: EventWriter<ViewEvent>,
) {
    let ctx = contexts.ctx_mut();
    egui::Window::new("Config").show(ctx, |ui| {

        if ui.button("File")
            .clicked() {
            let dir = std::env::current_dir();
            let mut dialog = commands.dialog()
                .add_filter("ttf", &["ttf"]);
            if let Ok(dir) = dir {
                dialog = dialog.set_directory(dir);
            }
            dialog.load_file::<FontFileContents>();
        }

        let res_1 = ui.add(
            egui::Slider::new(&mut view.start_addr, 0x0..=0x10FFFF)
                .text("Addr")
                .hexadecimal(4, false, true)
        );
        let res_2 = ui.add(
            egui::Slider::new(&mut view.len_x16, 0x0..=0x10).text("Len(x16 bytes)")
        );
        let res_3 = ui.add(
            egui::Slider::new(&mut view.weight, 1.0..=128.0f32).text("Weight")
        );

        if res_1.changed() || res_2.lost_focus() || res_3.changed() {
            ev_view.send(ViewEvent::Reflesh);
        }
    });
}

fn file_loaded(
    mut ev_loaded: EventReader<DialogFileLoaded<FontFileContents>>,
    mut view: ResMut<View>,
    mut fonts: ResMut<Assets<Font>>,
    mut ev_view: EventWriter<ViewEvent>,
) {
    for ev in ev_loaded.read() {
        let contents = ev.contents.clone();
        let font = Font::try_from_bytes(contents);
        if let Ok(font) = font {
            let h_font = fonts.add(font);
            view.h_font = Some(h_font);
            ev_view.send(ViewEvent::Reflesh);
        }
    }
}
