use crate::models::book_model::{Book, CreateBookResponse};

use sqlx::{Pool, Postgres};

use poem_openapi::{
    payload::Json,
    OpenApi,
};

pub struct BooksEndpoints {
    pub pool: Pool<Postgres>,
}


#[OpenApi]
impl BooksEndpoints {
    /// List books
    #[oai(path = "/books", method = "get")]
    pub async fn list_books(&self) -> Json<Vec<Book>> {
        let t_books: Vec<(i32, String, String, i16)> = sqlx::query_as("SELECT * FROM books")
            .fetch_all(&self.pool)
            .await
            .unwrap();
        // println!("t_books: {t_books:#?}");
        let t_books: Vec<Book> = t_books
            .iter()
            .map(|e| Book {
                id: e.0,
                title: e.1.clone(),
                author: e.2.clone(),
                pages: e.3 as u16,
            })
            .collect();
        return Json(t_books);
    }

    /// Create book
    #[oai(path = "/books", method = "post")]
    pub async fn create_books(&self, b: Json<Book>) -> CreateBookResponse {
        let x: Option<i32> = sqlx::query_scalar(
            "INSERT INTO books (title, author, pages) VALUES ($1, $2, $3) RETURNING id",
        )
        .bind(b.title.clone())
        .bind(b.author.clone())
        .bind(b.pages as i16)
        .fetch_optional(&self.pool)
        .await
        .unwrap();

        match x {
            Some(t_id) => return CreateBookResponse::Ok(Json(t_id as i64)),
            None => return CreateBookResponse::InternalServerError,
        }
    }
}
