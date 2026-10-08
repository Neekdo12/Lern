use std::str::FromStr;

use axum::{extract::State, http::StatusCode, routing::get, Json, Router};
use shared::Greeting;
use sqlx::{
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
    SqlitePool,
};
use tower_http::services::ServeDir;

async fn greeting(State(pool): State<SqlitePool>) -> Result<Json<Greeting>, StatusCode> {
    let message: String =
        sqlx::query_scalar("SELECT message FROM greetings ORDER BY id LIMIT 1")
            .fetch_one(&pool)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(Greeting { message }))
}

#[tokio::main]
async fn main() {
    let url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite://dev.db".to_string());
    let options = SqliteConnectOptions::from_str(&url)
        .unwrap()
        .create_if_missing(true);
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(options)
        .await
        .unwrap();

    sqlx::migrate!("./migrations").run(&pool).await.unwrap();

    let app = Router::new()
        .route("/api/greeting", get(greeting))
        .fallback_service(ServeDir::new("public"))
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("Listening on http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}
