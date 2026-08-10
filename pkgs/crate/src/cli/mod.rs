mod prelude;
use prelude::*;

mod cmd;
use cmd::*;

#[derive(Parser)]
#[command(name = "vit")]
#[command(about = "Vendored dependencies manager", long_about = None)]
#[command(arg_required_else_help = true)]
pub struct Cli {
    /// Use a specific manifest file instead of the default `vendor.toml`.
    #[arg(short, long, value_name = "MANIFEST_PATH")]
    pub manifest: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Option<CliCmd>,
}

impl Cli {
    pub async fn run() -> Result<()> {
        let cli = Self::parse();
        CliCmd::run(&cli).await
    }
}
