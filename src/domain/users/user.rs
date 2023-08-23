use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(
    Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, sqlx::FromRow,
)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[sqlx(default)]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    #[sqlx(default)]
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
}

pub async fn find_user(pool: &PgPool, email: String) -> Option<User> {
    sqlx::query_as!(
        User,
        r#"
    SELECT *
    FROM users
    WHERE email = $1
    "#,
        email
    )
    .fetch_one(pool)
    .await
    .ok()
}
