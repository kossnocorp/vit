use crate::cli::prelude::*;
use crate::vendor::update;

#[derive(Args)]
pub struct VitCliCmdUpdate {
    #[command(flatten)]
    manifest_args: VitCliArgsManifest,

    #[arg(value_name = "FILE")]
    file: String,
}

impl VitCliCmdUpdate {
    pub async fn run(&self) -> Result<()> {
        todo!()
    }
}
