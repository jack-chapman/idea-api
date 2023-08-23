use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use pbkdf2::password_hash::{PasswordHash, PasswordVerifier};
use pbkdf2::Pbkdf2;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::domain::users::find_user;

#[derive(Deserialize, Debug)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
}

#[derive(Serialize, Debug)]
pub struct LoginError {
    pub message: String,
}

#[tracing::instrument(skip(pool))]
pub async fn sign_in(
    State(pool): State<PgPool>,
    Json(body): Json<LoginRequest>,
) -> impl IntoResponse {
    let user = find_user(&pool, body.email).await;
    if user.is_none() {
        return invalid_login().into_response();
    }
    let user = user.unwrap();
    let hashed_password = PasswordHash::new(&user.password_hash).unwrap();
    if Pbkdf2
        .verify_password(body.password.as_bytes(), &hashed_password)
        .is_err()
    {
        return invalid_login().into_response();
    }
    StatusCode::OK.into_response()
}

fn invalid_login() -> (StatusCode, Json<LoginError>) {
    (
        StatusCode::BAD_REQUEST,
        Json(LoginError {
            message: "invalid email address or password".to_string(),
        }),
    )
}
