use super::prelude::*;

mod install;
use install::*;

mod add;
use add::*;

mod update;
use update::*;

#[derive(Subcommand)]
pub enum VitCliCmd {
    /// Install dependencies
    Install(VitCliCmdInstall),

    /// Add a dependency
    Add(VitCliCmdAdd),

    /// Update a dependency
    Update(VitCliCmdUpdate),
}

impl VitCliCmd {
    pub async fn run(&self) -> Result<()> {
        match &self {
            VitCliCmd::Install(cmd) => cmd.run().await,

            VitCliCmd::Add(cmd) => cmd.run().await,

            VitCliCmd::Update(cmd) => cmd.run().await,
        }
    }
}
