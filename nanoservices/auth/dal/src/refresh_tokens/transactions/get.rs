use crate::refresh_tokens::schema::RefreshTokenRecord;
use glue::errors::{NanoServiceError, NanoServiceErrorStatus};
use std::future::Future;
use crate::connections::sqlx_postgres::SQLX_POSTGRES_POOL;
use crate::users::descriptors::SqlxPostGresDescriptor;


pub trait GetRefreshTokenByJti {
    fn get_by_jti(jti: String)
        -> impl Future<Output = Result<RefreshTokenRecord, NanoServiceError>> + Send;
}


impl GetRefreshTokenByJti for SqlxPostGresDescriptor {
    fn get_by_jti(jti: String)
        -> impl Future<Output = Result<RefreshTokenRecord, NanoServiceError>> + Send {
        sqlx_postgres_get_by_jti(jti)
    }
}


async fn sqlx_postgres_get_by_jti(
    jti: String
) -> Result<RefreshTokenRecord, NanoServiceError> {
    sqlx::query_as::<_, RefreshTokenRecord>("
        SELECT * FROM refresh_tokens WHERE jti = $1"
    )
    .bind(jti)
    .fetch_optional(&*SQLX_POSTGRES_POOL).await.map_err(|e| {
        NanoServiceError::new(
            e.to_string(),
            NanoServiceErrorStatus::Unknown
        )
    })?
    .ok_or_else(|| NanoServiceError::new(
        "refresh token not found".to_string(),
        NanoServiceErrorStatus::Unauthorized
    ))
}
