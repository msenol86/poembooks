#![forbid(unsafe_code)]
use std::sync::{LazyLock, Mutex};

use crate::Book;


static BOOKS: LazyLock<Mutex<Vec<Book>>> = LazyLock::new(|| Mutex::new(vec![Book {
    id: 1,
    title: "War and Peace".to_string(),
    author: "Tolstoy".to_string(),
    pages: 362,
}, Book {
    id: 2,
    title: "Crime and Punishment".to_string(),
    author: "Dostoyevski".to_string(),
    pages: 672,
}]));


pub fn get_books() -> Vec<Book> {
    return BOOKS.lock().unwrap().to_vec();
}

pub fn add_book(b: Book) {
    BOOKS.lock().unwrap().push(b);
}
