use crate::prelude::*;

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct VitManifest {
    #[serde(default)]
    files: BTreeMap<String, String>,
}

impl VitManifest {
    pub fn new() -> Self {
        Self {
            files: BTreeMap::new(),
        }
    }

    pub fn add(&mut self, path: impl AsRef<str>, hash: impl AsRef<str>) {
        self.files
            .insert(path.as_ref().to_string(), hash.as_ref().to_string());
    }

    pub fn get(&self, path: impl AsRef<str>) -> Option<&String> {
        self.files.get(path.as_ref())
    }

    pub fn has(&self, path: impl AsRef<str>) -> bool {
        self.files.contains_key(path.as_ref())
    }
}
