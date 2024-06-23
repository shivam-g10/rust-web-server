use std::path::Path;

use poem::listener::TcpListener;
use server::app::bootstrap::{build_app, make_app_state};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    std::env::set_var("RUST_LOG", "debug");
    // load .env
    let _ = dotenvy::from_path(Path::new(format!("{}/.env", env!("CARGO_MANIFEST_DIR")).as_str()));

    tracing_subscriber::fmt::init();

    // Base environment variables
    let host = std::env::var("HOST").expect("HOST is not set in .env file");
    let port = std::env::var("PORT").expect("PORT is not set in .env file");
    let server_url = format!("{host}:{port}");

    let state = make_app_state().await;

    let app = build_app(state);

    poem::Server::new(TcpListener::bind(server_url))
        .run(app)
        .await
}
