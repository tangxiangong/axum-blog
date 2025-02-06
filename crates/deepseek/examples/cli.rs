use anyhow::{Context, Result};
use deepseek::prelude::*;
use dotenvy::dotenv;
use std::io;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let deepseek = DeepSeek::from_env().expect("Failed to create DeepSeek client");
    let mut chat = deepseek
        .model(Model::Chat)
        .temperature(0.0)
        .build()
        .context("Failed to build chat request")?;
    let mut history = vec![Message::system(
        "你是一个AI助手，请根据用户的问题给出回答。",
    )];
    loop {
        let mut input = String::new();
        print!("User：");
        io::stdin()
            .read_line(&mut input)
            .context("Failed to read line")?;
        let input = input.trim();
        if input.is_empty() {
            break;
        }
        println!();
        let message = Message::user(input);
        let response = chat
            .with_history(&history)
            .prompt(message.content())
            .await
            .context("Failed to prompt chat")?
            .content()
            .unwrap();
        println!("Assistant：{}", response);
        history.push(Message::user(input));
        history.push(Message::assistant(response));
    }
    Ok(())
}
