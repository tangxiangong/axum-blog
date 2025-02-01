use crate::prelude::{Model, Role};
use serde::Deserialize;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct StreamResponse {
    id: String,
    created: i64,
    system_fingerprint: Option<String>,
    object: String,
    choices: Vec<StreamChoice>,
    model: Model,
}

impl StreamResponse {
    pub fn delta_content(&self) -> String {
        self.choices.first().unwrap().delta.content.clone().unwrap()
    }
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
    role: Role,
}
