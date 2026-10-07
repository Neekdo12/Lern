use axum::{routing::get, Json, Router};
use shared::Greeting;
use tower_http::services::ServeDir;

async fn greeting() -> Json<Greeting> {
    Json(Greeting {
        message: "Ahojky".to_string(),
    })
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/api/greeting", get(greeting))
        .fallback_service(ServeDir::new("public"));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("Listening on http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}
