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

#[derive(Debug, Deserialize)]
pub struct IdQuery {
    pub id: u32,
}

#[derive(Debug, Deserialize)]
pub struct NameQuery {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct ParentIdQuery {
    pub parent_id: u32,
}
