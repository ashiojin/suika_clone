use bevy::prelude::*;

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum TitleScreenState {
    #[default]
    Inactive,
    Loading,
    Idle,
    Config,
    End, // TODO: Is it necessary?
}

#[derive(Component, Debug)]
pub struct InTitleScreen;

