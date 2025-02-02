use futures::StreamExt;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

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
    usage: Option<Usage>,
    choices: Vec<Choice>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Choice {
    finish_reason: Option<String>,
    index: usize,
    delta: ResMessage,
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
    role: Option<Role>,
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
    completion_tokens: Option<usize>,
    prompt_tokens: usize,
    prompt_cache_hit_tokens: Option<usize>,
    promt_cache_miss_tokens: Option<usize>,
    total_tokens: usize,
    completion_tokens_details: Option<Details>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Details {
    reasoning_tokens: Option<usize>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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
            content: "介绍一下你自己".into(),
        },
    ];
    let chat = Chat {
        model: Model::Chat,
        messages,
        stream: true,
    };

    let req = Client::new()
        .post(&url)
        .bearer_auth(&api_key)
        .json(&chat)
        .timeout(Duration::from_secs(10));
    let res = req.send().await.unwrap();
    let mut stream = res.bytes_stream();
    while let Some(item) = stream.next().await {
        let item = item?;
        // let item = serde_json::from_slice::<Response>(&item)?;
        let item = std::str::from_utf8(&item)
            .unwrap()
            .strip_prefix("data: ")
            .unwrap();
        if item.eq("[DONE]") {
            break;
        }
        // println!("{:#?}", item);

        let item: Response = serde_json::from_str(item)?;
        let completion = item
            .choices
            .first()
            .unwrap()
            .delta
            .content
            .as_ref()
            .unwrap();
        print!("{}", completion);
    }
    // let res = res.text().await.unwrap();
    // println!("{}", completion);
    Ok(())
}
