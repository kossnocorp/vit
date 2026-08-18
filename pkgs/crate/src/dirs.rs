use crate::prelude::*;

pub struct VitDirs(#[allow(dead_code)] ProjectDirs);

impl VitDirs {
    pub fn resolve() -> Result<Self> {
        let dirs = ProjectDirs::from("org", "vendorit", "vit")
            .with_context(|| "Failed to resolve Vit dirs")?;
        Ok(Self(dirs))
    }
}
