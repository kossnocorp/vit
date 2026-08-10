use super::prelude::*;

mod runnable;
pub use runnable::*;

mod install;
use install::*;

mod add;
use add::*;

#[derive(Subcommand)]
pub enum CliCmd {
    /// Install dependencies
    Install(CliCmdInstall),

    /// Add a dependency
    Add(CliCmdAdd),
}

impl CliCmd {
    pub async fn run(cli: &Cli) -> Result<()> {
        match &cli.command {
            Some(CliCmd::Install(cmd)) => cmd.run(cli).await,

            Some(CliCmd::Add(cmd)) => cmd.run(cli).await,

            None => bail!("No command specified. Use --help for usage information."),
        }
    }
}
