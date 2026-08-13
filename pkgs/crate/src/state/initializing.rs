use crate::prelude::*;

pub struct VitStateInitializing {
    dirs: ProjectDirs,
}

impl VitStateInitializing {
    pub fn try_new() -> Result<Self> {
        let dirs = ProjectDirs::from("org", "vendorit", "vendorit")
            .with_context(|| "Failed to create project directories")?;
        Ok(Self { dirs })
    }
}
