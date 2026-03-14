mod auth;
mod error;
mod handlers;
mod middleware;
mod models;
mod routes;
mod schema;

use routes::users_routes;
use tower_http::{
    request_id::{MakeRequestUuid, SetRequestIdLayer},
    trace::TraceLayer,
};
use tracing_subscriber;

use axum::{
    Json, Router,
    http::{StatusCode, header::HeaderName},
    response::{IntoResponse, Response},
    routing::get,
};
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env::var;
use std::time::Duration;

type Db = sqlx::PgPool;
fn init_tracking() {
    tracing_subscriber::fmt().with_env_filter("info").init();
}

#[tokio::main]
async fn main() {
    init_tracking();
    dotenv().ok();
    let url = var("DATABASE_URL").unwrap();

    let db: Db = PgPoolOptions::new()
        .max_connections(3)
        .min_connections(1)
        .acquire_timeout(Duration::from_secs(5))
        .idle_timeout(Duration::from_secs(10 * 60))
        .max_lifetime(Duration::from_secs(30 * 60))
        .connect(&url)
        .await
        .unwrap();

    let app = Router::new()
        .route("/health", get(health_check))
        .nest("/users", users_routes())
        .layer(TraceLayer::new_for_http())
        .layer(SetRequestIdLayer::new(
            HeaderName::from_static("x-request-id"),
            MakeRequestUuid,
        ))
        .with_state(db);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();

    println!("Server is here for u, ask me");

    axum::serve(listener, app).await.unwrap();
}

async fn health_check() -> Response {
    (StatusCode::OK, Json(serde_json::json!({"status": "ok"}))).into_response()
}
