use crate::Db;
use crate::auth::generate_token;
use crate::auth::hash_password;
use crate::auth::verify_password;
use crate::error::*;
use crate::models::*;
use axum::{
    Json,
    extract::{Extension, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use uuid::Uuid;

// .execute
//  .fetch_one :Resut<Row?
//  .fetch_optional:Result<Option<Row>>
// .fetch_all: Result<Vec<Row>>
// .fetch:Stream<Row>
//

pub async fn login(State(db): State<Db>, Json(payload): Json<LoginUser>) -> Response {
    match sqlx::query_as!(
        UserData2,
        r#"SELECT id, password FROM users WHERE email = $1"#,
        payload.email
    )
    .fetch_optional(&db)
    .await
    {
        Ok(Some(user)) => {
            if !verify_password(&payload.password, &user.password) {
                return (
                    StatusCode::UNAUTHORIZED,
                    Json(serde_json::json!({"error": "Password is incorrect"})),
                )
                    .into_response();
            }
            let token = generate_token(user.id);
            return (StatusCode::OK, Json(serde_json::json!({"token": token}))).into_response();
        }
        Ok(None) => not_found("User not found").into_response(),
        Err(e) => server_error(e).into_response(),
    }
}

pub async fn get_users(State(db): State<Db>) -> Response {
    match sqlx::query_as!(
        UserData,
        r#"
            SELECT id, name, email
            FROM users
        "#,
    )
    .fetch_all(&db)
    .await
    {
        Ok(users) => (StatusCode::OK, Json(users)).into_response(),
        Err(e) => server_error(e).into_response(),
    }
}
pub async fn register(State(db): State<Db>, Json(payload): Json<RegisterUser>) -> Response {
    let hashed_password = hash_password(&payload.password);
    let user_id = Uuid::new_v4();
    match sqlx::query_as!(
        UserData,
        r#"
            INSERT INTO users (id, name, email, password)
            VALUES ($1, $2, $3, $4)
            RETURNING id, name, email
        "#,
        user_id,
        payload.name,
        payload.email,
        hashed_password
    )
    .fetch_one(&db)
    .await
    {
        Ok(user) => (StatusCode::OK, Json(user)).into_response(),
        Err(e) => server_error(e).into_response(),
    }
}
pub async fn get_user(State(db): State<Db>, Extension(user_id): Extension<Uuid>) -> Response {
    match sqlx::query_as!(
        UserData,
        r#"
            SELECT id, name, email FROM users WHERE id = $1
        "#,
        user_id
    )
    .fetch_optional(&db)
    .await
    {
        Ok(Some(user)) => (StatusCode::OK, Json(user)).into_response(),
        Ok(None) => not_found("user not found").into_response(),
        Err(e) => server_error(e).into_response(),
    }
}
pub async fn delete_user(State(db): State<Db>, Extension(user_id): Extension<Uuid>) -> Response {
    match sqlx::query_as!(
        UserData,
        r#"
            DELETE FROM users WHERE id = $1
            RETURNING id, name, email
        "#,
        user_id
    )
    .fetch_optional(&db)
    .await
    {
        Ok(Some(user)) => (StatusCode::OK, Json(user)).into_response(),
        Ok(None) => not_found("user not found").into_response(),
        Err(e) => server_error(e).into_response(),
    }
}

pub async fn update_user(
    State(db): State<Db>,
    Extension(user_id): Extension<Uuid>,
    Json(new_details): Json<UpdateUser>,
) -> Response {
    if new_details.name.is_none() && new_details.email.is_none() {
        return no_fields_to_update("no fields to update").into_response();
    }
    match sqlx::query_as!(
        UserData,
        r#"
            UPDATE users
            SET name = COALESCE($1, name),
                email = COALESCE($2, email)
            WHERE id = $3
            RETURNING id, name, email
        "#,
        new_details.name,
        new_details.email,
        user_id
    )
    .fetch_optional(&db)
    .await
    {
        Ok(Some(user)) => (StatusCode::OK, Json(user)).into_response(),
        Ok(None) => not_found("user not found").into_response(),
        Err(e) => server_error(e).into_response(),
    }
}
