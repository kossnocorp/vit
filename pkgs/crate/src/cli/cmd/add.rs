use crate::cli::prelude::*;

#[derive(Args)]
pub struct VitCliCmdAdd {
    #[command(flatten)]
    manifest_args: VitCliArgsManifest,

    #[arg(value_name = "FILE")]
    file: String,
}

impl VitCliCmdAdd {
    pub async fn run(&self) -> Result<()> {
        VitVendor::add(self.manifest_args.manifest.as_deref(), &self.file).await
    }
}
