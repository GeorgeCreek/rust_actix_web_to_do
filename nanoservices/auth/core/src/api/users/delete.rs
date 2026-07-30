use auth_dal::users::schema::TrimmedUser;
use auth_dal::users::transactions::delete::DeleteOne;
use glue::errors::NanoServiceError;


pub async fn delete<T: DeleteOne>(id: i32)
    -> Result<TrimmedUser, NanoServiceError> {
    let user = T::delete_one(id).await?;
    Ok(user.into())
}
