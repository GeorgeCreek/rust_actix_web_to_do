use serde::{Deserialize, Serialize};
use sqlx::FromRow;


#[derive(Serialize, Deserialize, FromRow, Debug, Clone)]
pub struct RefreshTokenRecord {
    pub id: i32,
    pub user_id: i32,
    pub jti: String,
    pub expires_at: i64,
    pub revoked: bool,
    pub created_at: i64,
}


#[derive(Debug, Clone)]
pub struct NewRefreshToken {
    pub user_id: i32,
    pub jti: String,
    pub expires_at: i64,
    pub created_at: i64,
}
