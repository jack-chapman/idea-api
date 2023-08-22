use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use sqlx::PgPool;

use crate::domain::auth::{signup_user, PasswordError, SignupUserError, SignupUserPayload};

#[derive(serde::Serialize)]
struct RegisterResponse {
    message: String,
}

impl RegisterResponse {
    fn with_message(message: &str) -> Self {
        Self {
            message: message.to_string(),
        }
    }
}

impl From<PasswordError> for RegisterResponse {
    fn from(e: PasswordError) -> Self {
        match e {
            PasswordError::TooShort => RegisterResponse::with_message("password too short"),
            PasswordError::DoesNotMatch => RegisterResponse::with_message("passwords must match"),
        }
    }
}

impl From<SignupUserError> for (StatusCode, Json<RegisterResponse>) {
    fn from(e: SignupUserError) -> Self {
        match e {
            SignupUserError::UnknownError(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(RegisterResponse::with_message(&e)),
            ),
            SignupUserError::InvalidPassword(e) => {
                (StatusCode::BAD_REQUEST, Json(RegisterResponse::from(e)))
            }
        }
    }
}

#[tracing::instrument()]
pub async fn register(
    State(pool): State<PgPool>,
    Json(payload): Json<SignupUserPayload>,
) -> impl IntoResponse {
    if let Err(e) = signup_user(&pool, payload).await {
        tracing::error!("Unable to register user");
        return e.into();
    }
    (
        StatusCode::OK,
        Json(RegisterResponse::with_message("user registered")),
    )
}
