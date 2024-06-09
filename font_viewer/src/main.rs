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
    ));

    app.run();
}

fn setup(
    mut commands: Commands,
) {
    commands.insert_resource(
        View {
            h_font: None,
            start_addr: 0xE000,
            len_x16: 5,
        }
    );

    commands.spawn(
        Camera2dBundle::default(),
    );
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
    let h_font = if let Some(h) = view.h_font.clone() {
        h
    } else {
        return;
    };
    for e in q_view.iter() {
        commands.entity(e)
            .despawn_recursive();
    }
    commands.spawn((
        ViewTag,
        SpatialBundle {
            ..default()
        },
    ))
    .with_children(|b| {

        let weight = 32.0;
        let boxsize = Vec2::new(weight, weight);
        let margin = 4.;

        let text_style = TextStyle {
            font: h_font.clone(),
            font_size: weight,
            ..default()
        };

        for idx_x16 in 0..view.len_x16 {
            let y = 0. - (boxsize.y + margin) * idx_x16 as f32 -weight/2.;
            let addr = view.start_addr + idx_x16 as u32 * 0x10;

            let mut chars = vec![];
            for addr in addr..(addr+0x10) {
                chars.push(char::from_u32(addr));
            }

            for (idx, ch) in chars.iter().enumerate() {
                let x = 0. + (boxsize.x + margin) * idx as f32 +weight/2.;

                if let Some(ch) = ch {
                    b.spawn((
                        Text2dBundle {
                            text: Text::from_section(ch.to_string(), text_style.clone()),
                            transform: Transform::from_translation(
                                Vec2::new(x, y).extend(0.1)),
                            ..default()
                        },
                    ));
                }
            }
        }

        let background_size = Vec2::new(
                        boxsize.x + (0x10 as f32 -1.) * (boxsize.x + margin),
                        boxsize.y + (view.len_x16 as f32 -1.) * (boxsize.y + margin));
        let background_offset = Vec2::new(
            background_size.x / 2.,
            -background_size.y / 2.,
        );
        b.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::BLACK,
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
}

fn ui(
    mut commands: Commands,
    mut contexts: EguiContexts,
//    mut view: ResMut<View>,
//    fonts: Res<Assets<Font>>,
//    folder: Res<Assets<LoadedFolder>>,
//    mut ev_view: EventWriter<ViewEvent>,
//
) {
    let ctx = contexts.ctx_mut();
//    let folder = folder.get(view.h_foler.id());
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
        /*
        if let Some(foler) = folder {
            for h_font in foler.handles.iter() {
                let h_font = h_font.clone().typed::<Font>();
                let font = fonts.get(h_font.id());
                if let Some(_font) = font {
                    if ui.button("!").clicked() {
                        view.h_font = Some(h_font);
                        ev_view.send(ViewEvent::Reflesh);
                    }
                }
            }
        }
        */
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
