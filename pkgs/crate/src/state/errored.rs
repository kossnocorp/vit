use crate::prelude::*;

pub struct VitStateErrored {
    pub dirs: Option<VitDirs>,
    pub paths: Option<VitPaths>,
    pub error: Error,
}

impl VitStateErrored {
    pub fn create_error(err: Error) -> VitState {
        Self {
            dirs: None,
            paths: None,
            error: err.context("Failed to create Vit state"),
        }
        .into()
    }

    pub fn initialize_error(dirs: VitDirs, paths: Option<VitPaths>, err: Error) -> VitState {
        Self {
            dirs: Some(dirs),
            paths,
            error: err.context("Failed to initialize Vit state"),
        }
        .into()
    }
}

impl From<VitStateErrored> for VitState {
    fn from(val: VitStateErrored) -> Self {
        VitState::Errored(val)
    }
}
