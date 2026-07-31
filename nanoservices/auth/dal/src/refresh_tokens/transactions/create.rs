use crate::refresh_tokens::schema::{NewRefreshToken, RefreshTokenRecord};
use glue::errors::{NanoServiceError, NanoServiceErrorStatus};
use std::future::Future;
use crate::connections::sqlx_postgres::SQLX_POSTGRES_POOL;
use crate::users::descriptors::SqlxPostGresDescriptor;


pub trait SaveRefreshToken {
    fn save_refresh_token(token: NewRefreshToken)
        -> impl Future<Output = Result<RefreshTokenRecord, NanoServiceError>> + Send;
}


impl SaveRefreshToken for SqlxPostGresDescriptor {
    fn save_refresh_token(token: NewRefreshToken)
        -> impl Future<Output = Result<RefreshTokenRecord, NanoServiceError>> + Send {
        sqlx_postgres_save_refresh_token(token)
    }
}


async fn sqlx_postgres_save_refresh_token(
    token: NewRefreshToken
) -> Result<RefreshTokenRecord, NanoServiceError> {
    sqlx::query_as::<_, RefreshTokenRecord>("
        INSERT INTO refresh_tokens (user_id, jti, expires_at, revoked, created_at)
        VALUES ($1, $2, $3, FALSE, $4)
        RETURNING *"
    )
    .bind(token.user_id)
    .bind(token.jti)
    .bind(token.expires_at)
    .bind(token.created_at)
    .fetch_one(&*SQLX_POSTGRES_POOL).await.map_err(|e| {
        NanoServiceError::new(
            e.to_string(),
            NanoServiceErrorStatus::Unknown
        )
    })
}
