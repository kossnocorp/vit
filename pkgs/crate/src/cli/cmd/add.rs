use crate::cli::prelude::*;

#[derive(Args)]
pub struct CliCmdAdd {
    #[arg(value_name = "FILE")]
    file: String,
}

impl CliCmdRunnable for CliCmdAdd {
    async fn run(&self, cli: &Cli) -> Result<()> {
        println!("CLI config={:?}", cli.manifest);
        println!("Running add command with file={}", self.file);
        Ok(())
    }
}
