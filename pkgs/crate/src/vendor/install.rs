use crate::prelude::*;

use tokio::io::AsyncReadExt;

impl VitVendor {
    pub async fn install(manifest_path: Option<&Path>, offline: bool) -> Result<()> {
        let state = VitState::create()
            .initialize(manifest_path)
            .await?
            .initialize_lock()
            .await?;

        let VitState::Locked(mut state) = state else {
            bail!("Failed to initialize Vit state, expected locked state");
        };

        let mut installed = 0;
        let targets = state.manifest.targets()?;

        for (key, target) in &targets {
            let version = target.version();
            let destination = state.paths.target(target.as_ref());
            let current = state.lock.files.get(key);
            let locked_destination = current
                .map(|entry| Self::lock_destination(&state.paths, &entry.path))
                .transpose()?;
            let is_current = if let Some(entry) = current {
                &entry.version == version
                    && locked_destination.as_ref() == Some(&destination)
                    && Self::file_matches(&destination, &entry.hash).await?
            } else {
                false
            };
            if is_current {
                continue;
            }

            ensure!(
                !offline,
                "{key} is not available from the current lock and vendor directory in offline mode"
            );
            let download = target.source().download(target.as_ref()).await?;
            let next = VitLockFile::new(target.as_ref(), &download, &state.paths);
            download.write(&destination).await?;
            if let Some(previous) = locked_destination.filter(|path| path != &destination) {
                Self::remove_locked_file(&previous, &state.paths.root.join("vendor")).await?;
            }
            state.lock.files.insert(key.clone(), next);
            installed += 1;
        }

        let stale = state
            .lock
            .files
            .keys()
            .filter(|key| !targets.contains_key(*key))
            .cloned()
            .collect::<Vec<_>>();
        for key in &stale {
            let entry = &state.lock.files[key];
            Self::remove_locked_file(
                &Self::lock_destination(&state.paths, &entry.path)?,
                &state.paths.root.join("vendor"),
            )
            .await?;
            state.lock.files.remove(key);
        }

        state.lock.write_toml(&state.paths.lock).await?;
        println!("Installed {installed} and removed {} files", stale.len());
        Ok(())
    }

    async fn file_matches(path: &Path, expected_hash: &str) -> Result<bool> {
        let mut file = match tokio::fs::File::open(path).await {
            Ok(file) => file,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(false),
            Err(error) => {
                return Err(error).with_context(|| format!("Failed to read {}", path.display()));
            }
        };
        let mut hasher = Sha256::new();
        let mut buffer = [0; 64 * 1024];
        loop {
            let read = file
                .read(&mut buffer)
                .await
                .with_context(|| format!("Failed to read {}", path.display()))?;
            if read == 0 {
                break;
            }
            hasher.update(&buffer[..read]);
        }
        Ok(format!("sha256:{:x}", hasher.finalize()) == expected_hash)
    }

    fn lock_destination(paths: &VitPaths, value: &str) -> Result<PathBuf> {
        let relative = Path::new(value);
        ensure!(
            relative
                .components()
                .all(|part| matches!(part, Component::Normal(_)))
                && relative.starts_with("vendor"),
            "Lockfile path {value:?} is not a safe vendor path"
        );
        Ok(paths.root.join(relative))
    }

    async fn remove_locked_file(path: &Path, vendor_root: &Path) -> Result<()> {
        match tokio::fs::remove_file(path).await {
            Ok(()) => {}
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => {
                return Err(error).with_context(|| format!("failed to remove {}", path.display()));
            }
        }

        let mut parent = path.parent();
        while let Some(directory) = parent.filter(|directory| directory.starts_with(vendor_root)) {
            match tokio::fs::remove_dir(directory).await {
                Ok(()) => parent = directory.parent(),
                Err(error)
                    if matches!(
                        error.kind(),
                        std::io::ErrorKind::NotFound | std::io::ErrorKind::DirectoryNotEmpty
                    ) =>
                {
                    break;
                }
                Err(error) => {
                    return Err(error)
                        .with_context(|| format!("Failed to remove {}", directory.display()));
                }
            }
        }
        Ok(())
    }
}
