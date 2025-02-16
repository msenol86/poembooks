use sqlx::postgres::PgPoolOptions;
// use sqlx::mysql::MySqlPoolOptions;
// etc.

#[tokio::main] // Requires the `attributes` feature of `async-std`
               // or #[tokio::main]
               // or #[actix_web::main]
async fn main() -> Result<(), sqlx::Error> {
    // Create a connection pool
    //  for MySQL/MariaDB, use MySqlPoolOptions::new()
    //  for SQLite, use SqlitePoolOptions::new()
    //  etc.
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:ssenol@127.0.0.1/poembooks")
        .await?;

    // Make a simple query to return the given parameter (use a question mark `?` instead of `$1` for MySQL/MariaDB)
    let row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(&pool)
        .await?;

    // print!("Row: {row:?}");
    assert_eq!(row.0, 150);

    let results = sqlx::query(
        // Notice how we only have to bind the argument once and we can use it multiple times:
        "SELECT * FROM books where id=$1"
    )
    .bind(1)
    .fetch_all(&pool)
    .await?;
    print!("Row: {results:?}");
    Ok(())
}
