use crate::prelude::*;

pub struct VitPaths {
    pub root: PathBuf,
    pub manifest: PathBuf,
    pub lock: PathBuf,
}

impl VitPaths {
    pub fn resolve(argument: Option<&Path>) -> Result<Self> {
        let manifest = match argument {
            None => PathBuf::from("vendor.toml"),
            Some(path) if path.is_dir() => path.join("vendor.toml"),
            Some(path) => {
                ensure!(
                    path.file_name().is_some_and(|name| name == "vendor.toml"),
                    "--manifest file must be named vendor.toml"
                );
                path.to_owned()
            }
        };
        let root = manifest
            .parent()
            .unwrap_or_else(|| Path::new(""))
            .to_owned();
        Ok(Self {
            lock: root.join("vendor.lock.toml"),
            root,
            manifest,
        })
    }

    pub fn target(&self, spec: &VitTarget) -> PathBuf {
        self.root
            .join("vendor")
            .join(format!("@{}", spec.owner))
            .join(&spec.repo)
            .join(&spec.path)
    }
}
