#![forbid(unsafe_code)]
mod db;

use poem::{listener::TcpListener, Route, Server};
use poem_openapi::{
    param::Query,
    payload::{Json, PlainText},
    ApiResponse, Object, OpenApi, OpenApiService,
};
use sea_orm::{sqlx::Database, DatabaseConnection};

// use sea_orm::{Database, DatabaseConnection};

/// Book
#[derive(Debug, poem_openapi::Object, Clone, Eq, PartialEq)]
struct Book {
    /// Id
    #[oai(read_only)]
    id: i64,
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
enum CreateUserResponse {
    /// Returns when the user is successfully created.
    #[oai(status = 200)]
    Ok(Json<i64>),
    /// Return when locking error
    #[oai(status = 500)]
    InternalServerError,
}

struct Api<'a> {
    db: &'a DatabaseConnection
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
        self.db.
        // return Json(db::get_books());
    }

    /// Create book
    #[oai(path = "/books", method = "post")]
    async fn create_books(&self, b: Json<Book>) -> CreateUserResponse {
        let mut x = b.0;
        x.id = -1;
        // let book_id = db::add_book(x);
        match book_id {
            Some(new_id) => return CreateUserResponse::Ok(Json(new_id)),
            None => return CreateUserResponse::InternalServerError,
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let db = Database::connect("postgres://postgres:ssenol@127.0.0.1/poembooks" ).await.unwrap();
    let api_service =
        OpenApiService::new(Api {db: &db}, "Poem Bookstore Api", "1.0").server("http://localhost:3000");
    let ui = api_service.swagger_ui();
    let app = Route::new().nest("/", api_service).nest("/docs", ui);



    Server::new(TcpListener::bind("127.0.0.1:3000"))
        .run(app)
        .await;

    // Closing connection here
    db.close().await.unwrap();
    Ok(())
}
