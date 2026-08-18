mod prelude;

mod cli;
pub use cli::*;

mod state;
pub use state::*;

mod manifest;
pub use manifest::*;

mod lock;
pub use lock::*;

mod source;
pub use source::*;

pub mod file;
pub use file::*;

mod paths;
pub use paths::*;

mod dirs;
pub use dirs::*;

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
