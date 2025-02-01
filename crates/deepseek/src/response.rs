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
    pub fn content(&self) -> String {
        self.choices
            .first()
            .unwrap()
            .message
            .content
            .clone()
            .unwrap()
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
    completion_tokens: usize,
    prompt_tokens: usize,
    prompt_cache_hit_tokens: Option<usize>,
    promt_cache_miss_tokens: Option<usize>,
    total_tokens: usize,
    completion_tokens_details: Option<Details>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Clone)]
struct Details {
    reasoning_tokens: usize,
}
