use crate::models::book_model::{AiRequest, Book, CreateBookResponse, AiResponse};

use sqlx::{Pool, Postgres};

use poem_openapi::{payload::Json, types::Type, OpenApi};



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

    /// Check AI for Author-Title match
    #[oai(path = "/books/ai", method = "post")]
    pub async fn check_ai(&self, prompt: Json<AiRequest>) -> Json<AiResponse> {
        let xxx = prompt.as_raw_value().unwrap();
        let client = reqwest::Client::new();
        let resp = client.post("http://127.0.0.1:11434/api/generate").json(xxx).send().await.unwrap().json::<AiResponse>().await;
        match resp {
            Ok(ip) => {
                println!("resp: {:#?}", ip);
                return Json(ip);
            }
            Err(e) => {
                println!("Error: {}", e);
                return Json(AiResponse{model: "error".to_string(), response: "error".to_string(), done: false});
            }
        }
        // println!("resp: {:#?}", resp);
        // let ip = resp.json::<Ip>().await.unwrap();
            // .await
            // .unwrap()
            // .json::<Ip>()
            // .await
            // .unwrap();
        // println!("ip: {:#?}", ip);
        
    }
}
