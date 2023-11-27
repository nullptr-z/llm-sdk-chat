use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::IntoRequest;

#[derive(Debug, Clone, Serialize, Builder)]
#[builder(pattern = "mutable")]
pub struct CreateImageRequest {
    // A text description of the desired image(s). The maximum length is 4000 characters for dall-e-3.
    // 所需图像的文本描述。 dall-e-3 的最大长度为 4000 个字符。
    #[builder(setter(into))]
    prompt: String,
    // The model to use for image generation.
    // 用于图像生成的模型。
    #[builder(default)]
    model: ImageModel,
    // The number of images to generate. Must be between 1 and 10. For dall-e-3, only n=1 is supported.
    // 要生成的图像数量。必须介于 1 和 10 之间。对于 dall-e-3，仅支持 n=1。
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    n: Option<usize>,
    // The quality of the image that will be generated. hd creates images with finer details and greater consistency across the image. This param is only supported for dall-e-3.
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    quality: Option<ImageQuality>,
    // The format in which the generated images are returned. Must be one of url or b64_json
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    response_format: Option<ImageResponseFormat>,
    // The size of the generated images. Must be one of  1024x1024, 1792x1024, or 1024x1792 for dall-e-3 models.
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    size: Option<ImageSize>,
    // The style of the generated images. Must be one of vivid or natural. Vivid causes the model to lean towards generating hyper-real and dramatic images. Natural causes the model to produce more natural, less hyper-real looking images. This param is only supported for dall-e-3.
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    style: Option<ImageStyle>,
    // A unique identifier representing your end-user, which can help OpenAI to monitor and detect abuse. Learn more.
    #[builder(default, setter(strip_option, into))]
    #[serde(skip_serializing_if = "Option::is_none")]
    user: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub enum ImageModel {
    #[serde(rename = "dall-e-3")]
    DallE3,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ImageQuality {
    #[serde(rename = "standard")]
    Standard,
    #[serde(rename = "hd")]
    Hd,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ImageResponseFormat {
    Url,
    B64Json,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ImageSize {
    #[serde(rename = "1024x1024")]
    Large,
    #[serde(rename = "1792x1024")]
    LargeWide,
    #[serde(rename = "1024x1792")]
    LargeTall,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ImageStyle {
    #[serde(rename = "vivid")]
    Vivid,
    #[serde(rename = "natural")]
    Natural,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateImageResponse {
    pub created: u64,
    pub data: Vec<ImageObject>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ImageObject {
    // The base64-encoded JSON of the generated image, if response_format is b64_json.
    pub b64_json: Option<String>,
    // The URL of the generated image, if response_format is url (default).
    pub url: Option<String>,
    // The prompt that was used to generate the image, if there was any revision to the prompt.
    pub revised_prompt: String,
}

impl CreateImageRequest {
    pub fn new(prompt: impl Into<String>) -> Self {
        CreateImageRequestBuilder::default()
            .prompt(prompt)
            .build()
            .unwrap()
    }
}

impl IntoRequest for CreateImageRequest {
    fn into_request(self, client: reqwest::Client) -> reqwest::RequestBuilder {
        client
            .post("https://api.openai.com/v1/images/generations")
            .json(&self)
    }
}

impl Default for ImageModel {
    fn default() -> Self {
        ImageModel::DallE3
    }
}

impl Default for ImageQuality {
    fn default() -> Self {
        ImageQuality::Standard
    }
}

impl Default for ImageSize {
    fn default() -> Self {
        ImageSize::Large
    }
}

impl Default for ImageStyle {
    fn default() -> Self {
        ImageStyle::Vivid
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use anyhow::{Ok, Result};
    use serde_json::json;

    use crate::LLmSdk;

    use super::*;

    #[test]
    fn create_image_request_should_serialize() -> Result<()> {
        let req = CreateImageRequest::new("draw a picture of a chicken eating rice");
        let json = serde_json::to_value(&req)?;

        assert_eq!(
            json,
            json!({
                "prompt":"draw a picture of a chicken eating rice",
                "model":"dall-e-3"
            })
        );

        Ok(())
    }

    #[test]
    fn create_image_request_custom_should_serialize() -> Result<()> {
        let req = CreateImageRequestBuilder::default()
            .prompt("draw a picture of a chicken eating rice")
            .style(ImageStyle::Natural)
            .quality(ImageQuality::Hd)
            .build()?;
        let json = serde_json::to_value(&req)?;

        assert_eq!(
            json,
            json!({
                "prompt":"draw a picture of a chicken eating rice",
                "model":"dall-e-3",
                "style":"natural",
                "quality":"hd",
            })
        );

        Ok(())
    }

    #[tokio::test]
    async fn create_image_should_work() -> Result<()> {
        let sdk = LLmSdk::new(std::env::var("OPENAI_API_KEY")?);
        let req = CreateImageRequest::new("draw a picture of a chicken eating rice");
        let res = sdk.create_image(req).await?;
        let image = &res.data[0];

        assert_eq!(res.data.len(), 1);
        assert!(image.url.is_none());
        println!("image: {:?}", image);

        fs::write(
            "/tmp/llm-sdk/caterpillar.png",
            reqwest::get(image.url.as_ref().unwrap())
                .await?
                .bytes()
                .await?,
        )?;

        Ok(())
    }
}
