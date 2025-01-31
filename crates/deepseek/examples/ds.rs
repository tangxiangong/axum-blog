use anyhow::{Context, Result};
use deepseek::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    let deepseek_cli = DeepSeek::from_env();
    let mut chat = deepseek_cli
        .model(Model::Chat)
        .preamer("You are a helpful assistant.")
        .temperature(0.0)
        .build()
        .context("Failed to build chat request")?;
    let completion = chat
        .prompt("请利用 C++ 的模版偏特化和 SFINAE 技术实现判断模版参数是否为实数的 type trait，用于 concept 的约束。")
        // .prompt("hello")
        .await
        .context("Failed to prompt chat")?;

    println!("Completion: {}", completion);
    Ok(())
}
