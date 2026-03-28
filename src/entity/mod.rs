use serde::{Deserialize, Serialize};
use surrealdb::types::{Datetime, SurrealValue};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, SurrealValue)]
pub struct Admin {
    pub id: String,
    pub name: String,
    pub password: Vec<u8>,
    pub nickname: Option<String>,
    pub email: Option<String>,
    pub github: Option<String>,
    pub wechat: Option<String>,
    pub qq: Option<String>,
    pub avatar: Option<String>,
    pub created_at: Datetime,
    pub updated_at: Datetime,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, SurrealValue)]
pub struct Article {
    pub id: u32,
    pub title: String,
    pub summary: Option<String>,
    pub content: String,
    pub views: u32,
    pub publish: i8,
    pub created_at: Datetime,
    pub updated_at: Datetime,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, SurrealValue)]
pub struct ArticleCategory {
    pub article_id: u32,
    pub category_id: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, SurrealValue)]
pub struct ArticleTag {
    pub article_id: u32,
    pub tag_id: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, SurrealValue)]
pub struct Category {
    pub id: u32,
    pub name: String,
    pub parent_id: Option<u32>,
    pub created_at: Datetime,
    pub updated_at: Datetime,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, SurrealValue)]
pub struct Comment {
    pub id: u32,
    pub article: u32,
    pub name: String,
    pub email: Option<String>,
    pub parent_id: u32,
    pub created_at: Datetime,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, SurrealValue)]
pub struct Jwt {
    pub id: u32,
    pub user_uuid: String,
    pub token: String,
    pub expire_at: i64,
    pub expire_duration: i64,
    pub created_at: Datetime,
    pub updated_at: Datetime,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, SurrealValue)]
pub struct Tag {
    pub id: u32,
    pub name: String,
    pub created_at: Datetime,
    pub updated_at: Datetime,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, SurrealValue)]
pub struct Website {
    pub title: String,
    pub subtitle: Option<String>,
    pub description: Option<String>,
    pub logo: Option<String>,
    pub favicon: Option<String>,
    pub created_at: Datetime,
    pub updated_at: Datetime,
}
