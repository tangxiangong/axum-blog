use anyhow::{Context, Result};
use deepseek::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    let deepseek_cli = DeepSeek::from_env();
    let mut chat = deepseek_cli
        .model(Model::Reasoner)
        .preamer("You are a helpful assistant.")
        .temperature(0.0)
        .build()
        .context("Failed to build chat request")?;
    let completion = chat
        .prompt("单词 strawberry 中有几个字母 r?")
        .await
        .context("Failed to prompt chat")?
        .content()
        .unwrap();

    println!("Completion: {}", completion);
    Ok(())
}
