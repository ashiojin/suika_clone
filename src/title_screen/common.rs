use bevy::prelude::*;

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum TitleState {
    #[default]
    Loading,
    Idle,
    Config,
    End,
}

#[derive(Component, Debug)]
pub struct InTitleScreen;

