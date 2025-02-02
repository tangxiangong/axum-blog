use crate::{
    prelude::{Error, Model, Role},
    response::Usage,
};
use bytes::Bytes;
use futures::{Stream, StreamExt};
use reqwest::Result as HttpResult;
use serde::Deserialize;
// use std::time::Duration;
use std::{ops::Deref, pin::Pin};
// use tokio::time;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct BasicStreamResponse {
    id: String,
    created: i64,
    system_fingerprint: Option<String>,
    object: String,
    choices: Vec<StreamChoice>,
    model: Model,
    usage: Option<Usage>,
}

impl BasicStreamResponse {
    pub fn delta_content(&self) -> Option<String> {
        match self.choices.first() {
            Some(choice) => choice.delta.content.clone(),
            None => None,
        }
    }

    pub(crate) fn reasoning_content(&self) -> Option<String> {
        self.choices.first().unwrap().delta.resoning_content.clone()
    }

    pub(crate) fn usage(&self) -> Option<Usage> {
        self.usage.clone()
    }
}

pub(crate) type HttpByteStream = Pin<Box<dyn Stream<Item = HttpResult<Bytes>>>>;
pub struct ByteStream(HttpByteStream);

pub struct StreamResponse(pub Vec<BasicStreamResponse>);

impl Deref for StreamResponse {
    type Target = Vec<BasicStreamResponse>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl StreamResponse {
    fn usage(&self) -> Option<Usage> {
        self.0.last().unwrap().usage()
    }

    pub fn resoning_content(&self) -> Option<String> {
        self.0.first().unwrap().reasoning_content()
    }

    pub fn total_tokens(&self) -> Result<usize, Error> {
        match self.usage() {
            Some(usage) => Ok(usage.total_tokens),
            None => Err(Error::ConfigError(
                "未配置 `stream_options`, 无法获取token 使用统计".to_string(),
            )),
        }
    }
}

impl ByteStream {
    pub fn new(stream: impl Stream<Item = HttpResult<Bytes>> + 'static) -> Self {
        Self(Box::pin(stream))
    }

    pub async fn assemble(&mut self) -> Result<StreamResponse, Error> {
        let mut responses = Vec::new();
        let stream = &mut self.0;
        while let Some(chunk) = stream.next().await {
            let chunk = chunk?;
            let chunk = std::str::from_utf8(&chunk)?.strip_prefix("data: ");
            let chunk = match chunk {
                Some(chunk) => chunk.trim(),
                None => continue,
            };
            if chunk.is_empty() || chunk == "[DONE]" {
                break;
            }
            let res = serde_json::from_str::<BasicStreamResponse>(chunk)?;
            responses.push(res);
        }
        Ok(StreamResponse(responses))
    }

    pub async fn stream_to_stdout(&mut self) -> Result<String, Error> {
        let mut content = String::new();
        let stream = &mut self.0;
        while let Some(chunk) = stream.next().await {
            let chunk = chunk?;
            let chunk_str = std::str::from_utf8(&chunk)?;
            let chunk = chunk_str.strip_prefix("data: ");
            let chunk = match chunk {
                Some(chunk) => chunk.trim(),
                None => continue,
            };
            if chunk.is_empty() || chunk == "[DONE]" {
                break;
            }
            let res = serde_json::from_str::<BasicStreamResponse>(chunk)?;
            let delta_content = res.delta_content().unwrap_or_default();
            print!("{}", delta_content);
            std::io::Write::flush(&mut std::io::stdout())?;
            content.push_str(&delta_content);
        }
        println!();
        Ok(content)
    }

    // pub async fn keep_alive(&mut self) -> Result<(), Error> {
    //     let mut interval = time::interval(Duration::from_secs(15));

    //     loop {
    //         interval.tick().await;
    //         // 由于 Stream 不支持发送数据，我们改为返回一个信号，由外层处理心跳
    //         return Ok(());
    //     }
    // }
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct StreamChoice {
    finish_reason: Option<String>,
    index: usize,
    delta: StreamMessage,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct StreamMessage {
    content: Option<String>,
    resoning_content: Option<String>,
    role: Option<Role>,
}
