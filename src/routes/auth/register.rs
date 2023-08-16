use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use sqlx::PgPool;

use crate::domain::auth::{signup_user, PasswordError, SignupUserError, SignupUserPayload};

#[tracing::instrument()]
pub async fn register(
    State(pool): State<PgPool>,
    Json(payload): Json<SignupUserPayload>,
) -> impl IntoResponse {
    if let Err(e) = signup_user(&pool, payload).await {
        match e {
            SignupUserError::UnknownError(e) => {
                return (StatusCode::INTERNAL_SERVER_ERROR, e);
            }
            SignupUserError::InvalidPassword(e) => {
                return match e {
                    PasswordError::TooShort => {
                        (StatusCode::BAD_REQUEST, "password too short".to_string())
                    }
                    PasswordError::DoesNotMatch => {
                        (StatusCode::BAD_REQUEST, "passwords must match".to_string())
                    }
                };
            }
        };
    }
    (StatusCode::OK, "user registered".to_string())
}
