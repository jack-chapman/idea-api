use pbkdf2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Pbkdf2,
};
use serde::Deserialize;
use sqlx::PgPool;

use crate::domain::users::{create_user, CreateUser, CreateUserError};

#[derive(Debug, Deserialize)]
pub struct SignupUserPayload {
    name: String,
    email: String,
    password: String,
    confirm_password: String,
}

pub enum PasswordError {
    TooShort,
    DoesNotMatch,
}

pub enum SignupUserError {
    UnknownError(String),
    InvalidPassword(PasswordError),
}

pub async fn signup_user(pool: &PgPool, payload: SignupUserPayload) -> Result<(), SignupUserError> {
    if payload.password != payload.confirm_password {
        return Err(SignupUserError::InvalidPassword(
            PasswordError::DoesNotMatch,
        ));
    }
    if payload.password.len() < 7 {
        return Err(SignupUserError::InvalidPassword(PasswordError::TooShort));
    }

    let salt = SaltString::generate(&mut OsRng);

    let password_hash = Pbkdf2
        .hash_password(payload.password.as_bytes(), &salt)
        .unwrap();

    if let Err(e) = create_user(
        pool,
        CreateUser {
            email: payload.email,
            name: payload.name,
            password_hash: password_hash.to_string(),
        },
    )
    .await
    {
        let CreateUserError::UnknownError(e) = e;
        return Err(SignupUserError::UnknownError(e));
    }

    Ok(())
}
