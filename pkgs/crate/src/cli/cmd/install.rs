use crate::cli::prelude::*;

#[derive(Args)]
pub struct CliCmdInstall {
    /// Never hit network, use only local cache.
    #[arg(short, long, default_value_t = false)]
    offline: bool,
}

impl CliCmdRunnable for CliCmdInstall {
    async fn run(&self, cli: &Cli) -> Result<()> {
        println!("CLI config={:?}", cli.manifest);
        println!("Running install command with offline={}", self.offline);
        Ok(())
    }
}
