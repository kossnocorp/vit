use crate::prelude::*;

impl VitVendor {
    pub async fn update(manifest_path: Option<&Path>, input: &str) -> Result<()> {
        let target = VitSourceInput::parse_target(input)?;
        let state = VitState::create().initialize(manifest_path).await?;

        let VitState::Initialized(state) = state else {
            bail!("Failed to initialize Vit state, expected initialized state");
        };

        let targets = state.manifest.targets()?;
        let manifest_target = targets.get(target.key()).with_context(|| {
            format!(
                "{} is not present in {}",
                target.key(),
                state.paths.manifest.display()
            )
        })?;
        let version = manifest_target.version();

        ensure!(
            version == target.version(),
            "{} uses version {:?} in {}, not {:?}",
            target.key(),
            version,
            state.paths.manifest.display(),
            target.version()
        );

        let mut state = state.as_locked().await?;

        let download = target.source().download(target.as_ref()).await?;
        let destination = state.paths.target(target.as_ref());
        let next = VitLockFile::new(target.as_ref(), &download, &state.paths);
        let current_bytes = match tokio::fs::read(&destination).await {
            Ok(bytes) => Some(bytes),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => None,
            Err(error) => {
                return Err(error)
                    .with_context(|| format!("Failed to read {}", destination.display()));
            }
        };
        let changed = state.lock.files.get(target.key()) != Some(&next)
            || current_bytes.as_deref() != Some(&download.bytes);

        if changed {
            download.write(&destination).await?;
            state.lock.files.insert(target.key().clone(), next);
            state.lock.write_toml(&state.paths.lock).await?;
            println!("Updated {}", target.key());
        } else {
            println!("{} is already up to date", target.key());
        }
        Ok(())
    }
}
