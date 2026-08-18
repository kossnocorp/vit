use crate::cli::prelude::*;

#[derive(Args)]
pub struct VitCliCmdInstall {
    #[command(flatten)]
    manifest_args: VitCliArgsManifest,

    /// Never hit network, use only local cache.
    #[arg(short, long, default_value_t = false)]
    offline: bool,
}

impl VitCliCmdInstall {
    pub async fn run(&self) -> Result<()> {
        VitVendor::install(self.manifest_args.manifest.as_deref(), self.offline).await
    }
}
