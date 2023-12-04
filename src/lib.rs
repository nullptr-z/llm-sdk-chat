mod api;

use anyhow::{Ok, Result};
use api::*;
use async_trait::async_trait;
use reqwest::{Client, RequestBuilder, Response};
use std::time::Duration;

static TIMEOUT: u64 = 30;

#[derive(Debug)]
pub struct LLmSdk {
    pub(crate) token: String,
    pub(crate) client: Client,
}

pub trait IntoRequest {
    fn into_request(self, client: Client) -> RequestBuilder;
}

impl LLmSdk {
    pub fn new(token: impl Into<String>) -> Self {
        Self {
            token: token.into(),
            client: Client::new(),
        }
    }
    pub async fn chat_completion(
        &self,
        req: ChatCompletionRequest,
    ) -> Result<ChatCompletionResponse> {
        let req = self.prepare_request(req);
        let res = req.send_and_log().await?;

        Ok(res.json::<ChatCompletionResponse>().await?)
    }

    pub async fn create_image(&self, req: CreateImageRequest) -> Result<CreateImageResponse> {
        let req = self.prepare_request(req);
        let res = req.send().await?;
        Ok(res.json::<CreateImageResponse>().await?)
    }

    fn prepare_request(&self, req: impl IntoRequest) -> RequestBuilder {
        let req = req.into_request(self.client.clone());
        let req = if self.token.is_empty() {
            req
        } else {
            req.bearer_auth(&self.token)
        };
        req.timeout(Duration::from_secs(TIMEOUT))
    }
}

#[async_trait]
trait SendAndLong {
    async fn send_and_log(self) -> Result<Response>;
}

#[async_trait]
impl SendAndLong for RequestBuilder {
    async fn send_and_log(self) -> Result<Response> {
        let res = self.send().await?;
        let status = res.status();
        if status.is_client_error() || status.is_server_error() {
            let text = res.text().await?;
            tracing::error!("chat_completion failed: {}", text);
            return Err(anyhow::anyhow!("chat_completion failed: {}", text));
        }

        Ok(res)
    }
}

/// 在main函数之前运行
#[cfg(test)]
#[ctor::ctor]
fn init() {
    tracing_subscriber::fmt::init()
}
