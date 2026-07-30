use auth_dal::users::schema::TrimmedUser;
use auth_dal::users::transactions::get::GetAll;
use glue::errors::NanoServiceError;


pub async fn get_all<T: GetAll>()
    -> Result<Vec<TrimmedUser>, NanoServiceError> {
    let users = T::get_all().await?;
    Ok(users.into_iter().map(|user| user.into()).collect())
}
