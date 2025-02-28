use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UpdateWebsiteInfo {
    pub title: Option<String>,
    pub subtitle: Option<String>,
    pub description: Option<String>,
}
