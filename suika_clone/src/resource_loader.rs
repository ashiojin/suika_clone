use bevy::{
    prelude::*,
    asset::LoadState,
};

pub trait Loadable : Resource{
    fn get_untyped_handles(&self) -> Vec<UntypedHandle>;

    fn get_loading_state(&self, asset_server: &AssetServer) -> LoadingState {
        self.get_untyped_handles()
        .iter()
        .map(|h| asset_server.get_load_states(h.id()))
        .filter_map(|s| s.map(|(s, _, _)| s))
        .fold(LoadingState::Completed, |a, s| {
            let s = match s {
                LoadState::Loaded => LoadingState::Completed,
                LoadState::Failed => LoadingState::Error,
                _ => LoadingState::Loading,
            };
            LoadingState::max(a, s)
        })
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum LoadingState {
    Completed,
    Loading,
    Error,
}

