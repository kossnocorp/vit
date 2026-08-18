use crate::prelude::*;

impl VitVendor {
    pub async fn add(manifest_path: Option<&Path>, input: &str) -> Result<()> {
        let target = VitSourceInput::parse_target(input)?;
        let state = VitState::create().initialize(manifest_path).await?;

        let VitState::Initialized(state) = state else {
            bail!("Failed to initialize Vit state, expected initialized state");
        };

        let targets = state.manifest.targets()?;
        ensure!(
            !targets.contains_key(target.key()),
            "{} is already present in {}",
            target.key(),
            state.paths.manifest.display()
        );

        let mut state = state.as_locked().await?;

        let download = target.source().download(target.as_ref()).await?;
        let destination = state.paths.target(target.as_ref());
        let lock_entry = VitLockFile::new(target.as_ref(), &download, &state.paths);

        download.write(&destination).await?;

        state.manifest.add(target.key(), target.version());
        state.lock.files.insert(target.key().clone(), lock_entry);
        state.manifest.write_toml(&state.paths.manifest).await?;
        state.lock.write_toml(&state.paths.lock).await?;

        println!("Added {} to {}", target.key(), destination.display());
        Ok(())
    }
}
