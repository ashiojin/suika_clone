use crate::prelude::*;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use bevy_egui_kbgp::KbgpEguiResponseExt;

use super::TitleAssets;
use super::TitleState;
use super::list_ron::*;

use crate::game_ron::get_default_game_ron_name_and_asset_path;


#[derive(Resource, Debug, Default)]
pub struct ConfigData {
    copy: Config,
    ron_selected: usize,
    ron_options: Vec<Option<ListRonItem>>,
}



pub fn prepare(
    mut config_data: ResMut<ConfigData>,
    config: Res<Config>,
    title_asset: Res<TitleAssets>,
    list_ron: Res<Assets<ListRon>>,
) {
    config_data.copy = config.clone();

    let list_ron = list_ron.get(title_asset.h_list_ron.id())
        .expect("list.ron is not loaded yet.");

    let mut options = vec![None];
    options.append(&mut list_ron.list.iter().cloned().map(Some).collect());

    config_data.ron_options = options;

    let (def_name, _) = get_default_game_ron_name_and_asset_path();

    config_data.ron_selected = if config.game_ron_name == def_name {
        0 // Default
    } else {
        config_data.ron_options.iter()
            .position(|o|
                  o.as_ref()
                      .map(|e| e.name == config.game_ron_name)
                      .unwrap_or(false))
            .unwrap_or(0) // Default
    }
}

const VOLUME_OPTIONS: [i32; 11] = [
    0, 10, 20, 30, 40, 50,
    60, 70, 80, 90, 100,
];

pub fn ui_popup(
    mut contexts: EguiContexts,
    mut config: ResMut<Config>,
    mut config_data: ResMut<ConfigData>,
    mut next_state: ResMut<NextState<TitleState>>,
) {
    let (def_ron_name, _) = get_default_game_ron_name_and_asset_path();
    let ctx = contexts.ctx_mut();
    let ron_options = config_data.ron_options.clone();
    egui::CentralPanel::default().show(ctx,
        |ui| {
            ui.heading("Game");
            for (idx, ron) in ron_options.iter().enumerate() {
                let name = if let Some(ListRonItem { name, ..}) = ron {
                    name
                } else {
                    def_ron_name
                };
                let name = if config_data.ron_selected == idx {
                    format!("* {}", name)
                } else {
                    name.to_string()
                };
                if ui.button(name)
                    .kbgp_navigation()
                    .clicked() {
                    config_data.ron_selected = idx;
                }
            }

            ui.heading("Sounds");
            ui.label(format!("BGM Volume: {}", config_data.copy.bgm_volume));
            ui.horizontal(|ui| {
                for vol in &VOLUME_OPTIONS {
                    if ui.button(format!("{}", vol))
                        .kbgp_navigation()
                        .clicked() {
                        config_data.copy.bgm_volume = *vol;
                    }
                }
            });
            ui.label(format!("SE Volume: {}", config_data.copy.se_volume));
            ui.horizontal(|ui| {
                for vol in &VOLUME_OPTIONS {
                    if ui.button(format!("{}", vol))
                        .kbgp_navigation()
                        .clicked() {
                        config_data.copy.se_volume = *vol;
                    }
                }
            });


            ui.separator();
            ui.horizontal(|ui| {
                if ui.button("Cancel")
                    .kbgp_navigation()
                    .clicked() {
                    next_state.set(TitleState::Idle);
                }

                if ui.button("Ok")
                    .kbgp_navigation()
                    .clicked() {
                    apply(&mut config, &config_data);
                    next_state.set(TitleState::Idle);
                }
            });

        }
    );
}


fn apply(
    config: &mut Config,
    config_data: &ConfigData,
) {
    *config = config_data.copy.clone();

    // read ron
    let idx = config_data.ron_selected;
    let ron = &config_data.ron_options[idx];

    let (name, file) = if let Some(ListRonItem{ name, path: file }) = ron {
        (name.as_str(), file.as_str())
    } else {
        get_default_game_ron_name_and_asset_path()
    };

    config.game_ron_name = name.to_string();
    config.game_ron_asset_path = file.to_string();
}
