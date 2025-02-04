use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreateCategory {
    pub name: String,
    pub parent_id: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateCategory {
    pub id: u32,
    pub name: Option<String>,
    pub parent_id: Option<u32>,
}
