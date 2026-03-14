use axum::{Json, http::StatusCode};

pub fn not_found(msg: &str) -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::NOT_FOUND,
        Json(serde_json::json!({"msg" : msg})),
    )
}

pub fn server_error(msg: sqlx::Error) -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(serde_json::json!({"msg": msg.to_string()})),
    )
}

pub fn no_fields_to_update(msg: &str) -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::BAD_REQUEST,
        Json(serde_json::json!({"msg" : msg})),
    )
}
