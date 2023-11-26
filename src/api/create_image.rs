use serde::{Deserialize, Serialize};

use crate::IntoRequest;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateImageRequest {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateImageResponse {}

impl IntoRequest for CreateImageRequest {
    fn into_request(self, client: reqwest::Client) -> reqwest::RequestBuilder {
        client
            .post("https://api.openai.com/v1/images/generations")
            .json(&self)
    }
}
