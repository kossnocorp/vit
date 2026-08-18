use crate::prelude::*;

mod file;
pub use file::*;

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct VitLock {
    #[serde(default)]
    pub files: BTreeMap<VitManifestTargetUrl, VitLockFile>,
}

impl VitFileToml for VitLock {}
