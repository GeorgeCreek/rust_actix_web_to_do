use crate::users::schema::User;
use glue::errors::{NanoServiceError, NanoServiceErrorStatus};
use std::future::Future;
use crate::connections::sqlx_postgres::SQLX_POSTGRES_POOL;
use super::super::descriptors::SqlxPostGresDescriptor;


pub trait UpdateOne {
    fn update_one(id: i32, email: String, password: Option<String>)
        -> impl Future<Output = Result<User, NanoServiceError>> + Send;
}


impl UpdateOne for SqlxPostGresDescriptor {
    fn update_one(id: i32, email: String, password: Option<String>)
        -> impl Future<Output = Result<User, NanoServiceError>> + Send {
        sqlx_postgres_update_one(id, email, password)
    }
}


async fn sqlx_postgres_update_one(
    id: i32,
    email: String,
    password: Option<String>
) -> Result<User, NanoServiceError> {
    let user = match password {
        Some(password) => {
            sqlx::query_as::<_, User>("
                UPDATE users
                SET email = $1, password = $2
                WHERE id = $3
                RETURNING *"
            )
            .bind(email)
            .bind(password)
            .bind(id)
            .fetch_optional(&*SQLX_POSTGRES_POOL)
            .await
        },
        None => {
            sqlx::query_as::<_, User>("
                UPDATE users
                SET email = $1
                WHERE id = $2
                RETURNING *"
            )
            .bind(email)
            .bind(id)
            .fetch_optional(&*SQLX_POSTGRES_POOL)
            .await
        }
    }.map_err(|e| {
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
