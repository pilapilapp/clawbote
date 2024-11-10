use axum::{
    routing::get,
    Router,
};
use tracing::info;
use tracing_subscriber::{fmt, EnvFilter};

#[tokio::main]
async fn main() {
    // Initialize the tracing subscriber
    let filter = EnvFilter::new("info");
    fmt()
        .with_env_filter(filter)
        .init();

    // Build our application with a single route
    let app = Router::new()
        .route("/", get(hello_world));

    // Run it on localhost:3000
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    info!("Server running on http://127.0.0.1:3000");
    
    axum::serve(listener, app).await.unwrap();
}

// Basic handler that returns a static string
async fn hello_world() -> &'static str {
    "Hello, World!"
}
