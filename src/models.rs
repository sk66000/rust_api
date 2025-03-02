use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    pub id: i32,
    pub name: String,
    pub description: Option <String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateItem {
    pub name: String,
    pub description: Option <String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateItem {
    pub name: Option <String>,
    pub description: Option <String>,
}
