use crate::models::Config;
use crate::models::ShortenURLRequest;

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Redirect, Response},
};

use nanoid::nanoid;
use serde_json::json;
use uuid::Uuid;

use redis::AsyncCommands;

pub async fn check_health() -> Response {
    (StatusCode::OK, Json(json!({"status": "ok"}))).into_response()
}

pub async fn short_the_url(
    State(con): State<Config>,
    Json(payload): Json<ShortenURLRequest>,
) -> Response {
    let code = nanoid!(7);
    let url_uid = Uuid::new_v4();

    let short_url = format!("http://localhost:8000/{}", code);

    // insert into postgres
    let result = sqlx::query!(
        r#"
        INSERT INTO urls (id, short_code, full_url)
        VALUES ($1, $2, $3)
        "#,
        url_uid,
        code,
        payload.url
    )
    .execute(&con.db)
    .await;

    if let Err(e) = result {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
            .into_response();
    }

    // cache in redis
    if let Ok(mut con) = con.rc.get_multiplexed_async_connection().await {
        let _: Result<(), _> = con.set(&code, &payload.url).await;
    }

    (StatusCode::OK, Json(json!({ "short_url": short_url }))).into_response()
}

pub async fn redirect(State(con): State<Config>, Path(code): Path<String>) -> Response {
    let mut conn = match con.rc.get_multiplexed_async_connection().await {
        Ok(c) => c,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "redis connection failed"})),
            )
                .into_response();
        }
    };

    // check redis cache
    let cached: Result<String, _> = conn.get(&code).await;

    if let Ok(url) = cached {
        return Redirect::temporary(&url).into_response();
    }

    // check database
    match sqlx::query!(
        r#"
        SELECT full_url
        FROM urls
        WHERE short_code = $1
        "#,
        code
    )
    .fetch_optional(&con.db)
    .await
    {
        Ok(Some(record)) => {
            // store in redis
            let _: Result<(), _> = conn.set(&code, &record.full_url).await;

            Redirect::temporary(&record.full_url).into_response()
        }

        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(json!({"error": "url not found"})),
        )
            .into_response(),

        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
            .into_response(),
    }
}
