use crate::IntoRequest;
use derive_builder::Builder;
use reqwest::multipart::{Form, Part};
use serde::{Deserialize, Serialize};
use strum_macros::Display;

#[derive(Debug, Clone, Serialize, Builder)]
pub struct TranscriptionRequest {
    /// The audio file object (not file name) to transcribe, in one of these formats: flac, mp3, mp4, mpeg, mpga, m4a, ogg, wav, or webm.
    pub file: Vec<u8>,
    /// ID of the model to use. Only whisper-1 is currently available.
    #[builder(default)]
    pub model: TranscriptionModel,
    /// The language of the input audio. Supplying the input language in ISO-639-1 format will improve accuracy and latency.
    #[builder(default, setter(strip_option, into))]
    pub language: Option<String>,
    /// An optional text to guide the model's style or continue a previous audio segment. The prompt should match the audio language.
    #[builder(default, setter(strip_option, into))]
    pub prompt: Option<String>,
    /// The format of the transcript output, in one of these options: json, text, srt, verbose_json, or vtt.
    #[builder(default)]
    pub response_format: TranscriptionResponseFormat,
    /// The sampling temperature, between 0 and 1. Higher values like 0.8 will make the output more random, while lower values like 0.2 will make it more focused and deterministic. If set to 0, the model will use log probability to automatically increase the temperature until certain thresholds are hit.
    #[builder(default, setter(strip_option))]
    pub temperature: Option<f32>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Display)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum TranscriptionResponseFormat {
    #[default]
    Json,
    Text,
    Srt,
    VerboseJson,
    Vtt,
}

#[derive(Debug, Clone, Default, Serialize, Display)]
pub enum TranscriptionModel {
    #[serde(rename = "whisper-1")]
    #[strum(serialize = "whisper-1")]
    #[default]
    Whisper1,
}

#[derive(Debug, Clone, Deserialize, Builder)]
pub struct TranscriptionResponse {
    pub text: String,
}

impl TranscriptionRequest {
    pub fn new(stream: Vec<u8>) -> Self {
        TranscriptionRequestBuilder::default()
            .file(stream)
            .build()
            .unwrap()
    }

    fn into_form(self) -> Form {
        let part = Part::bytes(self.file)
            .file_name("file")
            .mime_str("audio/mp3")
            .unwrap();

        let form = Form::new()
            .part("file", part)
            .text("model", self.model.to_string())
            .text("response_format", self.response_format.to_string())
            .text("language", self.language.unwrap_or_default())
            .text("prompt", self.prompt.unwrap_or_default())
            .text(
                "temperature",
                self.temperature
                    .map_or_else(|| "".to_string(), |temp| temp.to_string()),
            );

        form
    }
}

impl IntoRequest for TranscriptionRequest {
    fn into_request(self, client: reqwest::Client) -> reqwest::RequestBuilder {
        client
            .post("https://api.openai.com/v1/audio/transcriptions")
            .multipart(self.into_form())
        // .form(&self)
    }
}

#[cfg(test)]
mod test {
    use std::fs;

    use crate::api::ChatResponseFormat;

    use super::*;
    use anyhow::Result;

    #[tokio::test]
    async fn transcription_should_work() -> Result<()> {
        let sdk = crate::LLmSdk::new(std::env::var("OPENAI_API_KEY")?);
        let stream = fs::read("fixtures/test.mp3")?;
        let req = TranscriptionRequest::new(stream);
        let res = sdk.transcription(req).await?;
        fs::write("res.xtx", &res.text);
        assert_eq!(res.text, "The quick brown fox jumped over the lazy dog.");

        Ok(())
    }

    #[tokio::test]
    async fn transcription_with_response_should_work() -> Result<()> {
        let sdk = crate::LLmSdk::new(std::env::var("OPENAI_API_KEY")?);
        let stream = fs::read("fixtures/test.mp3")?;
        let req = TranscriptionRequestBuilder::default()
            .file(stream)
            .response_format(TranscriptionResponseFormat::Text)
            .build()?;
        let res = sdk.transcription(req).await?;
        assert_eq!(res.text, "The quick brown fox jumped over the lazy dog.\n");

        Ok(())
    }
}
