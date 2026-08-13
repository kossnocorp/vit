use anyhow::{Context, Result, ensure};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::fs;
use std::path::{Component, Path, PathBuf};
use tempfile::NamedTempFile;

#[derive(Debug, Deserialize, Serialize, Default)]
struct VitManifest {
    #[serde(default)]
    files: BTreeMap<String, String>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
struct VitLock {
    #[serde(default)]
    files: BTreeMap<String, VitLockFile>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
struct VitLockFile {
    version: String,
    revision: String,
    hash: String,
    source: String,
    path: String,
}

#[derive(Debug, PartialEq)]
struct VitSpec {
    key: String,
    owner: String,
    repo: String,
    path: String,
    version: String,
}

#[derive(Deserialize)]
struct VitCommit {
    sha: String,
}

struct VitDownload {
    revision: String,
    bytes: Vec<u8>,
}

pub async fn add(manifest_arg: Option<&Path>, input: &str) -> Result<()> {
    let paths = VitPaths::resolve(manifest_arg)?;
    let spec = VitSpec::parse(input)?;
    let mut manifest: VitManifest = read_toml(&paths.manifest)?;
    ensure!(
        !manifest.files.contains_key(&spec.key),
        "{} is already present in {}",
        spec.key,
        paths.manifest.display()
    );

    let mut lock: VitLock = read_toml(&paths.lock)?;
    let download = fetch(&spec).await?;
    let target = paths.target(&spec);
    let lock_entry = lock_entry(&spec, &download, &paths);

    atomic_write(&target, &download.bytes)?;
    manifest
        .files
        .insert(spec.key.clone(), spec.version.clone());
    lock.files.insert(spec.key.clone(), lock_entry);
    write_toml(&paths.manifest, &manifest)?;
    write_toml(&paths.lock, &lock)?;

    println!("Added {} to {}", spec.key, target.display());
    Ok(())
}

pub async fn update(manifest_arg: Option<&Path>, input: &str) -> Result<()> {
    let paths = VitPaths::resolve(manifest_arg)?;
    let spec = VitSpec::parse(input)?;
    let manifest: VitManifest = read_toml(&paths.manifest)?;
    let version = manifest.files.get(&spec.key).with_context(|| {
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
    let download = fetch(&spec).await?;
    let target = paths.target(&spec);
    let next = lock_entry(&spec, &download, &paths);
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

impl VitSpec {
    fn parse(input: &str) -> Result<Self> {
        let (source, version) = input.rsplit_once('@').with_context(|| {
            format!("invalid file spec {input:?}; expected owner/repo/path@version")
        })?;
        ensure!(!version.is_empty(), "version must not be empty");

        let mut parts = source.splitn(3, '/');
        let owner = parts.next().unwrap_or_default();
        let repo = parts.next().unwrap_or_default();
        let path = parts.next().unwrap_or_default();
        ensure!(!owner.is_empty(), "repository owner must not be empty");
        ensure!(!repo.is_empty(), "repository name must not be empty");
        ensure!(!path.is_empty(), "repository file path must not be empty");
        ensure!(
            Path::new(path)
                .components()
                .all(|part| matches!(part, Component::Normal(_))),
            "repository file path must not contain absolute or traversal components"
        );
        ensure!(
            !version.contains('/') && version != "." && version != "..",
            "version must be a simple Git ref"
        );

        Ok(Self {
            key: source.to_owned(),
            owner: owner.to_owned(),
            repo: repo.to_owned(),
            path: path.to_owned(),
            version: version.to_owned(),
        })
    }
}

struct VitPaths {
    root: PathBuf,
    manifest: PathBuf,
    lock: PathBuf,
}

impl VitPaths {
    fn resolve(argument: Option<&Path>) -> Result<Self> {
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

    fn target(&self, spec: &VitSpec) -> PathBuf {
        self.root
            .join("vendor")
            .join(format!("@{}", spec.owner))
            .join(&spec.repo)
            .join(&spec.path)
    }
}

async fn fetch(spec: &VitSpec) -> Result<VitDownload> {
    let client = Client::builder()
        .user_agent("vendorit/0.1")
        .build()
        .context("failed to create HTTP client")?;
    let commit_url = format!(
        "https://api.github.com/repos/{}/{}/commits/{}",
        spec.owner, spec.repo, spec.version
    );
    let response = client
        .get(&commit_url)
        .send()
        .await
        .with_context(|| format!("failed to resolve {}", spec.version))?
        .error_for_status()
        .with_context(|| format!("GitHub could not resolve {}", spec.version))?;
    let commit: VitCommit = response
        .json()
        .await
        .context("GitHub returned an invalid commit response")?;
    let raw_url = format!(
        "https://raw.githubusercontent.com/{}/{}/{}/{}",
        spec.owner, spec.repo, commit.sha, spec.path
    );
    let bytes = client
        .get(&raw_url)
        .send()
        .await
        .with_context(|| format!("failed to fetch {}", spec.path))?
        .error_for_status()
        .with_context(|| format!("GitHub could not fetch {}", spec.path))?
        .bytes()
        .await
        .context("failed to read downloaded file")?
        .to_vec();
    Ok(VitDownload {
        revision: commit.sha,
        bytes,
    })
}

fn lock_entry(spec: &VitSpec, download: &VitDownload, paths: &VitPaths) -> VitLockFile {
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
    fn parses_supported_spec() {
        assert_eq!(
            VitSpec::parse("js-fns/js-fns/vitest.config.ts@main").unwrap(),
            VitSpec {
                key: "js-fns/js-fns/vitest.config.ts".into(),
                owner: "js-fns".into(),
                repo: "js-fns".into(),
                path: "vitest.config.ts".into(),
                version: "main".into(),
            }
        );
    }

    #[test]
    fn rejects_unsafe_specs() {
        assert!(VitSpec::parse("js-fns/js-fns/../secret@main").is_err());
        assert!(VitSpec::parse("js-fns/js-fns/file@feature/ref").is_err());
        assert!(VitSpec::parse("js-fns//file@main").is_err());
    }

    #[test]
    fn resolves_manifest_and_target_paths() {
        let directory = tempfile::tempdir().unwrap();
        let paths = VitPaths::resolve(Some(directory.path())).unwrap();
        let spec = VitSpec::parse("js-fns/js-fns/src/file.ts@main").unwrap();
        assert_eq!(paths.manifest, directory.path().join("vendor.toml"));
        assert_eq!(
            paths.target(&spec),
            directory.path().join("vendor/@js-fns/js-fns/src/file.ts")
        );
        assert!(VitPaths::resolve(Some(Path::new("other.toml"))).is_err());
    }

    #[test]
    fn serializes_expected_toml_shapes() {
        let mut manifest = VitManifest::default();
        manifest
            .files
            .insert("js-fns/js-fns/vitest.config.ts".into(), "main".into());
        let encoded = toml::to_string_pretty(&manifest).unwrap();
        assert!(encoded.contains("[files]"));
        assert!(encoded.contains("\"js-fns/js-fns/vitest.config.ts\" = \"main\""));
        let decoded: VitManifest = toml::from_str(&encoded).unwrap();
        assert_eq!(decoded.files, manifest.files);
    }
}
