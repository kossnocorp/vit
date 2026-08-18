use crate::prelude::*;

mod cache;
pub use cache::*;

pub static VIT_SOURCE_GITHUB: VitSourceGitHub = VitSourceGitHub;

pub struct VitSourceGitHub;

#[derive(Clone, Debug, PartialEq)]
pub struct VitSourceGitHubTarget {
    key: String,
    owner: String,
    repo: String,
    path: String,
    version: String,
    source_url: String,
}

#[async_trait]
impl VitSource for VitSourceGitHub {
    fn parse(&self, input: &str) -> Result<Option<Box<dyn VitTarget>>> {
        let Some(input) = input.strip_prefix("gh:") else {
            return Ok(None);
        };
        let (source, version) = input.rsplit_once('@').with_context(|| {
            format!("Invalid GitHub target {input:?}; expected gh:owner/repo/path@version")
        })?;
        ensure!(!version.is_empty(), "GitHub version must not be empty");

        let mut parts = source.splitn(3, '/');
        let owner = parts.next().unwrap_or_default();
        let repo = parts.next().unwrap_or_default();
        let path = parts.next().unwrap_or_default();

        ensure!(!owner.is_empty(), "Repository owner must not be empty");
        ensure!(!repo.is_empty(), "Repository name must not be empty");
        ensure!(
            [owner, repo].iter().all(|part| matches!(
                Path::new(part).components().next(),
                Some(Component::Normal(_))
            )),
            "Repository owner and name must be safe path components"
        );
        ensure!(!path.is_empty(), "Repository file path must not be empty");
        ensure!(
            Path::new(path)
                .components()
                .all(|part| matches!(part, Component::Normal(_))),
            "Repository file path must not contain absolute or traversal components"
        );

        let valid_ref = version.len() == 40 && version.bytes().all(|byte| byte.is_ascii_hexdigit())
            || git2::Reference::is_valid_name(&format!("refs/heads/{version}"));
        ensure!(valid_ref, "Version must be a valid Git ref or commit SHA");

        Ok(Some(Box::new(VitSourceGitHubTarget {
            key: format!("gh:{source}"),
            owner: owner.to_owned(),
            repo: repo.to_owned(),
            path: path.to_owned(),
            version: version.to_owned(),
            source_url: format!("https://github.com/{owner}/{repo}/blob/{version}/{path}"),
        })))
    }

    async fn download(&self, target: &dyn VitTarget) -> Result<VitSourceFile> {
        let target = target
            .as_any()
            .downcast_ref::<VitSourceGitHubTarget>()
            .context("GitHub source received a target from another source")?
            .clone();
        VitGitHubCache::try_new()?.fetch(target).await
    }
}

impl VitTarget for VitSourceGitHubTarget {
    fn key(&self) -> &str {
        &self.key
    }

    fn version(&self) -> &str {
        &self.version
    }

    fn source_url(&self) -> &str {
        &self.source_url
    }

    fn vendor_path(&self) -> PathBuf {
        PathBuf::from(format!("@{}", self.owner))
            .join(&self.repo)
            .join(&self.path)
    }

    fn source(&self) -> &'static dyn VitSource {
        &VIT_SOURCE_GITHUB
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_prefixed_github_target() {
        let target = VIT_SOURCE_GITHUB
            .parse("gh:js-fns/js-fns/vitest.config.ts@main")
            .unwrap()
            .unwrap();
        assert_eq!(target.key(), "gh:js-fns/js-fns/vitest.config.ts");
        assert_eq!(target.version(), "main");
        assert_eq!(
            target.vendor_path(),
            Path::new("@js-fns/js-fns/vitest.config.ts")
        );
        assert!(
            VIT_SOURCE_GITHUB
                .parse("js-fns/js-fns/file@main")
                .unwrap()
                .is_none()
        );
        assert!(
            VIT_SOURCE_GITHUB
                .parse("gh:js-fns/js-fns/../secret@main")
                .is_err()
        );
    }
}
