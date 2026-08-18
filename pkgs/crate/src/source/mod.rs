use crate::prelude::*;

use std::any::Any;

mod input;
pub use input::*;

mod github;
pub use github::*;

mod http;
pub use http::*;

mod file;
pub use file::*;

#[async_trait]
pub trait VitSource: Send + Sync {
    fn parse(&self, input: &str) -> Result<Option<Box<dyn VitTarget>>>;

    async fn download(&self, target: &dyn VitTarget) -> Result<VitSourceFile>;
}

pub trait VitTarget: Any + Send + Sync {
    fn key(&self) -> &VitManifestTargetUrl;

    fn version(&self) -> &VitManifestSourceVersion;

    fn source_url(&self) -> &str;

    fn vendor_path(&self) -> PathBuf;

    fn source(&self) -> &'static dyn VitSource;

    fn as_any(&self) -> &dyn Any;
}
