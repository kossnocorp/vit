use crate::prelude::*;

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct VitLockFile {
    pub version: String,
    pub revision: String,
    pub hash: String,
    pub source: String,
    pub path: String,
}

impl VitLockFile {
    pub fn new(target: &dyn VitTarget, download: &VitSourceFile, paths: &VitPaths) -> VitLockFile {
        let hash = Sha256::digest(&download.bytes);
        let path = paths.target(target);
        VitLockFile {
            version: target.version().to_owned(),
            revision: download.revision.clone(),
            hash: format!("sha256:{hash:x}"),
            source: target.source_url().to_owned(),
            path: path
                .strip_prefix(&paths.root)
                .unwrap_or(&path)
                .to_string_lossy()
                .into_owned(),
        }
    }
}
