use crate::prelude::*;

use std::fs;
use tempfile::NamedTempFile;

pub async fn add(manifest_arg: Option<&Path>, input: &str) -> Result<()> {
    let paths = VitPaths::resolve(manifest_arg)?;
    let spec = VitTarget::parse(input)?;
    let mut manifest: VitManifest = read_toml(&paths.manifest)?;
    ensure!(
        !manifest.has(&spec.key),
        "{} is already present in {}",
        spec.key,
        paths.manifest.display()
    );

    let mut lock: VitLock = read_toml(&paths.lock)?;
    let download = VitDownload::fetch(&spec).await?;
    let target = paths.target(&spec);
    let lock_entry = VitLockFile::new(&spec, &download, &paths);

    atomic_write(&target, &download.bytes)?;
    manifest.add(spec.key.clone(), spec.version.clone());
    lock.files.insert(spec.key.clone(), lock_entry);
    write_toml(&paths.manifest, &manifest)?;
    write_toml(&paths.lock, &lock)?;

    println!("Added {} to {}", spec.key, target.display());
    Ok(())
}

pub async fn update(manifest_arg: Option<&Path>, input: &str) -> Result<()> {
    let paths = VitPaths::resolve(manifest_arg)?;
    let spec = VitTarget::parse(input)?;
    let manifest: VitManifest = read_toml(&paths.manifest)?;
    let version = manifest.get(&spec.key).with_context(|| {
        format!(
            "{} is not present in {}",
            spec.key,
            paths.manifest.display()
        )
    })?;
    ensure!(
        version == &spec.version,
        "{} uses version {:?} in {}, not {:?}",
        spec.key,
        version,
        paths.manifest.display(),
        spec.version
    );

    let mut lock: VitLock = read_toml(&paths.lock)?;
    let download = VitDownload::fetch(&spec).await?;
    let target = paths.target(&spec);
    let next = VitLockFile::new(&spec, &download, &paths);
    let changed = lock.files.get(&spec.key) != Some(&next)
        || fs::read(&target).ok().as_deref() != Some(&download.bytes);

    if changed {
        atomic_write(&target, &download.bytes)?;
        lock.files.insert(spec.key.clone(), next);
        write_toml(&paths.lock, &lock)?;
        println!("Updated {}", spec.key);
    } else {
        println!("{} is already up to date", spec.key);
    }
    Ok(())
}

fn read_toml<T>(path: &Path) -> Result<T>
where
    T: for<'de> Deserialize<'de> + Default,
{
    match fs::read_to_string(path) {
        Ok(source) => {
            toml::from_str(&source).with_context(|| format!("failed to parse {}", path.display()))
        }

        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(T::default()),

        Err(error) => Err(error).with_context(|| format!("failed to read {}", path.display())),
    }
}

fn write_toml<T: Serialize>(path: &Path, value: &T) -> Result<()> {
    let mut source = toml::to_string_pretty(value).context("failed to serialize TOML")?;
    if !source.ends_with('\n') {
        source.push('\n');
    }
    atomic_write(path, source.as_bytes())
}

fn atomic_write(path: &Path, bytes: &[u8]) -> Result<()> {
    let parent = path.parent().unwrap_or_else(|| Path::new("."));
    fs::create_dir_all(parent).with_context(|| format!("failed to create {}", parent.display()))?;
    let mut temp = NamedTempFile::new_in(parent)
        .with_context(|| format!("failed to create temporary file in {}", parent.display()))?;
    use std::io::Write;
    temp.write_all(bytes)
        .with_context(|| format!("failed to write {}", path.display()))?;
    temp.persist(path)
        .map_err(|error| error.error)
        .with_context(|| format!("failed to replace {}", path.display()))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolves_manifest_and_target_paths() {
        let directory = tempfile::tempdir().unwrap();
        let paths = VitPaths::resolve(Some(directory.path())).unwrap();
        let spec = VitTarget::parse("js-fns/js-fns/src/file.ts@main").unwrap();
        assert_eq!(paths.manifest, directory.path().join("vendor.toml"));
        assert_eq!(
            paths.target(&spec),
            directory.path().join("vendor/@js-fns/js-fns/src/file.ts")
        );
        assert!(VitPaths::resolve(Some(Path::new("other.toml"))).is_err());
    }
}
