use crate::prelude::*;
use bevy::prelude::*;

use bevy_rand::prelude::*;
use bevy_prng::ChaCha8Rng;
use rand_core::RngCore;

#[derive(Component, Debug)]
pub struct Player {
    pub speed: f32,
    pub next_ball_level: BallLevel,
    pub max_ball_level: BallLevel,

    pub hold_ball: Option<BallLevel>,

    pub can_drop: bool,

    pub score: u32,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            speed: 3.0,
            next_ball_level: default(),
            max_ball_level: default(),

            hold_ball: None,

            can_drop: true,

            score: 0,
        }
    }
}
impl Player {
    pub fn new(speed: f32, first_ball_level: BallLevel, max_ball_level: BallLevel) -> Self {
        Self {
            speed,
            next_ball_level: first_ball_level,
            max_ball_level,
            ..default()
        }
    }
    pub fn set_next_ball_level_from_rng(&mut self, rng: &mut EntropyComponent<ChaCha8Rng>) {

        self.next_ball_level = BallLevel::from_rand_u32(rng.next_u32(),
            BallLevel::new(BALL_LEVEL_MIN), self.max_ball_level);
    }
    pub fn is_fakeball_exists(&self) -> bool {
        self.can_drop
    }
}
