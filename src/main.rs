use askama::Template;
use axum::{routing::get, Router};
use dotenv::dotenv;
use tower_http::services::{ServeDir, ServeFile};
use tower_http::trace::TraceLayer;
use tracing::debug;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod db;
mod handlers;
mod model;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setting up variables
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    dotenv().ok();

    let port = std::env::var("PORT").unwrap_or("8888".to_string());
    let addr = format!("0.0.0.0:{}", port);

    //Setting up Database
    let pool = db::connect_to_database().await;

    // Setting up router
    let app = Router::new()
        .route("/", get(index))
        .nest_service("/assets", ServeDir::new("assets"))
        .nest_service("/favicon.ico", ServeFile::new("assets/favicon.ico"))
        .merge(handlers::api_routes())
        .layer(TraceLayer::new_for_http())
        .fallback(index)
        .with_state(pool);

    // Starting server
    println!("Starting server: Listening at {}", &addr);
    debug!("Starting server: Listening at {}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

async fn index() -> IndexTemplate {
    IndexTemplate
}

// Askma
#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate;
