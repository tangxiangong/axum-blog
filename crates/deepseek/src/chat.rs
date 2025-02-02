//! 对话补全
//! 根据输入的上下文，来让模型补全对话内容

use crate::{prelude::*, stream::ByteStream};
use derive_builder::Builder;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Builder)]
#[builder(pattern = "mutable")]
pub struct Chat {
    #[serde(skip)]
    pub cli: DeepSeek,
    /// 对话消息列表
    messages: Vec<Message>,
    /// 使用的模型的 ID: deepseek-chat, deepseek-reasoner
    model: Model,
    /// 介于 -2.0 和 2.0 之间的数字。如果该值为正，那么新 token 会根据其在已有文本中的出现频率受到相应的惩罚，降低模型重复相同内容的可能性。默认为 0
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(strip_option), default)]
    frequency_penalty: Option<f64>,
    /// 介于 1 到 8192 间的整数，限制一次请求中模型生成 completion 的最大 token 数。输入 token 和输出 token 的总长度受模型的上下文长度的限制。默认使用 4096
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(strip_option), default)]
    max_tokeons: Option<usize>,
    /// 介于 -2.0 和 2.0 之间的数字。如果该值为正，那么新 token 会根据其是否已在已有文本中出现受到相应的惩罚，从而增加模型谈论新主题的可能性。默认为 0
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(strip_option), default)]
    presence_penalty: Option<f64>,
    /// 指定默认必须输出的格式, 设置为 { "type": "json_object" } 以启用 JSON 模式，该模式保证模型生成的消息是有效的 JSON。默认为 { "type": "plain_text" }
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(strip_option), default)]
    response_format: Option<ResponseFormat>,
    /// 一个 string 或最多包含 16 个 string 的 list，在遇到这些词时，API 将停止生成更多的 token。
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(strip_option, into), default)]
    stop: Option<Vec<String>>,
    /// 如果设置为 True，将会以 SSE（server-sent events）的形式以流式发送消息增量。消息流以 data: [DONE] 结尾。
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(strip_option), default)]
    stream: Option<bool>,
    /// 流式输出相关选项。只有在 `stream` 参数为 `true` 时，才可设置此参数。
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(strip_option), default)]
    stream_options: Option<StreamOptions>,
    /// 采样温度，介于 0 和 2 之间。更高的值，如 0.8，会使输出更随机，而更低的值，如 0.2，会使其更加集中和确定。 我们通常建议可以更改这个值或者更改 top_p，但不建议同时对两者进行修改。 默认为 1.0
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(strip_option), default)]
    temperature: Option<f64>,
    /// 介于 0 和 1 之间。 作为调节采样温度的替代方案，模型会考虑前 top_p 概率的 token 的结果。所以 0.1 就意味着只有包括在最高 10% 概率中的 token 会被考虑。 我们通常建议修改这个值或者更改 temperature，但不建议同时对两者进行修改。默认为 1
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(strip_option), default)]
    top_p: Option<f64>,
    /// 是否返回所输出 token 的对数概率。如果为 true，则在 message 的 content 中返回每个输出 token 的对数概率。默认为 false
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(strip_option), default)]
    logprobs: Option<bool>,
    /// 一个介于 0 到 20 之间的整数 N，指定每个输出位置返回输出概率 top N 的 token，且返回这些 token 的对数概率。指定此参数时，logprobs 必须为 true。
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(strip_option), default)]
    top_logprobs: Option<usize>,
}

impl Chat {
    async fn send(&mut self) -> Result<reqwest::Response, Error> {
        let url = format!("{}/v1/chat/completions", self.cli.base_url);
        let res = Client::new()
            .post(&url)
            .bearer_auth(&self.cli.api_key)
            .json(self)
            .timeout(Duration::from_secs(120))
            .send()
            .await?
            .error_for_status()?;
        Ok(res)
    }

    pub async fn stream_prompt(&mut self, content: &str) -> Result<ByteStream, Error> {
        let message = Message::user(content);
        self.messages.push(message);
        let res = self.send().await?;
        let stream = res.bytes_stream();
        Ok(ByteStream::new(stream))
    }

    pub async fn prompt(&mut self, content: &str) -> Result<Response, Error> {
        let message = Message::user(content);
        self.messages.push(message);
        let res: Response = self.send().await?.json().await?;
        Ok(res)
    }
}

impl ChatBuilder {
    pub fn preamer(&mut self, content: &str) -> &mut Self {
        let message = Message::system(content);
        self.messages = Some(vec![message]);
        self
    }
}

/// 对话消息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    /// 消息的内容
    content: String,
    /// 该消息的发起角色 system, user, assistant
    role: Role,
    /// 可以选填的参与者的名称，为模型提供信息以区分相同角色的参与者。
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
}

impl Message {
    pub fn new(content: impl Into<String>, role: Role, name: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            role,
            name: Some(name.into()),
        }
    }

    pub fn system(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            role: Role::System,
            name: None,
        }
    }

    pub fn user(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            role: Role::User,
            name: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    System,
    User,
    Assistant,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Model {
    #[serde(rename = "deepseek-chat")]
    Chat,
    #[serde(rename = "deepseek-reasoner")]
    Reasoner,
}

#[derive(Debug, Clone, Serialize)]
pub struct ResponseFormat {
    #[serde(rename = "type")]
    format_type: FormatType,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum FormatType {
    #[serde(rename = "json_object")]
    JsonObject,
    #[serde(rename = "text")]
    Text,
}

#[derive(Debug, Clone, Serialize)]
pub struct StreamOptions {
    /// 如果设置为 true，在流式消息最后的 data: [DONE] 之前将会传输一个额外的块。此块上的 usage 字段显示整个请求的 token 使用统计信息，而 choices 字段将始终是一个空数组。所有其他块也将包含一个 usage 字段，但其值为 null。
    include_usage: bool,
}
