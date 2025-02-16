#![forbid(unsafe_code)]
// use std::sync::{LazyLock, Mutex};

use std::collections::HashMap;

use sqlx::{Column, Row, ValueRef, postgres::PgRow};


use crate::Book;

// static BOOKS: LazyLock<Mutex<Vec<Book>>> = LazyLock::new(|| Mutex::new(vec![Book {
//     id: 1,
//     title: "War and Peace".to_string(),
//     author: "Tolstoy".to_string(),
//     pages: 362,
// }, Book {
//     id: 2,
//     title: "Crime and Punishment".to_string(),
//     author: "Dostoyevski".to_string(),
//     pages: 672,
// }]));

// Check here:
// https://github.com/SeaQL/sea-orm/tree/master/examples/poem_example

// use sea_orm::entity::prelude::*;

// pub fn get_books() -> Vec<Book> {
//     return BOOKS.lock().unwrap().to_vec();
// }

// pub fn add_book(mut b: Book) -> Option<i64> {
//     let books = BOOKS.lock();
//     match books {
//         Ok(mut books) => {
//             let new_id = books.iter().map(|e| e.id).max().unwrap_or(0) + 1;
//             b.id = new_id;
//             books.push(b);
//             return Some(new_id);
//         }
//         Err(e) => {
//             eprintln!("Failed to lock books: {}", e);
//             return None
//         }
//     }
// }

pub fn row_to_hashmap(row: &PgRow) -> HashMap<String, String> {
    let mut result = HashMap::new();
    for col in row.columns() {
        let value = row.try_get_raw(col.ordinal()).unwrap();
        let value = match value.is_null() {
            true => "NULL".to_string(),
            false => {
                println!("converted value: {}", value.as_str().unwrap().to_string());
                value.as_str().unwrap().to_string()},
        };
        result.insert(col.name().to_string(), value);
    }

    result
}
