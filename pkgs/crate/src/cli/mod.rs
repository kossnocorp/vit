mod prelude;
use prelude::*;

mod cmd;
use cmd::*;

mod args;
use args::*;

#[derive(Parser)]
#[command(name = "vit")]
#[command(about = "Vendored dependencies manager", long_about = None)]
#[command(arg_required_else_help = true)]
pub struct VitCli {
    #[command(subcommand)]
    pub command: Option<VitCliCmd>,
}

impl VitCli {
    pub async fn run() -> Result<()> {
        let cli = Self::parse();

        match &cli.command {
            Some(cmd) => cmd.run().await,

            None => bail!("No command specified. Use --help for usage information."),
        }
    }
}
