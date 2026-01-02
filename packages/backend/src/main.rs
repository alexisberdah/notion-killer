use anyhow::Result;
use axum::{
    routing::{get, post},
    Router,
};
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod api;
mod config;
mod domain;
mod error;
mod infrastructure;

use api::handlers;
use config::Settings;

#[derive(Clone)]
pub struct AppState {
    pub db: sqlx::PgPool,
    pub config: Settings,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Load environment variables
    dotenvy::dotenv().ok();

    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "notion_killer_backend=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration
    let config = Settings::from_env()?;

    tracing::info!("Connecting to database...");

    // Create database connection pool
    let db = PgPoolOptions::new()
        .max_connections(config.database.max_connections)
        .connect(&config.database.url)
        .await?;

    // Migrations are run manually via: sqlx migrate run
    // Or using the SQL file directly in production
    tracing::info!("Database connected successfully");

    let state = AppState {
        db,
        config: config.clone(),
    };

    // Build router
    let app = Router::new()
        // Health check
        .route("/health", get(handlers::health::health_check))
        // Auth routes
        .route("/api/v1/auth/register", post(handlers::auth::register))
        .route("/api/v1/auth/login", post(handlers::auth::login))
        .route("/api/v1/auth/refresh", post(handlers::auth::refresh_token))
        .route("/api/v1/auth/logout", post(handlers::auth::logout))
        // User routes
        .route("/api/v1/users/me", get(handlers::users::get_me))
        // Workspace routes
        .route(
            "/api/v1/workspaces",
            get(handlers::workspaces::list_workspaces)
                .post(handlers::workspaces::create_workspace),
        )
        .route(
            "/api/v1/workspaces/:id",
            get(handlers::workspaces::get_workspace)
                .patch(handlers::workspaces::update_workspace)
                .delete(handlers::workspaces::delete_workspace),
        )
        // Page routes
        .route("/api/v1/pages", get(handlers::pages::list_pages).post(handlers::pages::create_page))
        .route(
            "/api/v1/pages/:id",
            get(handlers::pages::get_page)
                .patch(handlers::pages::update_page)
                .delete(handlers::pages::delete_page),
        )
        .route("/api/v1/pages/:id/duplicate", post(handlers::pages::duplicate_page))
        .route("/api/v1/pages/:id/move", post(handlers::pages::move_page))
        .route("/api/v1/pages/:id/breadcrumbs", get(handlers::pages::get_breadcrumbs))
        .route("/api/v1/workspaces/:id/page-tree", get(handlers::pages::get_page_tree))
        // Middleware
        .layer(TraceLayer::new_for_http())
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], config.server.port));
    tracing::info!("Server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
