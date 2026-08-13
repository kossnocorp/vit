use crate::prelude::*;

#[derive(Debug, PartialEq)]
pub struct VitTarget {
    pub key: String,
    pub owner: String,
    pub repo: String,
    pub path: String,
    pub version: String,
}

impl VitTarget {
    pub fn parse(input: &str) -> Result<Self> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_supported_spec() {
        assert_eq!(
            VitTarget::parse("js-fns/js-fns/vitest.config.ts@main").unwrap(),
            VitTarget {
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
        assert!(VitTarget::parse("js-fns/js-fns/../secret@main").is_err());
        assert!(VitTarget::parse("js-fns/js-fns/file@feature/ref").is_err());
        assert!(VitTarget::parse("js-fns//file@main").is_err());
    }
}
