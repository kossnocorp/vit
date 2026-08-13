use crate::cli::prelude::*;
use crate::vendor::add;

#[derive(Args)]
pub struct VitCliCmdAdd {
    #[command(flatten)]
    manifest_args: VitCliArgsManifest,

    #[arg(value_name = "FILE")]
    file: String,
}

impl VitCliCmdAdd {
    pub async fn run(&self) -> Result<()> {
        todo!()
    }
}
