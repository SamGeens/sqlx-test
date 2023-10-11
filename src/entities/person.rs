use serde::{Deserialize, Serialize};
use crate::entities::prelude::*;


#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct Person {
    #[serde(rename = "id")]
    pub id: i32,

    #[serde(rename = "name")]
    pub name: Option<String>,

    #[serde(rename = "age")]
    pub age: Option<i32>,

    #[serde(rename = "isActive")]
    pub isactive: Option<bool>,

    #[serde(rename = "language")]
    pub language: Option<Vec<Language>>,
}
