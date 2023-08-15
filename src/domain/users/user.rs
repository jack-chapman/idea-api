use serde::{Deserialize, Serialize};

#[derive(
    Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, sqlx::FromRow,
)]
pub struct User {
    id: i32,
    name: String,
    email: String,
    password_hash: String,
    created_at: chrono::DateTime<chrono::Utc>,
    #[sqlx(default)]
    updated_at: chrono::DateTime<chrono::Utc>,
    #[sqlx(default)]
    deleted_at: chrono::DateTime<chrono::Utc>,
}
