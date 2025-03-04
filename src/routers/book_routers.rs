use crate::models::book_model::{
    AiRequest, AiResponse, Book, CreateBookResponse, DeleteBookResponse, GetBookResponse,
};

use sqlx::{Pool, Postgres};

use poem_openapi::{param::Path, payload::Json, types::Type, OpenApi};

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
        Json(t_books)
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
            Some(t_id) => CreateBookResponse::Ok(Json(t_id as i64)),
            None => CreateBookResponse::InternalServerError,
        }
    }

    /// Delete a book
    #[oai(path = "/books/:id", method = "delete")]
    pub async fn delete_book(&self, id: Path<i32>) -> DeleteBookResponse {
        let result: Result<Option<i32>, _> =
            sqlx::query_scalar("DELETE FROM books WHERE id = $1  RETURNING id")
                .bind(id.0)
                .fetch_optional(&self.pool)
                .await;
        match result {
            Ok(deleted_book_id) => match deleted_book_id {
                Some(t_id) => DeleteBookResponse::Ok(Json(t_id as i64)),
                None => DeleteBookResponse::NotFoundError,
            },
            Err(_) => DeleteBookResponse::InternalServerError,
        }
    }

    /// Get a book
    #[oai(path = "/books/:id", method = "get")]
    pub async fn get_book(&self, id: Path<i32>) -> GetBookResponse {
        let t_book: Option<(i32, String, String, i16)> =
            sqlx::query_as("SELECT * FROM books WHERE id = $1")
                .bind(id.0)
                .fetch_optional(&self.pool)
                .await
                .unwrap();
        match t_book {
            Some(t_book) => {
                let t_book = Book {
                    id: t_book.0,
                    title: t_book.1,
                    author: t_book.2,
                    pages: t_book.3 as u16,
                };
                GetBookResponse::Ok(Json(t_book))
            }
            None => GetBookResponse::NotFoundError,
        }
    }

    /// Check AI for Author-Title match
    #[oai(path = "/books/ai", method = "post")]
    pub async fn check_ai(&self, prompt: Json<AiRequest>) -> Json<AiResponse> {
        let ai_request = prompt.as_raw_value().unwrap();
        let client = reqwest::Client::new();
        let resp = client
            .post("http://127.0.0.1:11434/api/generate")
            .json(ai_request)
            .send()
            .await
            .unwrap();
        Json(resp.json::<AiResponse>().await.unwrap())
    }
}
