use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ArticleForm {
    pub title: String,
    pub summary: Option<String>,
    pub content: String,
    pub publish: i8,
    pub category_ids: Vec<u32>,
    pub tag_ids: Vec<u32>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ArticleQuery {
    pub page: Option<u64>,
    pub page_size: Option<u64>,
    pub keyword: Option<String>,
    pub publish: Option<i8>,
    pub category_id: Option<u32>,
    pub tag_id: Option<u32>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ArticleDetail {
    pub id: u32,
    pub title: String,
    pub summary: Option<String>,
    pub content: String,
    pub views: u32,
    pub publish: i8,
    pub created_at: DateTimeLocal,
    pub updated_at: DateTimeLocal,
    pub categories: Vec<CategoryInfo>,
    pub tags: Vec<TagInfo>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ArticleInfo {
    pub id: u32,
    pub title: String,
    pub summary: Option<String>,
    pub views: u32,
    pub publish: i8,
    pub created_at: DateTimeLocal,
    pub updated_at: DateTimeLocal,
    pub categories: Vec<CategoryInfo>,
    pub tags: Vec<TagInfo>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CategoryInfo {
    pub id: u32,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TagInfo {
    pub id: u32,
    pub name: String,
}
