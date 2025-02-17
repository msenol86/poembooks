#![forbid(unsafe_code)]

mod models;
mod routers;

use models::book_model::User;
use poem::{listener::TcpListener, Route, Server};
use poem_openapi::{
    param::Query,
    payload::{Json, PlainText},
    ApiResponse, OpenApi, OpenApiService,
};
use routers::book_routers::BooksEndpoints;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

struct UserEndpoints {
    pool: Pool<Postgres>,
}

// #[derive(Debug, Clone, Eq, PartialEq)]
// enum HealthState {
//     Healthy,
//     NotHealthy,
// }

#[derive(Debug, poem_openapi::Object, Clone, Eq, PartialEq)]
struct HeatlhStatus {
    pub status: String,
}

#[derive(ApiResponse)]
pub enum HealthStatusResponse {
    /// Returns when the book is successfully created.
    #[oai(status = 200)]
    Ok(Json<HeatlhStatus>),
    /// Return when something wrong
    #[oai(status = 500)]
    InternalServerError,
}

// Check here:
// https://github.com/poem-web/poem/blob/master/examples/openapi/users-crud/src/main.rs

#[OpenApi]
impl UserEndpoints {
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

    #[oai(path = "/health", method = "get")]
    pub async fn check_health(&self) -> HealthStatusResponse {
        return HealthStatusResponse::Ok(Json(HeatlhStatus {
            status: "healthy".to_string(),
        }));
    }
}

#[tokio::main]
async fn main() {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:ssenol@127.0.0.1/poembooks")
        .await
        .unwrap();
    let all_endpoints = (
        UserEndpoints { pool: pool.clone() },
        BooksEndpoints { pool: pool.clone() },
    );
    let api_service = OpenApiService::new(all_endpoints, "Poem Bookstore Api", "1.0")
        .server("http://127.0.0.1:3000");
    let ui = api_service.swagger_ui();
    let app = Route::new().nest("/", api_service).nest("/docs", ui);

    Server::new(TcpListener::bind("127.0.0.1:3000"))
        .run(app)
        .await;

    // Closing connection here
    // Ok(())
}
