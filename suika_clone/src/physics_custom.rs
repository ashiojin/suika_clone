use crate::prelude::*;
use bevy::prelude::*;
use bevy_xpbd_2d::{
    prelude::*,
    SubstepSchedule,
    SubstepSet,
};

pub struct LimitVelocityPlugin;

impl Plugin for LimitVelocityPlugin {
    fn build(&self, app: &mut App) {
        let substep_schedule = app.get_schedule_mut(SubstepSchedule)
            .expect("Add SubstepSchedule first");
        substep_schedule
            .add_systems(
                limit_velocity_of_ball
                    .after(SubstepSet::SolveVelocities)
                    .before(SubstepSet::StoreImpulses)
            );
    }
}

fn limit_velocity_of_ball(
    mut q_ball: Query<&mut LinearVelocity>,
    config: Res<FixedConfig>,
) {
    let max = config.max_velocity;
    let max_sq = max * max;
    for mut vel in q_ball.iter_mut() {
        let l_sq = vel.length_squared();
        if l_sq > max_sq {
            let l = l_sq.sqrt();
            *vel = (vel.0 / l * max).into();

            info!("limit! {} <= {}", l, max);
        }
    }
}

