use crate::prelude::*;

pub struct VitStateLocked {
    pub dirs: VitDirs,
    pub paths: VitPaths,
    pub manifest: VitManifest,
    pub lock: VitLock,
}

impl From<VitStateLocked> for VitState {
    fn from(state: VitStateLocked) -> VitState {
        VitState::Locked(state)
    }
}
