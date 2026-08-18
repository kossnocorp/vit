use crate::prelude::*;

pub struct VitPaths {
    pub root: PathBuf,
    pub manifest: PathBuf,
    pub lock: PathBuf,
}

impl VitPaths {
    pub async fn resolve(path: Option<&Path>) -> Result<Self> {
        let manifest = Self::resolve_manifest_path(path)
            .await
            .with_context(|| "Failed to resolve Vit paths")?;

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

    async fn resolve_manifest_path(path: Option<&Path>) -> Result<PathBuf> {
        let path_buf = match path {
            None => PathBuf::from("vendor.toml"),

            Some(path) => {
                match tokio::fs::metadata(path).await {
                    Ok(metadata) if metadata.is_dir() => return Ok(path.join("vendor.toml")),
                    Ok(_) => {}
                    Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
                    Err(error) => {
                        return Err(error)
                            .with_context(|| format!("Failed to inspect {}", path.display()));
                    }
                }
                ensure!(
                    path.file_name().is_some_and(|name| name == "vendor.toml"),
                    "Manifest file name must be vendor.toml, got {path:?}",
                );
                path.to_owned()
            }
        };
        Ok(path_buf)
    }

    pub fn target(&self, target: &dyn VitTarget) -> PathBuf {
        self.root.join("vendor").join(target.vendor_path())
    }
}
