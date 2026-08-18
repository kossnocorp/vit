mod add;

mod install;

mod update;

pub struct VitVendor;
#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use std::fs;

    #[tokio::test]
    async fn install_removes_files_missing_from_manifest_and_updates_lock() {
        let directory = tempfile::tempdir().unwrap();
        let paths = VitPaths::resolve(Some(directory.path())).await.unwrap();
        VitManifest::new()
            .write_toml(&paths.manifest)
            .await
            .unwrap();

        let stale_path = directory.path().join("vendor/@owner/repo/stale.txt");
        fs::create_dir_all(stale_path.parent().unwrap()).unwrap();
        fs::write(&stale_path, "stale").unwrap();
        let unrelated_path = directory.path().join("vendor/unrelated.txt");
        fs::write(&unrelated_path, "keep").unwrap();

        let mut lock = VitLock::default();
        lock.files.insert(
            "gh:owner/repo/stale.txt".to_owned(),
            VitLockFile {
                version: "main".to_owned(),
                revision: "revision".to_owned(),
                hash: "sha256:stale".to_owned(),
                source: "https://example.com/stale.txt".to_owned(),
                path: "vendor/@owner/repo/stale.txt".to_owned(),
            },
        );
        lock.write_toml(&paths.lock).await.unwrap();

        VitVendor::install(Some(directory.path()), false)
            .await
            .unwrap();

        assert!(!stale_path.exists());
        assert!(!directory.path().join("vendor/@owner").exists());
        assert_eq!(fs::read_to_string(unrelated_path).unwrap(), "keep");
        assert!(
            VitLock::read_toml(&paths.lock)
                .await
                .unwrap()
                .files
                .is_empty()
        );
    }

    #[tokio::test]
    async fn resolves_manifest_and_target_paths() {
        let directory = tempfile::tempdir().unwrap();
        let paths = VitPaths::resolve(Some(directory.path())).await.unwrap();
        let target = VitSourceInput::parse_target("gh:js-fns/js-fns/src/file.ts@main").unwrap();
        assert_eq!(paths.manifest, directory.path().join("vendor.toml"));
        assert_eq!(
            paths.target(target.as_ref()),
            directory.path().join("vendor/@js-fns/js-fns/src/file.ts")
        );
        assert!(
            VitPaths::resolve(Some(Path::new("other.toml")))
                .await
                .is_err()
        );
    }
}
