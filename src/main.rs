#![forbid(unsafe_code)]
mod db;

use std::{collections::HashMap, fmt::Debug, ops::Deref};

use db::row_to_hashmap;
use sqlx::{postgres::PgPoolOptions, Postgres, Pool};
use poem::{listener::TcpListener, Route, Server};
use poem_openapi::{
    param::Query,
    payload::{Json, PlainText},
    ApiResponse, Object, OpenApi, OpenApiService,
};
use sqlx::Row;

// use sea_orm::{Database, DatabaseConnection};

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
enum CreateUserResponse {
    /// Returns when the user is successfully created.
    #[oai(status = 200)]
    Ok(Json<i64>),
    /// Return when locking error
    #[oai(status = 500)]
    InternalServerError,
}

struct Api {
    pool: Pool<Postgres>
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
        let results = sqlx::query(
            // Notice how we only have to bind the argument once and we can use it multiple times:
            "SELECT * FROM books where id=$1"
        )
        .bind(1)
        .fetch_all(&self.pool)
        .await.unwrap();
        // print!("Row: {:#?}", results);


        let t_book: (i32, String, String, i16) = sqlx::query_as("SELECT * FROM books where id=$1").bind(1).fetch_one(&self.pool).await.unwrap();
        // let x = results.iter().map(|e| format!("{:#?}", e));
        // for a in x {
        //     println!("Value: {a}");
        // }
        // let x = results.get(0).unwrap().columns();
        // let id: i32 = results.get(0).unwrap().get(0);
        // let x: Vec<HashMap<String, String>> = results.iter().map(|e| row_to_hashmap(e)).collect();
        // let y = x.get(0).unwrap();
        // let tmp_str = "Test".to_string();
        // print!("result: {:#?}", y);
        // let id: i64= y.get("id").unwrap().parse().unwrap();
        // let title= y.get("title").unwrap_or(&tmp_str);
        // let author= y.get("author").unwrap_or(&tmp_str);
        // println!("id: {id} title: {title}");
        // let pages: u16= y.get("pages").unwrap_or(&"200".to_string()).parse().unwrap_or(1);
        println!("t_book: {t_book:#?}");
        return Json(vec![Book{id: t_book.0, title: t_book.1, author: t_book.2, pages: t_book.3 as u16}]);
        // return Json(vec![Book{id: 1, title: "Test".to_string(), author: "Test".to_string(), pages: 172}]);
    }

    // #[oai(path = "/books", method = "post")]
    // async fn create_books(&self, b: Json<Book>) -> CreateUserResponse {
    //     let mut x = b.0;
    //     x.id = -1;
    //     // let book_id = db::add_book(x);
    //     match book_id {
    //         Some(new_id) => return CreateUserResponse::Ok(Json(new_id)),
    //         None => return CreateUserResponse::InternalServerError,
    //     }
    // }
}

#[tokio::main]
async fn main() {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:ssenol@127.0.0.1/poembooks")
        .await.unwrap();
    let api_service =
        OpenApiService::new(Api {pool: pool}, "Poem Bookstore Api", "1.0").server("http://localhost:3000");
    let ui = api_service.swagger_ui();
    let app = Route::new().nest("/", api_service).nest("/docs", ui);



    Server::new(TcpListener::bind("127.0.0.1:3000"))
        .run(app)
        .await;

    // Closing connection here
    // Ok(())
}
