use crate::cli::prelude::*;

#[derive(Args)]
pub struct VitCliArgsManifest {
    /// Use a specific manifest file instead of the default `vendor.toml`.
    #[arg(short, long, value_name = "MANIFEST_PATH")]
    pub manifest: Option<PathBuf>,
}
