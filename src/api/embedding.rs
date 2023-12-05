use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::IntoRequest;

#[derive(Debug, Clone, Serialize, Builder)]
#[builder(pattern = "mutable")]
pub struct EmbeddingRequest {
    input: EmbeddingInput,

    /// ID of the model to use. You can use the List models API to see all of your available models, or see our Model overview for descriptions of them.
    #[builder(default)]
    model: EmbeddingModel,

    /// The format to return the embeddings in. Can be either float or base64.
    #[builder(default, setter(strip_option, into))]
    #[serde(skip_serializing_if = "Option::is_none")]
    encoding_format: Option<EmbeddingEncodingFormat>,

    /// A unique identifier representing your end-user, which can help OpenAI to monitor and detect abuse.
    #[builder(default, setter(strip_option, into))]
    #[serde(skip_serializing_if = "Option::is_none")]
    user: Option<String>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize)]
pub enum EmbeddingEncodingFormat {
    #[default]
    #[serde(rename = "float")]
    FLOAT,
    #[serde(rename = "base64")]
    BASE64,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EmbeddingModel {
    #[default]
    #[serde(rename = "text-embedding-ada-002")]
    TextEmbeddingAda002,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum EmbeddingInput {
    String(String),
    StringArray(Vec<String>),
}

impl EmbeddingRequest {
    pub fn new(input: impl Into<EmbeddingInput>) -> Self {
        EmbeddingRequestBuilder::default()
            .input(input.into())
            .build()
            .unwrap()
    }
}

impl IntoRequest for EmbeddingRequest {
    fn into_request(self, client: reqwest::Client) -> reqwest::RequestBuilder {
        println!("【 self 】==> {:#?}", serde_json::to_string(&self).unwrap());
        client
            .post("https://api.openai.com/v1/embeddings")
            .json(&self)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct EmbeddingResponse {
    pub object: String,
    pub data: Vec<EmbeddingData>,
    pub model: String,
    pub usage: EmbeddingUsage,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EmbeddingUsage {
    pub prompt_tokens: usize,
    pub total_tokens: usize,
}

#[allow(dead_code)]
/// Represents an embedding vector returned by embedding endpoint.
#[derive(Debug, Clone, Deserialize)]
pub struct EmbeddingData {
    /// The index of the embedding in the list of embeddings.
    #[serde(default)]
    index: usize,
    /// The embedding vector, which is a list of floats. The length of vector depends on the model as listed in the embedding guide.
    embedding: Vec<f32>,
    /// The object type, which is always "embedding".
    object: String,
}

impl From<Vec<String>> for EmbeddingInput {
    fn from(value: Vec<String>) -> Self {
        EmbeddingInput::StringArray(value)
    }
}

impl From<&[String]> for EmbeddingInput {
    fn from(value: &[String]) -> Self {
        EmbeddingInput::StringArray(value.to_vec())
    }
}

impl From<String> for EmbeddingInput {
    fn from(value: String) -> Self {
        EmbeddingInput::String(value)
    }
}

impl From<&str> for EmbeddingInput {
    fn from(value: &str) -> Self {
        EmbeddingInput::String(value.into())
    }
}

#[cfg(test)]
mod tests {
    use crate::LLmSdk;

    use super::*;
    use anyhow::{Ok, Result};

    #[tokio::test]
    async fn embeddings_should_work() -> Result<()> {
        let sdk = LLmSdk::new(std::env::var("OPENAI_API_KEY")?);
        let req = EmbeddingRequest::new("The food was delicious and the waiter...");
        let res = sdk.embedding(req).await?;
        assert_eq!(res.data.len(), 1);
        assert_eq!(res.object, "list");

        let data = &res.data[0];
        assert_eq!(data.embedding.len(), 1536);
        assert_eq!(data.index, 0);
        assert_eq!(data.object, "embedding");

        Ok(())
    }

    #[tokio::test]
    async fn embeddings_input_array_should_work() -> Result<()> {
        let sdk = LLmSdk::new(std::env::var("OPENAI_API_KEY")?);
        let req = EmbeddingRequest::new(vec![
            "The quick brown fox jumped over the lazy dog.".into(),
            "我是谁？我在哪？".into(),
        ]);
        let res = sdk.embedding(req).await?;
        assert_eq!(res.data.len(), 2);
        assert_eq!(res.object, "list");
        let data = &res.data[1];
        assert_eq!(data.embedding.len(), 1536);
        assert_eq!(data.index, 1);
        assert_eq!(data.object, "embedding");

        Ok(())
    }
}
