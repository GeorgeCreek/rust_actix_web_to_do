use crate::users::schema::User;
use glue::errors::{NanoServiceError, NanoServiceErrorStatus};
use std::future::Future;
use crate::connections::sqlx_postgres::SQLX_POSTGRES_POOL;
use super::super::descriptors::SqlxPostGresDescriptor;


pub trait DeleteOne {
    fn delete_one(id: i32)
        -> impl Future<Output = Result<User, NanoServiceError>> + Send;
}


impl DeleteOne for SqlxPostGresDescriptor {
    fn delete_one(id: i32)
        -> impl Future<Output = Result<User, NanoServiceError>> + Send {
        sqlx_postgres_delete_one(id)
    }
}


async fn sqlx_postgres_delete_one(id: i32)
    -> Result<User, NanoServiceError> {
    let user = sqlx::query_as::<_, User>("
        DELETE FROM users
        WHERE id = $1
        RETURNING *"
    )
    .bind(id)
    .fetch_optional(&*SQLX_POSTGRES_POOL)
    .await
    .map_err(|e| {
        NanoServiceError::new(
            e.to_string(),
            NanoServiceErrorStatus::Unknown
        )
    })?;

    match user {
        None => Err(NanoServiceError::new(
            "User not found".to_string(),
            NanoServiceErrorStatus::NotFound
        )),
        Some(user) => Ok(user)
    }
}
