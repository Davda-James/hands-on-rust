mod handlers;
mod models;
mod redis_setup;
mod routes;
mod schema;

use handlers::*;

use axum::{
    Router,
    routing::{get, post},
};
use dotenvy::dotenv;
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::time::Duration;

use crate::models::Config;

type Db = PgPool;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let url = std::env::var("DATABASE_URL").unwrap();
    let db: Db = PgPoolOptions::new()
        .max_connections(3)
        .min_connections(1)
        .acquire_timeout(Duration::from_secs(5))
        .idle_timeout(Duration::from_secs(10 * 60))
        .max_lifetime(Duration::from_secs(30 * 60))
        .connect(&url)
        .await
        .unwrap();
    let redis_client = redis_setup::init_redis();
    let config = Config {
        db,
        rc: redis_client,
    };

    let app = Router::new()
        .route("/health", get(check_health))
        .route("/short", post(short_the_url))
        .route("/{id}", get(redirect))
        .with_state(config);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    println!("Server is listening");
    axum::serve(listener, app).await.unwrap();
}
