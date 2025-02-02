use std::time::Duration;

use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
struct Message {
    role: Role,
    content: String,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
enum Role {
    System,
    User,
    Assistant,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, PartialEq, Eq)]
enum Model {
    #[serde(rename = "deepseek-chat")]
    Chat,
    #[serde(rename = "deepseek-reasoner")]
    Reasoner,
}

#[derive(Debug, Serialize)]
struct Chat {
    model: Model,
    messages: Vec<Message>,
    stream: bool,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Response {
    id: String,
    created: i64,
    system_fingerprint: Option<String>,
    object: String,
    usage: Usage,
    choices: Vec<Choice>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Choice {
    finish_reason: String,
    index: usize,
    message: ResMessage,
    logprobs: Option<Logprobs>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Logprobs {
    content: Option<Vec<Content>>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Content {
    token: String,
    logprob: f64,
    bytes: Option<Vec<u8>>,
    top_logprobs: Vec<TopLogprobs>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct TopLogprobs {
    tokens: String,
    logprob: f64,
    bytes: Option<Vec<u8>>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct ResMessage {
    content: Option<String>,
    reasoning_content: Option<String>,
    tool_calls: Option<Vec<Calls>>,
    role: Role,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Calls {
    id: String,
    #[serde(rename = "type")]
    type_: String,
    function: Function,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Function {
    name: String,
    arguments: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Usage {
    completion_tokens: usize,
    prompt_tokens: usize,
    prompt_cache_hit_tokens: Option<usize>,
    promt_cache_miss_tokens: Option<usize>,
    total_tokens: usize,
    completion_tokens_details: Option<Details>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Details {
    reasoning_tokens: usize,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let api_key = std::env::var("DEEPSEEK_API_KEY").expect("API KEY 未在环境变量中设置");
    let base_url = std::env::var("DEEPSEEK_BASE_URL").expect("BASE URL 未在环境变量中设置");
    let url = format!("{}/v1/chat/completions", base_url);

    let messages = vec![
        Message {
            role: Role::System,
            content: "You are a helpful assistant".into(),
        },
        Message {
            role: Role::User,
            content: "Hello".into(),
        },
    ];
    let chat = Chat {
        model: Model::Chat,
        messages,
        stream: false,
    };

    let req = Client::new()
        .post(&url)
        .bearer_auth(&api_key)
        .json(&chat)
        .timeout(Duration::from_secs(10));
    let res = req.send().await.unwrap();
    let res: Response = res.json().await.unwrap();
    let completion = res
        .choices
        .first()
        .unwrap()
        .message
        .content
        .as_ref()
        .unwrap();
    // let res = res.text().await.unwrap();
    println!("{}", completion);
}
