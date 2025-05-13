use axum::Router;
use axum::routing::get;
use std::net::SocketAddr;

struct AppState {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let state = AppState {};
    let state = std::sync::Arc::new(state);
    let cors = tower_http::cors::CorsLayer::new()
        .allow_origin(tower_http::cors::Any)
        .allow_methods(tower_http::cors::Any)
        .allow_headers(tower_http::cors::Any);

    let listening_addr = std::env::var("LISTENING_ADDRESS").unwrap();
    println!("Listening on: {}", listening_addr);
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .layer(cors)
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(&listening_addr).await?;

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await?;
    Ok(())
}
