use crate::cli::prelude::*;

pub trait CliCmdRunnable {
    async fn run(&self, cli: &Cli) -> Result<()>;
}
