use crate::prelude::*;

pub struct VitSourceInput;

impl VitSourceInput {
    pub fn parse_target(input: &str) -> Result<Box<dyn VitTarget>> {
        let sources: [&'static dyn VitSource; 2] = [&VIT_SOURCE_GITHUB, &VIT_SOURCE_HTTP];
        for source in sources {
            if let Some(target) = source.parse(input)? {
                return Ok(target);
            }
        }

        bail!("Unsupported source {input:?}; expected gh:owner/repo/path@version or an HTTP URL")
    }

    pub fn parse_manifest_target(
        key: &VitManifestTargetUrl,
        version: &VitManifestSourceVersion,
    ) -> Result<Box<dyn VitTarget>> {
        let input = if key.as_str().starts_with("gh:") {
            format!("{key}@{version}")
        } else {
            key.as_str().to_owned()
        };
        let target = Self::parse_target(&input)
            .with_context(|| format!("Invalid manifest target {key:?}"))?;
        ensure!(
            target.version() == version,
            "Manifest version {version:?} does not match source {key:?}"
        );
        Ok(target)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_sources_in_order() {
        assert!(
            VitSourceInput::parse_target("gh:js-fns/js-fns/vitest.config.ts@main")
                .unwrap()
                .as_any()
                .is::<VitSourceGitHubTarget>()
        );
        assert!(
            VitSourceInput::parse_target("https://example.com/assets/file.js")
                .unwrap()
                .as_any()
                .is::<VitSourceHttpTarget>()
        );
        assert!(VitSourceInput::parse_target("js-fns/js-fns/file.js@main").is_err());
    }

    #[test]
    fn restores_targets_from_manifest_entries() {
        let github = VitSourceInput::parse_manifest_target(
            &VitManifestTargetUrl::new("gh:js-fns/js-fns/vitest.config.ts"),
            &VitManifestSourceVersion::new("main"),
        )
        .unwrap();
        assert_eq!(github.version(), &VitManifestSourceVersion::new("main"));

        let url = "https://example.com/assets/file.js";
        let key = VitManifestTargetUrl::new(url);
        let http = VitSourceInput::parse_manifest_target(&key, &VitManifestSourceVersion::new(url))
            .unwrap();
        assert_eq!(http.key(), &key);
        assert!(
            VitSourceInput::parse_manifest_target(&key, &VitManifestSourceVersion::new("other"))
                .is_err()
        );
    }
}
