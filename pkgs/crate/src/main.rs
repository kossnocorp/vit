use prelude::*;

mod cli;
mod prelude;

#[tokio::main]
async fn main() {
    match Cli::run().await {
        Ok(_) => {}

        Err(err) => {
            println!("Error: {:?}", err);
            std::process::exit(1);
        }
    }
}
