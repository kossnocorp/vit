mod prelude;

mod cli;
pub use cli::*;

mod state;
pub use state::*;

mod manifest;
pub use manifest::*;

mod target;
pub use target::*;

mod lock;
pub use lock::*;

mod download;
pub use download::*;

mod paths;
pub use paths::*;

mod vendor;
pub use vendor::*;

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
