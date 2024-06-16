use crate::prelude::*;
use bevy::prelude::*;
use bevy_rand::prelude::*;
use bevy_prng::ChaCha8Rng;
    use rand_core::RngCore;

#[derive(Component, Debug)]
pub struct Effect(Timer);

#[derive(Component, Debug)]
pub struct EffectAccelation(Vec2);

#[derive(Component, Debug)]
pub struct EffectVelocity(Vec2);

#[derive(Component, Debug)]
pub struct EffectRotation(f32);

#[derive(Component, Debug)]
pub struct EffectAlpha(effects::Linear<f32>);

#[derive(Component, Debug)]
pub struct EffectRed(effects::Linear<f32>);

#[derive(Component, Debug)]
pub struct EffectGreen(effects::Linear<f32>);

#[derive(Component, Debug)]
pub struct EffectBlue(effects::Linear<f32>);

#[derive(Resource, Debug)]
pub struct EffectManager {
    rand: EntropyComponent<ChaCha8Rng>,
}


fn make_scattering_effect(
    pos: Vec2,
    effect: &effects::Scattering,
    rnd: &mut EntropyComponent<ChaCha8Rng>,
) -> Vec<impl Bundle> {

    let num = effect.num.rand(rnd);
    (0..num).map(|_|{
        let l = effect.h_images.len();
        let idx = (rnd.next_u32() % l as u32) as usize;
        let h_img = effect.h_images[idx].clone();
        let img_scale = effect.image_scale;
        let time = effect.time.rand(rnd);
        let vel = effect.velocity.rand(rnd);
        let theta = effect.theta.rand(rnd);
        let init_rot = Quat::from_rotation_z(theta.to_radians());
        let init_velocity = init_rot.mul_vec3(Vec3::Y * vel).xy();
        let rot = effect.rotation.rand(rnd);
        let acc = effect.accelation.rand(rnd);
        let alpha = effect.alpha.clone();
        let red = effect.red.clone();
        let green = effect.green.clone();
        let blue = effect.blue.clone();
        let init_color = Color::rgba(
            red.get(0.),
            green.get(0.),
            blue.get(0.),
            alpha.get(0.),
        );
        (
            Effect(Timer::from_seconds(time, TimerMode::Once)),
            SpriteBundle {
                texture: h_img,
                sprite: Sprite {
                    color: init_color,
                    ..default()
                },
                transform: Transform::from_translation(pos.extend(Z_EFFECT))
                    .with_scale(Vec2::splat(img_scale).extend(1.)),
                ..default()
            },
            EffectVelocity(init_velocity),
            EffectRotation(rot),
            EffectAccelation(acc),
            EffectAlpha(alpha),
            EffectRed(red),
            EffectGreen(green),
            EffectBlue(blue),
        )
    }).collect()
}

pub fn spawn_effect_manager(
    mut commands: Commands,
    mut global_ent: ResMut<GlobalEntropy<ChaCha8Rng>>,
) {
    commands.insert_resource(
        EffectManager {
            rand: global_ent.fork_rng(),
        }
    );
}

pub fn spawn_effect(
    pos: Vec2,
    commands: &mut Commands,
    effect: &EffectDef,
    manager: &mut EffectManager,
) {
    let bundles = match effect {
        EffectDef::Scattering(s) => {
            make_scattering_effect(pos, s, &mut manager.rand)
        },
    };
    commands.spawn_batch(bundles);
}

#[allow(clippy::type_complexity)]
pub fn update_effect(
    mut commands: Commands,
    mut q_effects: Query<(
        Entity,
        &mut Sprite,
        &mut Transform,
        &mut Effect,
        &mut EffectVelocity,
        Option<&EffectAccelation>,
        Option<&EffectRotation>,
        Option<&EffectAlpha>,
        Option<&EffectRed>,
        Option<&EffectGreen>,
        Option<&EffectBlue>,
        )>,
    time: Res<Time>,
) {
    let delta = time.delta();
    let delta_second = time.delta_seconds();
    for (entity, mut sprite, mut trans, mut effect, mut velocity, accelation, rotation, alpha, red, green, blue) in q_effects.iter_mut() {
        effect.0.tick(delta);
        if effect.0.finished() {
            commands.entity(entity)
                .despawn_recursive();
        } else {
            let fraction = effect.0.fraction_remaining();
            if let Some(accelation) = accelation {
                velocity.0 += accelation.0 * delta_second;
            }
            let sec = effect.0.elapsed_secs();
            let rotation = if let Some(rotation) = rotation {
                rotation.0 * 360. * sec
            } else {
                0.
            }.to_radians();
            let alpha = if let Some(alpha) = alpha {
                alpha.0.get(fraction)
            } else {
                1.
            };
            let red = if let Some(red) = red {
                red.0.get(fraction)
            } else {
                1.
            };
            let green = if let Some(green) = green {
                green.0.get(fraction)
            } else {
                1.
            };
            let blue = if let Some(blue) = blue {
                blue.0.get(fraction)
            } else {
                1.
            };

            let cur_pos = trans.translation.xy();
            let next_pos = cur_pos + velocity.0;

            sprite.color = Color::rgba(red, green, blue, alpha);

            trans.translation.x = next_pos.x;
            trans.translation.y = next_pos.y;
            trans.rotation = Quat::from_rotation_z(rotation);
        }
    }
}
