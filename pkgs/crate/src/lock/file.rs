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
    pub fn new(spec: &VitTarget, download: &VitDownload, paths: &VitPaths) -> VitLockFile {
        let hash = Sha256::digest(&download.bytes);
        let target = paths.target(spec);
        VitLockFile {
            version: spec.version.clone(),
            revision: download.revision.clone(),
            hash: format!("sha256:{hash:x}"),
            source: format!(
                "https://github.com/{}/{}/blob/{}/{}",
                spec.owner, spec.repo, spec.version, spec.path
            ),
            path: target
                .strip_prefix(&paths.root)
                .unwrap_or(&target)
                .to_string_lossy()
                .into_owned(),
        }
    }
}
