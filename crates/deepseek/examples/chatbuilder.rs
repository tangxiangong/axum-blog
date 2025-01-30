use deepseek::{DeepSeek, chat::*};
use serde_json::json;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let deepseek = DeepSeek::from_env();
    let chat = deepseek.model(Model::Chat).preamer("你好").build().unwrap();
    let chat_str = json!(chat).to_string();
    println!("{}", chat_str);
}
