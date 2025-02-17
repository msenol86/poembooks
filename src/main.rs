#![forbid(unsafe_code)]
use std::fmt::Debug;

use poem::{listener::TcpListener, Route, Server};
use poem_openapi::{
    param::Query,
    payload::{Json, PlainText},
    ApiResponse, OpenApi, OpenApiService,
};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};


/// Book
#[derive(Debug, poem_openapi::Object, Clone, Eq, PartialEq, sqlx::FromRow)]
struct Book {
    /// Id
    #[oai(read_only)]
    id: i32,
    /// Title
    title: String,
    /// Author
    author: String,
    /// Pages
    pages: u16,
}

/// Create user schema
#[derive(Debug, poem_openapi::Object, Clone, Eq, PartialEq)]
struct User {
    /// Id
    // #[oai(read_only)]
    // id: i64,
    /// Name
    #[oai(validator(max_length = 64))]
    name: String,
}

#[derive(ApiResponse)]
enum CreateBookResponse {
    /// Returns when the book is successfully created.
    #[oai(status = 200)]
    Ok(Json<i64>),
    /// Return when something wrong
    #[oai(status = 500)]
    InternalServerError,
}

struct Api {
    pool: Pool<Postgres>,
}

// Check here:
// https://github.com/poem-web/poem/blob/master/examples/openapi/users-crud/src/main.rs

#[OpenApi]
impl Api {
    /// Hello world
    #[oai(path = "/", method = "get")]
    async fn index(&self) -> PlainText<&'static str> {
        PlainText("Hello World")
    }

    /// Hello world with name input
    #[oai(path = "/name", method = "get")]
    async fn greet_with_name(&self, p_name: Query<Option<String>>) -> Json<User> {
        match p_name.0 {
            Some(name) => Json(User { name }),
            None => Json(User {
                name: format!("{}", "test"),
            }),
        }
    }

    /// List books
    #[oai(path = "/books", method = "get")]
    async fn list_books(&self) -> Json<Vec<Book>> {
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
    async fn create_books(&self, b: Json<Book>) -> CreateBookResponse {
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

#[tokio::main]
async fn main() {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:ssenol@127.0.0.1/poembooks")
        .await
        .unwrap();
    let api_service = OpenApiService::new(Api { pool: pool }, "Poem Bookstore Api", "1.0")
        .server("http://localhost:3000");
    let ui = api_service.swagger_ui();
    let app = Route::new().nest("/", api_service).nest("/docs", ui);

    Server::new(TcpListener::bind("127.0.0.1:3000"))
        .run(app)
        .await;

    // Closing connection here
    // Ok(())
}
