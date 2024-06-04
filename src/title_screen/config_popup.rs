use crate::prelude::*;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use super::TitleAssets;
use super::TitleState;
use super::list_ron::*;

use crate::game_ron::get_default_game_ron_name_and_file_name;


#[derive(Resource, Debug, Default)]
pub struct ConfigData {
    copy: Config,
    ron_selected: Option<usize>,
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

}


pub fn ui_popup(
    mut contexts: EguiContexts,
    mut config: ResMut<Config>,
    mut config_data: ResMut<ConfigData>,
    mut next_state: ResMut<NextState<TitleState>>,
) {
    let ctx = contexts.ctx_mut();
    let ron_options = config_data.ron_options.clone();
    egui::Window::new("Config").show(
        ctx,
        |ui| {
            ui.add(
                egui::Slider::new(&mut config_data.copy.bgm_volume, 0..=100)
                    .text("BGM Volume")
            );
            ui.add(
                egui::Slider::new(&mut config_data.copy.se_volume, 0..=100)
                    .text("SE Volume")
            );

            for (idx, ron) in ron_options.iter().enumerate() {
                let name = if let Some(ListRonItem { name, ..}) = ron {
                    name
                } else {
                    "(default)"
                };
                if ui.button(name).clicked() {
                    config_data.ron_selected = Some(idx);
                }
            }

            if ui.button("Cancel")
                .clicked() {
                next_state.set(TitleState::Idle);
            }

            if ui.button("Ok")
                .clicked() {
                apply(&mut config, &config_data);
                next_state.set(TitleState::Idle);
            }
        }
    );
}


fn apply(
    config: &mut Config,
    config_data: &ConfigData,
) {
    info!("{:?}", config_data);
    *config = config_data.copy.clone();

    // read ron
    if let Some(idx) = config_data.ron_selected {
        info!("1");
        let ron = &config_data.ron_options[idx];

        let (name, file) = if let Some(ListRonItem{ name, file }) = ron {
            (name.as_str(), file.as_str())
        } else {
            info!("3");
            get_default_game_ron_name_and_file_name()
        };

        config.game_ron_name = name.to_string();
        config.game_ron_file_name = file.to_string();
    }
}
