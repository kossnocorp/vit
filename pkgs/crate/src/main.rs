use prelude::*;

mod cli;
mod prelude;
mod vendor;

#[tokio::main]
async fn main() {
    match VitCli::run().await {
        Ok(_) => {}

        Err(err) => {
            println!("Error: {:?}", err);
            std::process::exit(1);
        }
    }
}
