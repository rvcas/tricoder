use async_trait::async_trait;
use reqwest::Client;

use crate::{
    error::Error,
    modules::{HttpFinding, HttpModule, Module},
};

pub struct GitHeadDisclosure;

impl GitHeadDisclosure {
    pub fn new() -> Self {
        GitHeadDisclosure {}
    }

    pub fn is_head_file(&self, content: &str) -> bool {
        Some(0) == content.to_lowercase().trim().find("ref:")
    }
}

impl Module for GitHeadDisclosure {
    fn name(&self) -> String {
        String::from("http/git_head_disclosure")
    }

    fn description(&self) -> String {
        String::from("Check for .git/HEAD file disclosure")
    }
}

#[async_trait]
impl HttpModule for GitHeadDisclosure {
    async fn scan(
        &self,
        http_client: &Client,
        endpoint: &str,
    ) -> Result<Option<HttpFinding>, Error> {
        let url = format!("{}/.git/HEAD", &endpoint);
        let res = http_client.get(&url).send().await?;

        if !res.status().is_success() {
            return Ok(None);
        }

        let body = res.text().await?;
        if self.is_head_file(&body) {
            return Ok(Some(HttpFinding::GitHeadDisclosure(url)));
        }

        Ok(None)
    }
}
