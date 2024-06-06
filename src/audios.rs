use bevy::{
    prelude::*,
    audio::Volume,
};


#[derive(Component, Debug)]
pub struct Bgm;

#[allow(clippy::type_complexity)]
pub fn force_single_bgm(
    mut commands: Commands,
    mut set: ParamSet<(
        Query<Entity, With<Bgm>>,
        Query<Entity, Added<Bgm>>,
    )>,
) {
    if let Ok(added_bgm) = set.p1().get_single() {
        set.p0().iter()
            .filter(|&e| e != added_bgm)
            .for_each(|e| {
                commands.entity(e)
                    .despawn_recursive();
            });
    }
}

pub fn stop_bgm(
    mut commands: Commands,
    mut q_bgm: Query<Entity, With<Bgm>>,
) {
    for bgm in q_bgm.iter_mut() {
        commands.entity(bgm).despawn_recursive();
    }
}

pub fn spawn_bgm(
    commands: &mut Commands,
    h_bgm: Handle<AudioSource>,
    volume: Volume,
) {
    commands.spawn((
        Bgm,
        AudioBundle {
            source: h_bgm,
            settings: PlaybackSettings {
                mode: bevy::audio::PlaybackMode::Loop,
                volume,
                ..default()
            },
        }
    ));
}

#[derive(Component, Debug)]
pub struct Se;

pub fn spawn_se(
    commands: &mut Commands,
    h_se: Handle<AudioSource>,
    volume: Volume,
) {
    commands.spawn((
        Se,
        AudioBundle {
            source: h_se,
            settings: PlaybackSettings {
                mode: bevy::audio::PlaybackMode::Despawn,
                volume,
                ..default()
            },
        },
    ));
}
