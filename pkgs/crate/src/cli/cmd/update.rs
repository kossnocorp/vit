use crate::cli::prelude::*;

#[derive(Args)]
pub struct VitCliCmdUpdate {
    #[command(flatten)]
    manifest_args: VitCliArgsManifest,

    #[arg(value_name = "FILE")]
    file: String,
}

impl VitCliCmdUpdate {
    pub async fn run(&self) -> Result<()> {
        VitVendor::update(self.manifest_args.manifest.as_deref(), &self.file).await
    }
}
