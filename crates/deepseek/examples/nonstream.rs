use anyhow::{Context, Result};
use deepseek::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    let deepseek_cli = DeepSeek::from_env().unwrap();
    let mut chat = deepseek_cli
        .model(Model::Chat)
        .preamble("You are a helpful assistant.")
        .temperature(0.0)
        .build()
        .context("Failed to build chat request")?;
    let completion = chat
        .prompt("Hello")
        .await
        .context("Failed to prompt chat")?
        .content()
        .unwrap();

    println!("Completion: {}", completion);
    Ok(())
}
