#![forbid(unsafe_code)]
mod db;

use poem::{listener::TcpListener, Route, Server};
use poem_openapi::{param::Query, payload::Json, payload::PlainText, OpenApi, OpenApiService};

/// Book
#[derive(Debug, poem_openapi::Object, Clone, Eq, PartialEq)]
struct Book {
    id: i64,
    title: String,
    author: String,
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

struct Api;

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
            Some(name) => Json(User { name: name }),
            None => Json(User {
                name: format!("{}", "test"),
            }),
        }
    }

    /// List books
    #[oai(path = "/books", method = "get")]
    async fn list_books(&self) -> Json<Vec<Book>> {
        // let xx: Vec<Book> = Vec::new();
        return Json(db::get_books());
    }
}

#[tokio::main]
async fn main() {
    let api_service =
        OpenApiService::new(Api, "Poem Bookstore Api", "1.0").server("http://localhost:3000");
    let ui = api_service.swagger_ui();
    let app = Route::new().nest("/", api_service).nest("/docs", ui);

    Server::new(TcpListener::bind("127.0.0.1:3000"))
        .run(app)
        .await;
}
