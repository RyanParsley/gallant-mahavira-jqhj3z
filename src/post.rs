use super::author::Author;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Post {
    pub id: i64,
    pub title: String,
    pub description: String,
    pub body: String,
    pub author: Author,
    pub category: Option<String>
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponsePost {
    pub id: i64,
    pub title: String,
    pub description: String,
    pub body: String,
    pub author: Author,
}
