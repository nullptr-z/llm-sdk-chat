mod api;

use anyhow::{Ok, Result};
use api::*;
use async_trait::async_trait;
use bytes::Bytes;
use reqwest::{Client, RequestBuilder, Response};
use schemars::{schema_for, JsonSchema};
use std::time::Duration;

static TIMEOUT: u64 = 30;

#[derive(Debug)]
pub struct LLmSdk {
    pub(crate) base_url: String,
    pub(crate) token: String,
    pub(crate) client: Client,
}

pub trait IntoRequest {
    fn into_request(self, client: Client) -> RequestBuilder;
}

impl LLmSdk {
    pub fn new(base_url: impl Into<String>, token: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
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
        let res = req.send_and_log().await?;
        Ok(res.json::<CreateImageResponse>().await?)
    }

    /// Response media stream
    pub async fn speech(&self, req: SpeechRequest) -> Result<Bytes> {
        let req = self.prepare_request(req);
        let res = req.send_and_log().await?;
        Ok(res.bytes().await?)
    }

    pub async fn whisper(&self, req: WhisperRequest) -> Result<WhisperResponse> {
        let is_json = req.response_format == WhisperResponseFormat::Json;
        let req = self.prepare_request(req);
        let res = req.send_and_log().await?;

        let ret = if is_json {
            res.json::<WhisperResponse>().await?
        } else {
            WhisperResponse {
                text: res.text().await?,
            }
        };

        Ok(ret)
    }

    pub async fn embedding(&self, req: EmbeddingRequest) -> Result<EmbeddingResponse> {
        let req = self.prepare_request(req);
        let res = req.send_and_log().await?;
        Ok(res.json::<EmbeddingResponse>().await?)
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
            tracing::error!("API failed: {}", text);
            return Err(anyhow::anyhow!("API failed: {}", text));
        }

        Ok(res)
    }
}

/// For tool function. If you have a function that you want ChatGPT to call, you shall put all params into a struct
/// and derive schmears::JsonSchema for it. Then you use `StructName::to_schema` to generate json schema for tools.
pub trait ToSchema: JsonSchema {
    fn to_schema() -> serde_json::Value {
        serde_json::to_value(schema_for!(Self)).unwrap()
    }
}

impl<T: JsonSchema> ToSchema for T {}

/// 在main函数之前运行
#[cfg(test)]
#[ctor::ctor]
fn init() {
    tracing_subscriber::fmt::init()
}

lazy_static::lazy_static! {
    static ref SDK:LLmSdk=LLmSdk::new(
        "https://api.openai.com/v1",
        std::env::var("OPENAI_API_KEY").unwrap_or("".to_string())
    );
}
