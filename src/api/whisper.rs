use crate::{IntoRequest, SDK};
use derive_builder::Builder;
use reqwest::multipart::{Form, Part};
use serde::{Deserialize, Serialize};
use strum_macros::Display;

#[derive(Debug, Clone, Serialize, Builder)]
pub struct WhisperRequest {
    /// The audio file object (not file name) to transcribe, in one of these formats: flac, mp3, mp4, mpeg, mpga, m4a, ogg, wav, or webm.
    pub file: Vec<u8>,
    /// ID of the model to use. Only whisper-1 is currently available.
    #[builder(default)]
    pub model: WhisperModel,
    /// The language of the input audio. Supplying the input language in ISO-639-1 format will improve accuracy and latency.
    #[builder(default, setter(strip_option, into))]
    pub language: Option<String>,
    /// An optional text to guide the model's style or continue a previous audio segment. The prompt should match the audio language.
    #[builder(default, setter(strip_option, into))]
    pub prompt: Option<String>,
    /// The format of the transcript output, in one of these options: json, text, srt, verbose_json, or vtt.
    #[builder(default)]
    pub response_format: WhisperResponseFormat,
    /// The sampling temperature, between 0 and 1. Higher values like 0.8 will make the output more random, while lower values like 0.2 will make it more focused and deterministic. If set to 0, the model will use log probability to automatically increase the temperature until certain thresholds are hit.
    #[builder(default, setter(strip_option))]
    pub temperature: Option<f32>,

    #[builder(default)]
    #[serde(skip_serializing)]
    request_type: WhisperRequestType,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum WhisperRequestType {
    #[default]
    Transcription,
    Translation,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Display)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum WhisperResponseFormat {
    #[default]
    Json,
    Text,
    Srt,
    VerboseJson,
    Vtt,
}

#[derive(Debug, Clone, Default, Serialize, Display)]
pub enum WhisperModel {
    #[serde(rename = "whisper-1")]
    #[strum(serialize = "whisper-1")]
    #[default]
    Whisper1,
}

#[derive(Debug, Clone, Deserialize, Builder)]
pub struct WhisperResponse {
    pub text: String,
}

impl WhisperRequest {
    pub fn transcription(stream: Vec<u8>) -> Self {
        WhisperRequestBuilder::default()
            .request_type(WhisperRequestType::Transcription)
            .file(stream)
            .build()
            .unwrap()
    }

    pub fn translation(stream: Vec<u8>) -> Self {
        WhisperRequestBuilder::default()
            .request_type(WhisperRequestType::Translation)
            .file(stream)
            .build()
            .unwrap()
    }

    fn into_form(self) -> Form {
        let part = Part::bytes(self.file)
            .file_name("file")
            .mime_str("audio/mp3")
            .unwrap();

        let mut form = Form::new()
            .part("file", part)
            .text("model", self.model.to_string())
            .text("response_format", self.response_format.to_string())
            .text("prompt", self.prompt.unwrap_or_default())
            .text(
                "temperature",
                self.temperature
                    .map_or_else(|| "".to_string(), |temp| temp.to_string()),
            );

        if self.request_type == WhisperRequestType::Transcription && self.language.is_some() {
            form = form.text("language", self.language.unwrap());
        }

        form
    }
}

impl IntoRequest for WhisperRequest {
    fn into_request(self, client: reqwest::Client) -> reqwest::RequestBuilder {
        let api_url = if self.request_type == WhisperRequestType::Translation {
            format!("{}{}", SDK.base_url, "/audio/translations")
        } else {
            format!("{}{}", &SDK.base_url, "/audio/transcriptions")
        };

        client.post(api_url).multipart(self.into_form())
    }
}

#[cfg(test)]
mod test {
    use std::fs;

    use super::*;
    use anyhow::Result;

    #[tokio::test]
    async fn transcription_should_work() -> Result<()> {
        let sdk = &crate::SDK;
        let stream = fs::read("fixtures/test.mp3")?;
        let req = WhisperRequest::transcription(stream);
        let res = sdk.whisper(req).await?;
        fs::write("res.xtx", &res.text);
        assert_eq!(res.text, "The quick brown fox jumped over the lazy dog.");

        Ok(())
    }

    #[tokio::test]
    async fn transcription_with_response_should_work() -> Result<()> {
        let sdk = &crate::SDK;
        let stream = fs::read("fixtures/test.mp3")?;
        let req = WhisperRequestBuilder::default()
            .file(stream)
            .response_format(WhisperResponseFormat::Text)
            .build()?;
        let res = sdk.whisper(req).await?;
        assert_eq!(res.text, "The quick brown fox jumped over the lazy dog.\n");

        Ok(())
    }

    #[tokio::test]
    async fn transcription_with_request_type_should_work() -> Result<()> {
        let sdk = &crate::SDK;
        let stream = fs::read("fixtures/wow.mp3")?;
        let req = WhisperRequestBuilder::default()
            .file(stream)
            .response_format(WhisperResponseFormat::Srt)
            .request_type(WhisperRequestType::Translation)
            .build()?;
        let res = sdk.whisper(req).await?;
        fs::write("res.txt", &res.text);
        assert_eq!(
            res.text,
            "1\n00:00:00,000 --> 00:00:02,000\n欢迎来到爱泽拉斯 Welcome to愛泽拉斯\n\n\n"
        );

        Ok(())
    }
}
