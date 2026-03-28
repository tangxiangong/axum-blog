use serde::{Deserialize, Serialize};
use surrealdb::types::Datetime;

#[derive(Debug, Deserialize, Serialize)]
pub struct CommentForm {
    pub article_id: u32,
    pub name: String,
    pub email: Option<String>,
    pub content: String,
    pub parent_id: Option<u32>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CommentQuery {
    pub page: Option<u64>,
    pub page_size: Option<u64>,
    pub article_id: Option<u32>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CommentInfo {
    pub id: u32,
    pub article_id: u32,
    pub name: String,
    pub email: Option<String>,
    pub content: String,
    pub parent_id: u32,
    pub created_at: Datetime,
    pub children: Vec<CommentInfo>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CommentTree {
    pub id: u32,
    pub article_id: u32,
    pub name: String,
    pub email: Option<String>,
    pub content: String,
    pub parent_id: u32,
    pub created_at: Datetime,
    pub children: Vec<CommentInfo>,
}
