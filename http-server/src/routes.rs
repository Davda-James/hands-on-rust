use crate::Db;
use crate::handlers::*;
use crate::middleware::auth_middleware;
use axum::{
    Router, middleware,
    routing::{delete, get, post, put},
};
use std::sync::Arc;
use tower_governor::{
    GovernorLayer, governor::GovernorConfigBuilder, key_extractor::PeerIpKeyExtractor,
};

pub fn users_routes() -> Router<Db> {
    let governor_conf = Arc::new(
        GovernorConfigBuilder::default()
            .per_second(4)
            .burst_size(4)
            .key_extractor(PeerIpKeyExtractor)
            .finish()
            .unwrap(),
    );

    let protected_routes = Router::new()
        .route("/", get(get_users))
        .route("/{id}", get(get_user))
        .route("/{id}", put(update_user))
        .route("/{id}", delete(delete_user))
        .layer(middleware::from_fn(auth_middleware))
        .layer(GovernorLayer::new(governor_conf));
    let public_routes = Router::new()
        .route("/", post(register))
        .route("/login", post(login));
    axum::Router::merge(protected_routes, public_routes)
}
