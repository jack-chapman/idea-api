use sqlx::PgPool;

pub struct CreateUser {
    pub name: String,
    pub email: String,
    pub password_hash: String,
}

pub enum CreateUserError {
    UnknownError(String),
}

pub async fn create_user(pool: &PgPool, create_user: CreateUser) -> Result<(), CreateUserError> {
    sqlx::query!(
        r#"
    INSERT INTO users (name, email, password_hash)
    VALUES ($1, $2, $3)
    "#,
        create_user.name,
        create_user.email,
        create_user.password_hash
    )
    .execute(pool)
    .await
    .map_err(|e| CreateUserError::UnknownError(e.to_string()))?;

    Ok(())
}
