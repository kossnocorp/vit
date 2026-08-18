use crate::prelude::*;

pub struct VitStateInitializing {
    pub dirs: VitDirs,
}

impl VitStateInitializing {
    pub fn create_state() -> VitState {
        match VitDirs::resolve() {
            Ok(dirs) => VitStateInitializing { dirs }.into(),

            Err(error) => VitStateErrored::create_error(error),
        }
    }

    pub async fn as_initialized_state(self, path: Option<&Path>) -> Result<VitState> {
        match VitPaths::resolve(path).await {
            Ok(paths) => match VitManifest::read_toml(&paths.manifest).await {
                Ok(manifest) => Ok(VitStateInitialized {
                    dirs: self.dirs,
                    paths,
                    manifest,
                }
                .into()),

                Err(err) => Ok(VitStateErrored::initialize_error(
                    self.dirs,
                    Some(paths),
                    err,
                )),
            },

            Err(err) => Ok(VitStateErrored::initialize_error(self.dirs, None, err)),
        }
    }
}

impl From<VitStateInitializing> for VitState {
    fn from(state: VitStateInitializing) -> VitState {
        VitState::Initializing(state)
    }
}
