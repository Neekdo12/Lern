use axum::Router;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    // Serves everything in ./public, with index.html at "/"
    let app = Router::new().fallback_service(ServeDir::new("public"));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("Listening on http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}
