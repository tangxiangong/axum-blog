use anyhow::{Context, Result};
use deepseek::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    let deepseek_cli = DeepSeek::from_env().unwrap();
    let mut chat = deepseek_cli
        .model(Model::Reasoner)
        .preamble("You are a helpful assistant.")
        .temperature(0.0)
        .stream(true)
        .build()
        .context("Failed to build chat request")?;
    let _ = chat
        .stream_prompt("hello")
        .await
        .context("Failed to prompt chat")?
        .stream_to_stdout()
        .await
        .context("Failed to stream to stdout")?;
    println!();
    Ok(())
}
