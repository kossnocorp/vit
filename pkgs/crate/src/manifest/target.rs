use crate::prelude::*;

/// Target URL, e.g., "gh:kossnocorp/dev/mise.toml" or "gh:kossnocorp/dev".
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[serde(transparent)]
pub struct VitManifestTargetUrl(String);

impl VitManifestTargetUrl {
    pub fn new(url: impl AsRef<str>) -> Self {
        Self(url.as_ref().to_owned())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn join(&self, path: &VitManifestTargetPath) -> Result<Self> {
        ensure!(!self.0.is_empty(), "Manifest target URL must not be empty");
        ensure!(!path.0.is_empty(), "Manifest target path must not be empty");
        ensure!(
            Path::new(&path.0)
                .components()
                .all(|part| matches!(part, Component::Normal(_))),
            "Manifest target path {:?} must be relative and must not contain traversal components",
            path.0
        );
        Ok(Self(format!("{}/{}", self.0.trim_end_matches('/'), path.0)))
    }
}

impl std::fmt::Display for VitManifestTargetUrl {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(formatter)
    }
}

/// Target path that can be joined with [VitManifestTargetUrl], e.g., "mise.toml".
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[serde(transparent)]
pub struct VitManifestTargetPath(String);

impl VitManifestTargetPath {
    pub fn new(path: impl AsRef<str>) -> Self {
        Self(path.as_ref().to_owned())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for VitManifestTargetPath {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(formatter)
    }
}
