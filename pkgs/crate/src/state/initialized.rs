use crate::prelude::*;

pub struct VitStateInitialized {
    pub dirs: VitDirs,
    pub paths: VitPaths,
    pub manifest: VitManifest,
}

impl VitStateInitialized {
    pub async fn as_locked(self) -> Result<VitStateLocked> {
        let lock = VitLock::read_toml(&self.paths.lock).await?;
        Ok(VitStateLocked {
            dirs: self.dirs,
            paths: self.paths,
            manifest: self.manifest,
            lock,
        })
    }
}

impl From<VitStateInitialized> for VitState {
    fn from(val: VitStateInitialized) -> Self {
        VitState::Initialized(val)
    }
}
