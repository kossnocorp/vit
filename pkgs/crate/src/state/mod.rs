use crate::prelude::*;

mod initializing;
pub use initializing::*;

mod errored;
pub use errored::*;

mod initialized;
pub use initialized::*;

mod locked;
pub use locked::*;

pub enum VitState {
    Initializing(VitStateInitializing),
    Errored(VitStateErrored),
    Initialized(VitStateInitialized),
    Locked(VitStateLocked),
}

impl VitState {
    pub fn create() -> Self {
        VitStateInitializing::create_state()
    }

    pub async fn initialize(self, path: Option<&Path>) -> Result<VitState> {
        match self {
            VitState::Errored(_) => {
                // Nothing to do, already in errored state
                Ok(self)
            }

            VitState::Initializing(state) => state.as_initialized_state(path).await,

            VitState::Initialized(_) | VitState::Locked(_) => {
                bail!("Already initialized")
            }
        }
    }

    pub async fn initialize_lock(self) -> Result<VitState> {
        match self {
            VitState::Errored(_) => {
                // Nothing to do, already in errored state
                Ok(self)
            }

            VitState::Initializing(_) => {
                bail!("Cannot load lock while initializing")
            }

            VitState::Initialized(state) => Ok(state.as_locked().await?.into()),

            VitState::Locked(_) => {
                bail!("Already in locked state")
            }
        }
    }
}
