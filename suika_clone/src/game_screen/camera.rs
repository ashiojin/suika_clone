use bevy::{
    prelude::*,
    render::camera::ScalingMode,
};

use crate::prelude::*;

#[derive(Component, Debug)]
pub struct PinnedToPlayingCamera(pub Vec2);

#[derive(Component, Debug)]
pub struct PlayCamera {
    set_pos: Vec2,
}
impl PlayCamera {
    fn new(pos: Vec2) -> Self {
        Self {
            set_pos: pos,
        }
    }
}
#[derive(Component, Debug)]
pub struct CameraMoving {
    from: Option<Vec2>,
    to: Option<Vec2>,
    timer: Timer,
    auto_remove: bool,
}
impl CameraMoving {
    fn new(from: Option<Vec2>, to: Option<Vec2>, sec: f32, auto_remove: bool) -> Self {
        Self {
            from,
            to,
            timer: Timer::from_seconds(sec, TimerMode::Once),
            auto_remove,
        }
    }

    pub fn move_to(to: Vec2, sec: f32) -> Self {
        Self::new(None, Some(to), sec, false)
    }
    pub fn back_to_default(sec: f32) -> Self {
        Self::new(None, None, sec, true)
    }
    pub fn back_to_default_immediately() -> Self {
        Self::back_to_default(0.)
    }
}

#[derive(Component, Debug)]
pub struct DisabledByPlayingCamera;

/// Spawn PlayingCamera and Disable other camera
pub fn spawn_camera(
    mut commands: Commands,
    mut q_other: Query<(Entity, &mut Camera), Without<PlayCamera>>,
    config: Res<FixedConfig>,
) {
    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.projection.scaling_mode = ScalingMode::FixedVertical(960.);
    camera_bundle.camera.order = CAM_ORDER_PLAYING;
    camera_bundle.transform.translation.x = config.playing_cam_offset.x;
    camera_bundle.transform.translation.y = config.playing_cam_offset.y;
    commands.spawn((
        PlayCamera::new(config.playing_cam_offset),
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
    mut commands: Commands,
    mut q_cam: Query<(Entity, &mut Transform, Option<&mut CameraMoving>, &PlayCamera), With<PlayCamera>>,
    time: Res<Time>,
) {
    for (entity, mut t, mut mov, cam) in q_cam.iter_mut() {
        if let Some(mov) = mov.as_deref_mut() {
            if !mov.timer.finished() {
                mov.timer.tick(time.delta());
                let frac = mov.timer.fraction();

                if mov.from.is_none() {
                    mov.from = Some(t.translation.xy());
                }
                if mov.to.is_none() {
                    mov.to = Some(cam.set_pos);
                }

                let v = mov.from.unwrap().lerp(mov.to.unwrap(), frac);

                t.translation.x = v.x;
                t.translation.y = v.y;
                if mov.timer.finished() && mov.auto_remove {
                    commands.entity(entity)
                        .remove::<CameraMoving>();
                }
            }
        } else if t.translation.xy() != cam.set_pos {
            t.translation.x = cam.set_pos.x;
            t.translation.y = cam.set_pos.y;
        } else {
            // Do nothing
        }
    }

}

#[allow(clippy::type_complexity)]
pub fn despawn_camera(
    mut commands: Commands,
    q_cam: Query<Entity, With<PlayCamera>>,
    mut q_other: Query<(Entity, &mut Camera), (With<DisabledByPlayingCamera>, Without<PlayCamera>)>,
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
        Query<&Transform, With<PlayCamera>>,
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
