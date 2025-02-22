#![forbid(unsafe_code)]

mod models;
mod routers;

use std::env;

use models::book_model::User;
use poem::{listener::TcpListener, Route, Server};
use poem_openapi::{
    param::Query,
    payload::{Json, PlainText},
    ApiResponse, OpenApi, OpenApiService,
};
use routers::book_routers::BooksEndpoints;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub struct UserEndpoints {
    pool: Pool<Postgres>,
}

#[derive(Debug, poem_openapi::Enum, Clone, Eq, PartialEq)]
pub enum HealthState {
    #[oai(rename = "healthy")]
    Healthy,
    #[oai(rename = "not_healthy")]
    NotHealthy,
}

#[derive(Debug, poem_openapi::Object, Clone, Eq, PartialEq)]
pub struct HeatlhStatus {
    pub status: HealthState,
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
// https://github.com/poem-web/poem/blob/master/examples/openapi/todos/src/main.rs

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
            status: HealthState::Healthy,
        }));
    }
}

pub async fn get_postgresql_host() -> String {
    return env::var("PG_HOST").unwrap_or("127.0.0.1".to_string());
}

async fn get_pool() -> Option<Pool<Postgres>> {
    let mut pool: Result<Pool<Postgres>, sqlx::Error> = Err(sqlx::Error::PoolTimedOut);

    for i in 0..5 {
        if i > 0 {
            println!("Attempt {} to connect to database failed", i);
        }
        pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&format!("postgres://postgres:ssenol@{}/poembooks", get_postgresql_host().await))
            .await;
        println!("pool: {pool:#?}");
    }
    if pool.is_err() {
        return None;
    } else {
        return Some(pool.unwrap());
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pool = get_pool().await;
    if pool.is_none() {
        println!("Failed to connect to database");
        return Err(Box::from("Failed to connect to database"));
    }
    let pool = pool.unwrap();

    let all_endpoints = (
        UserEndpoints { pool: pool.clone() },
        BooksEndpoints { pool: pool.clone() },
    );

    let api_service = OpenApiService::new(all_endpoints, "Poem Bookstore Api", "1.0")
        .server("http://0.0.0.0:3000");
    let ui = api_service.swagger_ui();
    let app = Route::new().nest("/", api_service).nest("/docs", ui);

    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(app)
        .await?;

    // Closing connection here
    Ok(())
}
