use bevy::{
    prelude::*,
    render::camera::ScalingMode,
};

use crate::prelude::*;

#[derive(Component, Debug)]
pub struct PinnedToPlayingCamera(pub Vec2);

#[derive(Component, Debug)]
pub struct PlayingCamera;

#[derive(Component, Debug)]
pub struct DisabledByPlayingCamera;

/// Spawn PlayingCamera and Disable other camera
pub fn spawn_camera(
    mut commands: Commands,
    mut q_other: Query<(Entity, &mut Camera), Without<PlayingCamera>>,
    config: Res<FixedConfig>,
) {
    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.projection.scaling_mode = ScalingMode::FixedVertical(960.);
    camera_bundle.camera.order = CAM_ORDER_PLAYING;
    camera_bundle.transform.translation.x = config.playing_cam_offset.x;
    camera_bundle.transform.translation.y = config.playing_cam_offset.y;
    commands.spawn((
        PlayingCamera,
        camera_bundle,
    ));

    for (cam_e, mut cam) in q_other.iter_mut() {
        if cam.is_active {
            cam.is_active = false;

            commands.entity(cam_e)
                .insert(DisabledByPlayingCamera);
        }
    }
}

pub fn update_camera(
    mut q_cam: Query<&mut Transform, With<PlayingCamera>>,
    config: Res<FixedConfig>,
) {
    for mut t in q_cam.iter_mut() {
        t.translation.x = config.playing_cam_offset.x;
        t.translation.y = config.playing_cam_offset.y;
    }

}

#[allow(clippy::type_complexity)]
pub fn despawn_camera(
    mut commands: Commands,
    q_cam: Query<Entity, With<PlayingCamera>>,
    mut q_other: Query<(Entity, &mut Camera), (With<DisabledByPlayingCamera>, Without<PlayingCamera>)>,
) {
    for e in q_cam.iter() {
        commands.entity(e)
            .despawn_recursive();
    }

    for (cam_e, mut cam) in q_other.iter_mut() {
        cam.is_active = true;
        commands.entity(cam_e)
            .remove::<DisabledByPlayingCamera>();

    }
}

#[allow(clippy::type_complexity)]
pub fn update_pinned_to_camera(
    mut set: ParamSet<(
        Query<(&mut Transform, &PinnedToPlayingCamera)>,
        Query<&Transform, With<PlayingCamera>>,
    )>,
) {
    if let Ok(cam_xy) = set.p1().get_single().map(|t| t.translation.xy()) {
        for (mut trans, PinnedToPlayingCamera(offset)) in set.p0().iter_mut() {
            let xy = cam_xy + *offset;
            trans.translation.x = xy.x;
            trans.translation.y = xy.y;
        }
    }
}
