use serde::{Deserialize, Serialize};
use bevy::prelude::*;

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
#[derive(Reflect)]
pub struct ListRonItem {
    pub name: String,
    pub file: String,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
#[derive(Reflect)]
#[derive(Asset)]
pub struct ListRon {
    pub list: Vec<ListRonItem>,
}

