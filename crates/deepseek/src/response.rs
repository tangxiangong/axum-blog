use crate::prelude::{Model, Role};
use serde::Deserialize;

#[allow(dead_code)]
#[derive(Debug, Deserialize, Clone)]
pub struct Response {
    id: String,
    created: i64,
    system_fingerprint: Option<String>,
    object: String,
    usage: Usage,
    choices: Vec<Choice>,
    model: Model,
}

impl Response {
    pub fn content(&self) -> Option<String> {
        self.choices.first().unwrap().message.content.clone()
    }

    pub fn reasoning_content(&self) -> Option<String> {
        self.choices
            .first()
            .unwrap()
            .message
            .reasoning_content
            .clone()
    }

    pub fn completion_tokens(&self) -> usize {
        self.usage.completion_tokens
    }

    pub fn prompt_tokens(&self) -> usize {
        self.usage.prompt_tokens
    }

    pub fn total_tokens(&self) -> usize {
        self.usage.total_tokens
    }

    pub fn reasoning_tokens(&self) -> Option<usize> {
        self.usage
            .completion_tokens_details
            .as_ref()
            .map(|details| details.reasoning_tokens)
    }
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Clone)]
struct Choice {
    finish_reason: String,
    index: usize,
    message: ResMessage,
    logprobs: Option<Logprobs>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Clone)]
struct Logprobs {
    content: Option<Vec<Content>>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Clone)]
struct Content {
    token: String,
    logprob: f64,
    bytes: Option<Vec<u8>>,
    top_logprobs: Vec<TopLogprobs>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Clone)]
struct TopLogprobs {
    tokens: String,
    logprob: f64,
    bytes: Option<Vec<u8>>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Clone)]
struct ResMessage {
    content: Option<String>,
    reasoning_content: Option<String>,
    tool_calls: Option<Vec<Calls>>,
    role: Role,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Clone)]
struct Calls {
    id: String,
    r#type: String,
    function: Function,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Clone)]
struct Function {
    name: String,
    arguments: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Clone)]
pub(crate) struct Usage {
    pub(crate) completion_tokens: usize,
    pub(crate) prompt_tokens: usize,
    pub(crate) prompt_cache_hit_tokens: Option<usize>,
    pub(crate) promt_cache_miss_tokens: Option<usize>,
    pub(crate) total_tokens: usize,
    completion_tokens_details: Option<Details>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Clone)]
pub(crate) struct Details {
    pub(crate) reasoning_tokens: usize,
}
