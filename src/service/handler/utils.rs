use crate::{AppResult, model::Image};
use chrono::Local;
use tokio::{fs::File, io::AsyncWriteExt};
use uuid::Uuid;

pub async fn save_image(image: Image) -> AppResult<String> {
    let uid = Uuid::new_v4().to_string();
    let current_time = Local::now().to_string();
    let ext = image.extension();
    let path_str = format!("./data/{}-{}.{}", current_time, uid, ext);
    let mut file = File::create(&path_str).await?;
    file.write_all(image.data()).await?;
    Ok(path_str)
}
