use crate::prelude::*;

mod file;
pub use file::*;

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct VitLock {
    #[serde(default)]
    pub files: BTreeMap<String, VitLockFile>,
}

impl VitFileToml for VitLock {}
