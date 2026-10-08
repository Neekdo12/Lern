use std::str::FromStr;
// I hope this workes for a long time
use axum::{Json, Router, extract::State, http::StatusCode, routing::get};
use shared::Greeting;
use sqlx::{
    SqlitePool,
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
};
use tower_http::services::{ServeDir, ServeFile};

async fn greeting(State(pool): State<SqlitePool>) -> Result<Json<Greeting>, StatusCode> {
    let message =
        sqlx::query_scalar!(r#"SELECT message AS "message!" FROM greetings ORDER BY id LIMIT 1"#)
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
        .fallback_service(ServeDir::new("web/dist").fallback(ServeFile::new("web/dist/index.html")))
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("Listening on http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}
