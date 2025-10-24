use axum::{
    extract::DefaultBodyLimit,
    http::Method,
    routing::get,
    Router,
};
use std::net::SocketAddr;
use tower::ServiceBuilder;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod auth;
mod config;
mod database;
mod middleware;
mod models;
mod routes;

use config::Config;
use database::Database;
use routes::{auth::auth_routes, users::user_routes, posts::post_routes, products::product_routes, campaigns::campaign_routes, events::event_routes, creators::creator_routes, articles::articles_routes, podcasts::podcast_routes};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "funify_backend=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration
    let config = Config::from_env()?;
    
    // Initialize database
    let db = Database::new(&config.database_url).await?;
    
    // Run migrations
    db.run_migrations().await?;

    // Build our application with routes
    let app = Router::new()
        .route("/health", get(health_check))
        .nest("/api/auth", auth_routes())
        .nest("/api/users", user_routes())
        .nest("/api/creators", creator_routes())
        .nest("/api/posts", post_routes())
        .nest("/api/products", product_routes())
        .nest("/api/campaigns", campaign_routes())
        .nest("/api/events", event_routes())
        .nest("/api/articles", articles_routes())
        .nest("/api/podcasts", podcast_routes())
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(
                    CorsLayer::new()
                        .allow_origin(Any)
                        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE, Method::OPTIONS])
                        .allow_headers(Any),
                )
                .layer(axum::middleware::from_fn(middleware::auth_middleware))
                .layer(DefaultBodyLimit::max(10 * 1024 * 1024)), // 10MB limit
        )
        .with_state(db);

    // Run the server
    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    tracing::info!("Server running on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn health_check() -> &'static str {
    "OK"
}