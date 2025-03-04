#![forbid(unsafe_code)]

mod models;
mod routers;

use std::env;

use models::book_model::User;
use poem::{listener::TcpListener, middleware::Cors, EndpointExt, Route, Server};
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
    #[oai(status = 200)]
    Ok(Json<HeatlhStatus>),
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
                name: "test".to_string(),
            }),
        }
    }

    #[oai(path = "/health", method = "get")]
    pub async fn check_health(&self) -> HealthStatusResponse {
        HealthStatusResponse::Ok(Json(HeatlhStatus {
            status: HealthState::Healthy,
        }))
    }
}

fn get_postgresql_host() -> String {
    env::var("PG_HOST").unwrap_or("127.0.0.1".to_string())
}

fn get_host_addr() -> String {
    env::var("HOST_ADDR").unwrap_or("localhost".to_string())
}

fn get_host_port() -> String {
    env::var("HOST_PORT").unwrap_or("3000".to_string())
}

async fn get_pool() -> Option<Pool<Postgres>> {
    let mut pool: Result<Pool<Postgres>, sqlx::Error> = Err(sqlx::Error::PoolTimedOut);

    for i in 0..5 {
        if i > 0 {
            println!("Attempt {} to connect to database failed", i);
        }
        pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&format!(
                "postgres://postgres:ssenol@{}/poembooks",
                get_postgresql_host()
            ))
            .await;
        println!("pool: {pool:#?}");
    }
    
    match pool {
        Ok(p) => Some(p),
        Err(_) => None,
    } 
}

fn test_func() {
    todo!()
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
        .server(format!("http://{}:{}", get_host_addr(), get_host_port()));
    let ui = api_service.swagger_ui();
    let app = Route::new()
        .nest("/", api_service)
        .nest("/docs", ui)
        .with(Cors::new().allow_methods(vec![poem::http::Method::GET, poem::http::Method::POST]));

    Server::new(TcpListener::bind(format!("{}:{}", get_host_addr(), get_host_port())))
        .run(app)
        .await?;

    // Closing connection here
    Ok(())
}
