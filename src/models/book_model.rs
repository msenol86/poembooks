// use sea_orm::entity::prelude::*;

// Check here for sea ORM Model
// https://github.com/SeaQL/sea-orm/blob/master/src/tests_cfg/cake.rs
use poem_openapi::{payload::Json, ApiResponse};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

/// Book
#[derive(Debug, poem_openapi::Object, Clone, Eq, PartialEq, sqlx::FromRow)]
pub struct Book {
    /// Id
    #[oai(read_only)]
    pub id: i32,
    /// Title
    pub title: String,
    /// Author
    pub author: String,
    /// Pages
    pub pages: u16,
}

#[derive(ApiResponse)]
pub enum CreateBookResponse {
    /// Returns when the book is successfully created.
    #[oai(status = 200)]
    Ok(Json<i64>),
    /// Return when something wrong
    #[oai(status = 500)]
    InternalServerError,
}

/// Create user schema
#[derive(Debug, poem_openapi::Object, Clone, Eq, PartialEq)]
pub struct User {
    /// Id
    // #[oai(read_only)]
    // id: i64,
    /// Name
    #[oai(validator(max_length = 64))]
    pub name: String,
}

// {
//     "model": "tinyllama",
//     "prompt": "who are the possible authors of book titled \"Mother\"?", "stream": false
//   }

fn default_model() -> String {
    return "tinyllama".to_string();
}

fn default_stream() -> bool {
    return false;
}
#[derive(Debug, poem_openapi::Object, Clone, Eq, PartialEq, Serialize)]
pub struct AiRequest {
    #[oai(default = "default_model")]
    pub model: String,
    pub prompt: String,
    #[oai(default = "default_stream")]
    pub stream: bool,
}

#[derive(Debug, Deserialize, poem_openapi::Object, Clone)]
pub struct AiResponse {
    pub model: String,
    pub response: String,
    pub done: bool,
}
