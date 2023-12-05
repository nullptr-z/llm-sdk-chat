use serde::{Deserialize, Serialize};

use crate::{IntoRequest, ToSchema};
use derive_builder::Builder;

#[derive(Debug, Clone, Serialize, Builder)]
pub struct ChatCompletionRequest {
    /// A list of messages comprising the conversation so far.w
    #[builder(setter(into))]
    messages: Vec<ChatCompletionMessage>,
    /// ID of the model to use. See the model endpoint compatibility table for details on which models work with the Chat API.
    #[builder(default)]
    model: ChatCompleteModel,
    /// Number between -2.0 and 2.0. Positive values penalize new tokens based on their existing frequency in the text so far, decreasing the model's likelihood to repeat the same line verbatim.
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    frequency_penalty: Option<i32>,
    // logit_bias: Option<i32>,
    /// The maximum number of tokens to generate in the chat completion.
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<usize>,
    /// How many chat completion choices to generate for each input message. Note that you will be charged based on the number of generated tokens across all of the choices. Keep n as 1 to minimize costs.
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    n: Option<usize>,
    /// Number between -2.0 and 2.0. Positive values penalize new tokens based on whether they appear in the text so far, increasing the model's likelihood to talk about new topics.
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    presence_penalty: Option<usize>,
    /// An object specifying the format that the model must output.
    /// Setting to { "type": "json_object" } enables JSON mode, which guarantees the message the model generates is valid JSON.
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    response_format: Option<ChatResponseFormatObject>,
    /// This feature is in Beta. If specified, our system will make a best effort to sample deterministically, such that repeated requests with the same seed and parameters should return the same result. Determinism is not guaranteed, and you should refer to the system_fingerprint response parameter to monitor changes in the backend.
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    seed: Option<usize>,
    /// Up to 4 sequences where the API will stop generating further tokens.
    // TODO: make this as an enum
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    stop: Option<String>,
    /// If set, partial message deltas will be sent, like in ChatGPT. Tokens will be sent as data-only server-sent events as they become available, with the stream terminated by a data: [DONE]
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    stream: Option<bool>,
    /// What sampling temperature to use, between 0 and 2. Higher values like 0.8 will make the output more random, while lower values like 0.2 will make it more focused and deterministic.
    /// We generally recommend altering this or top_p but not both.
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<i32>,
    /// An alternative to sampling with temperature, called nucleus sampling, where the model considers the results of the tokens with top_p probability mass. So 0.1 means only the tokens comprising the top 10% probability mass are considered.
    /// We generally recommend altering this or temperature but not both.
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    top_p: Option<i32>,
    /// A list of Tools the model may call. Currently, only functions are supported as a tool. Use this to provide a list of functions the model may generate JSON inputs for.
    #[builder(default, setter(into))]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    tools: Vec<Tool>,
    /// Controls which (if any) function is called by the model. none means the model will not call a function and instead generates a message. auto means the model can pick between generating a message or calling a function. Specifying a particular function via {"type: "function", "function": {"name": "my_function"}} forces the model to call that function.
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_choice: Option<ToolChoice>,
    /// A unique identifier representing your end-user, which can help OpenAI to monitor and detect abuse.
    #[builder(default, setter(strip_option, into))]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    user: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ToolChoice {
    #[default]
    None,
    Auto,
    Function {
        r#type: ToolType,
        name: String,
    },
}

#[derive(Debug, Clone, Serialize)]
pub struct Tool {
    /// The type of the tool. Currently, only function is supported.
    r#type: ToolType,
    /// The function is type of the tool.
    function: FunctionInfo,
}

#[derive(Debug, Clone, Serialize)]
pub struct FunctionInfo {
    /// A description of what the function does, used by the model to choose when and how to call the function.
    description: String,
    /// The name of the function to be called. Must be a-z, A-Z, 0-9, or contain underscores and dashes, with a maximum length of 64.
    name: String,
    /// The parameters the functions accepts, described as a JSON Schema object. See the guide for examples, and the JSON Schema reference for documentation about the format.
    /// To describe a function that accepts no parameters, provide the value {"type": "object", "properties": {}}.
    parameters: serde_json::Value,
}

#[derive(Debug, Clone, Serialize)]
pub struct ChatResponseFormatObject {
    r#type: ChatResponseFormat,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
/// tag 指定生成字段名称
pub enum ChatResponseFormat {
    Text,
    #[default]
    Json,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case", tag = "role")]
/// tag 指定生成字段名称
pub enum ChatCompletionMessage {
    /// A message from a system.
    System(SystemMessage),
    /// A mess age from a user.
    User(UserMessage),
    /// A message from the assistant.
    Assistant(AssistantMessage),
    /// A mess age from a toll.
    Tool(ToolMessage),
    // A message from  a function.
    // Function(FunctionMessage),
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChatCompleteModel {
    #[default]
    #[serde(rename = "gpt-3.5-turbo-1106")]
    GPT3Turbo,
    #[serde(rename = "gpt-3.5-turbo-instruct")]
    GPT3TurboInstruct,
    #[serde(rename = "gpt-4-1106-preview")]
    GPT4Turbo,
    #[serde(rename = "gpt-4-1106-vision-preview")]
    GPT4TurboVersion,
    #[serde(rename = "babbage-002")]
    Test,
}

#[derive(Debug, Clone, Serialize, Builder)]
pub struct SystemMessage {
    /// The contents of the system message
    content: String,
    /// An optional name for the participant. Provides the model information to differentiate between participants of the same role.
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Builder)]
pub struct UserMessage {
    /// The contents of the user message
    content: String,
    /// An optional name for the participant. Provides the model information to differentiate between participants of the same role.
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssistantMessage {
    /// The contents of the assistant message
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<String>,
    /// An optional name for the participant. Provides the model information to differentiate between participants of the same role.
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    /// The tool calls generated by the model, such as function calls.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    tool_calls: Vec<ToolCalls>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ToolMessage {
    /// The contents of the tool message.
    content: String,
    /// Tool call that this message is responding to.
    tool_call_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCalls {
    /// The ID of the tool call.
    id: String,
    /// The type of the tool. Currently, only function is supported.
    r#type: ToolType,
    /// The function that the model called.
    function: FunctionCall,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct FunctionCall {
    /// The name of the function to call.
    name: String,
    /// The arguments to call the function with, as generated by the model in JSON format.
    arguments: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum ToolType {
    #[default]
    Function,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ChatCompletionResponse {
    /// A unique identifier for the chat completion.
    id: String,
    /// A list of chat completionchoices. Can be more than one if n is greater than 1.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    choices: Vec<ChatCompletionChoice>,
    /// The Unix timestamp (in seconds) of when the chat completion was created.
    pub created: usize,
    /// The model used for the chat completion.
    pub model: ChatCompleteModel,
    /// This fingerprint represents the backend configuration that the model runs with.
    /// Can be used in conjunction with the seed request parameter to understand when backend changes have been made that might impact determinism.
    pub system_fingerprint: String,
    /// The object type, which is always chat.completion.
    pub object: String,
    /// Usage statistics for the completion request.
    pub usage: ChatCompletionUsage,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ChatCompletionChoice {
    /// The reason the model stopped generating tokens. This will be stop if the model hit a natural stop point or a provided stop sequence, length if the maximum number of tokens specified in the request was reached, content_filter if content was omitted due to a flag from our content filters, tool_calls if the model called a tool, or function_call (deprecated) if the model called a function.
    pub finish_reason: FinishReason,
    /// The index of the choice in the list of choices.
    pub index: usize,
    /// A chat completion message generated by the model.
    pub message: AssistantMessage,
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub struct ChatCompletionUsage {
    /// Number of tokens in the generated completion.
    completion_tokens: usize,
    /// Number of tokens in the prompt.
    prompt_tokens: usize,
    /// Total number of tokens used in the request (prompt + completion).
    total_tokens: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum FinishReason {
    #[default]
    Stop,
    Length,
    ContentFilter,
    ToolCalls,
}

impl IntoRequest for ChatCompletionRequest {
    fn into_request(self, client: reqwest::Client) -> reqwest::RequestBuilder {
        let url = format!("{}{}", crate::SDK.base_url, "/chat/completions");
        client.post(url).json(&self)
    }
}

#[allow(dead_code)]
impl ChatCompletionMessage {
    pub fn new_system(content: impl Into<String>, name: &str) -> ChatCompletionMessage {
        ChatCompletionMessage::System(SystemMessage {
            content: content.into(),
            name: Self::get_name(name),
        })
    }

    pub fn new_user(content: impl Into<String>, name: &str) -> ChatCompletionMessage {
        ChatCompletionMessage::User(UserMessage {
            content: content.into(),
            name: Self::get_name(name),
        })
    }

    #[inline]
    fn get_name(name: &str) -> Option<String> {
        (!name.is_empty()).then(|| name.into())
    }
}

impl Tool {
    #[allow(dead_code)]
    pub fn new_function<T: ToSchema>(name: impl Into<String>, des: impl Into<String>) -> Tool {
        Tool {
            r#type: ToolType::Function,
            function: FunctionInfo {
                name: name.into(),
                description: des.into(),
                parameters: T::to_schema(),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ToSchema;

    use super::*;
    use anyhow::{Ok, Result};
    use schemars::JsonSchema;
    use std::fs;

    #[allow(dead_code)]
    #[derive(Debug, Clone, JsonSchema, Deserialize)]
    struct GetWeatherArgs {
        /// Get the weather for the city
        city: String,
        /// the unit
        unit: TemperatureUnit,
    }

    #[derive(Debug, Clone)]
    struct GetWeatherResponse {
        temperature: f32,
        unit: TemperatureUnit,
    }

    #[allow(dead_code)]
    #[derive(Debug, Default, Clone, PartialEq, Eq, Copy, JsonSchema, Deserialize)]
    enum TemperatureUnit {
        // 摄氏度
        #[default]
        Celsius,
        // 华氏度
        Fahrenheit,
    }

    #[allow(dead_code)]
    #[derive(Debug, JsonSchema)]
    struct ExplainMoodArgs {
        /// The mood to explain.
        name: String,
    }

    #[allow(dead_code)]
    fn get_weather_forecast(args: GetWeatherArgs) -> GetWeatherResponse {
        match args.unit {
            TemperatureUnit::Celsius => GetWeatherResponse {
                temperature: 22.1,
                unit: TemperatureUnit::Celsius,
            },
            TemperatureUnit::Fahrenheit => GetWeatherResponse {
                temperature: 70.1,
                unit: TemperatureUnit::Fahrenheit,
            },
        }
    }

    #[test]
    fn chat_completion_request_serialize_should_work() {
        let mut req = get_simple_completion_request();
        req.tool_choice = Some(ToolChoice::Function {
            r#type: ToolType::Function,
            name: "my_function".to_string(),
        });

        let json = serde_json::to_value(req).unwrap();

        assert_eq!(
            json,
            serde_json::json!({
                "tool_choice":{
                    "function":{
                        "type":"function",
                        "name":"my_function"
                    }
                },
                "model": "gpt-3.5-turbo-1106",
                "messages":[
                    {
                        "role": "system",
                        "content": "I'm Q-bot.",
                        "name": "Q-bot"
                    },
                    {
                        "role": "user",
                        "content": "HI!",
                        "name": "zheng"
                    }
                ]
            })
        );
    }

    #[test]
    fn chat_choice_completion_request_serialize_should_work() {
        let mut req = get_simple_completion_request();
        req.tool_choice = Some(ToolChoice::Auto);

        let json = serde_json::to_value(req).unwrap();

        assert_eq!(
            json,
            serde_json::json!({
                "tool_choice":"auto",
                "messages":[]
            })
        );
    }

    #[tokio::test]
    async fn simple_chat_completion_should_work() -> Result<()> {
        let req = get_simple_completion_request();
        let sdk = &crate::SDK;
        let res = sdk.chat_completion(req).await?;

        let ress = res.clone();
        tokio::task::spawn_blocking(move || {
            let str = format!("{:#?}", ress.clone());
            let _ = fs::write("res.text", str);
        })
        .await
        .expect("Failed to spawn blocking task");

        assert_eq!(res.model, ChatCompleteModel::GPT3Turbo);
        assert_eq!(res.choices.len(), 1);
        let choice = &res.choices[0];
        assert_eq!(choice.finish_reason, FinishReason::Stop);
        assert_eq!(choice.index, 0);
        assert_eq!(choice.message.tool_calls.len(), 0);

        Ok(())
    }

    #[test]
    fn chat_completion_request_with_tools_serialize_should_work() {
        let req = get_tool_completion_request();
        let json = serde_json::to_value(req).unwrap();

        assert_eq!(
            json,
            serde_json::json!({
                "model": "gpt-3.5-turbo-1106",
                "messages":[
                    {
                        "role": "system",
                        "content": "I can choose the right function for you.",
                        "name": "Q-bot"
                    },
                    {
                        "role": "user",
                        "content": "What is the weather like in ShangHai?",
                        "name": "zheng"
                    }
                ],
                "tools":[
                    {
                        "type":"function",
                        "function":{
                            "name":"get_weather_forecast",
                            "description":"Get the weather forecast for a city.",
                            "parameters":GetWeatherArgs::to_schema(),
                        }
                    },
                    {
                        "type":"function",
                        "function":{
                            "name":"explain_mood",
                            "description":"Explain the meaning of the give mood.",
                            "parameters":ExplainMoodArgs::to_schema(),
                        }
                    }
                ]
            })
        );
    }

    #[tokio::test]
    async fn chat_completion_with_tools_should_work() -> Result<()> {
        let req = get_tool_completion_request();
        let sdk = &crate::SDK;
        let res = sdk.chat_completion(req).await?;

        let ress = res.clone();
        tokio::task::spawn_blocking(move || {
            let str = format!("{:#?}", ress.clone());
            fs::write("res.txt", str);
        })
        .await
        .expect("Failed to spawn blocking task");

        assert_eq!(res.model, ChatCompleteModel::GPT3Turbo);
        assert_eq!(res.choices.len(), 1);
        let choice = &res.choices[0];
        assert_eq!(choice.finish_reason, FinishReason::ToolCalls);
        assert_eq!(choice.index, 0);
        assert_eq!(choice.message.content, None);
        assert_eq!(choice.message.tool_calls.len(), 1);
        let tool_call = &choice.message.tool_calls[0];
        assert_eq!(tool_call.function.name, "get_weather_forecast");
        let ret = get_weather_forecast(serde_json::from_str(&tool_call.function.arguments)?);
        assert_eq!(ret.unit, TemperatureUnit::Celsius);
        assert_eq!(ret.temperature, 22.1);

        Ok(())
    }

    fn get_simple_completion_request() -> ChatCompletionRequest {
        let messages = vec![
            ChatCompletionMessage::new_system("I'm Q-bot.", "Q-bot"),
            ChatCompletionMessage::new_user("HI!", "zheng"),
        ];
        let req: ChatCompletionRequest = ChatCompletionRequestBuilder::default()
            // .tool_choice(ToolChoice::Auto)
            .messages(messages)
            .build()
            .unwrap();

        req
    }

    // 使用提供的tools作为回答依赖
    fn get_tool_completion_request() -> ChatCompletionRequest {
        let messages = vec![
            ChatCompletionMessage::new_system("I can choose the right function for you.", "Q-bot"),
            ChatCompletionMessage::new_user("What is the weather like in ShangHai?", "zheng"),
        ];

        let tools = vec![
            Tool::new_function::<GetWeatherArgs>(
                "get_weather_forecast",
                "Get the weather forecast for a city.",
            ),
            Tool::new_function::<ExplainMoodArgs>(
                "explain_mood",
                "Explain the meaning of the give mood.",
            ),
        ];

        let req: ChatCompletionRequest = ChatCompletionRequestBuilder::default()
            // .tool_choice(ToolChoice::Auto)
            .messages(messages)
            .tools(tools)
            .build()
            .unwrap();

        req
    }
}
