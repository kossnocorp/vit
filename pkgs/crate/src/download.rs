use crate::prelude::*;

use reqwest::Client;

pub struct VitDownload {
    pub revision: String,
    pub bytes: Vec<u8>,
}

impl VitDownload {
    pub async fn fetch(spec: &VitTarget) -> Result<VitDownload> {
        let client = Client::builder()
            .user_agent("vendorit.org/0.1")
            .build()
            .context("failed to create HTTP client")?;

        let commit_url = format!(
            "https://api.github.com/repos/{}/{}/commits/{}",
            spec.owner, spec.repo, spec.version
        );

        let response = client
            .get(&commit_url)
            .send()
            .await
            .with_context(|| format!("failed to resolve {}", spec.version))?
            .error_for_status()
            .with_context(|| format!("GitHub could not resolve {}", spec.version))?;

        #[derive(Deserialize)]
        struct Commit {
            sha: String,
        }

        let commit: Commit = response
            .json()
            .await
            .context("GitHub returned an invalid commit response")?;

        let raw_url = format!(
            "https://raw.githubusercontent.com/{}/{}/{}/{}",
            spec.owner, spec.repo, commit.sha, spec.path
        );

        let bytes = client
            .get(&raw_url)
            .send()
            .await
            .with_context(|| format!("failed to fetch {}", spec.path))?
            .error_for_status()
            .with_context(|| format!("GitHub could not fetch {}", spec.path))?
            .bytes()
            .await
            .context("failed to read downloaded file")?
            .to_vec();

        Ok(VitDownload {
            revision: commit.sha,
            bytes,
        })
    }
}
