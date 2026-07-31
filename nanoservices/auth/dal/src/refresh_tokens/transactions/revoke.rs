use glue::errors::{NanoServiceError, NanoServiceErrorStatus};
use std::future::Future;
use crate::connections::sqlx_postgres::SQLX_POSTGRES_POOL;
use crate::users::descriptors::SqlxPostGresDescriptor;


pub trait RevokeRefreshToken {
    fn revoke_by_jti(jti: String)
        -> impl Future<Output = Result<(), NanoServiceError>> + Send;

    fn revoke_all_active_for_user(user_id: i32)
        -> impl Future<Output = Result<u64, NanoServiceError>> + Send;
}


impl RevokeRefreshToken for SqlxPostGresDescriptor {
    fn revoke_by_jti(jti: String)
        -> impl Future<Output = Result<(), NanoServiceError>> + Send {
        sqlx_postgres_revoke_by_jti(jti)
    }

    fn revoke_all_active_for_user(user_id: i32)
        -> impl Future<Output = Result<u64, NanoServiceError>> + Send {
        sqlx_postgres_revoke_all_active_for_user(user_id)
    }
}


async fn sqlx_postgres_revoke_by_jti(jti: String) -> Result<(), NanoServiceError> {
    let result = sqlx::query("
        UPDATE refresh_tokens SET revoked = TRUE WHERE jti = $1"
    )
    .bind(jti)
    .execute(&*SQLX_POSTGRES_POOL).await.map_err(|e| {
        NanoServiceError::new(
            e.to_string(),
            NanoServiceErrorStatus::Unknown
        )
    })?;

    if result.rows_affected() == 0 {
        return Err(NanoServiceError::new(
            "refresh token not found".to_string(),
            NanoServiceErrorStatus::Unauthorized
        ));
    }
    Ok(())
}


async fn sqlx_postgres_revoke_all_active_for_user(
    user_id: i32
) -> Result<u64, NanoServiceError> {
    let result = sqlx::query("
        UPDATE refresh_tokens
        SET revoked = TRUE
        WHERE user_id = $1 AND revoked = FALSE"
    )
    .bind(user_id)
    .execute(&*SQLX_POSTGRES_POOL).await.map_err(|e| {
        NanoServiceError::new(
            e.to_string(),
            NanoServiceErrorStatus::Unknown
        )
    })?;
    Ok(result.rows_affected())
}

