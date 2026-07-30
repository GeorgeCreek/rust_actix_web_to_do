use auth_dal::users::schema::{NewUser, TrimmedUser};
use auth_dal::users::transactions::update::UpdateOne;
use glue::errors::NanoServiceError;
use serde::Deserialize;


#[derive(Deserialize)]
pub struct UpdateUser {
    pub id: i32,
    pub email: String,
    pub password: Option<String>
}


pub async fn update<T: UpdateOne>(data: UpdateUser)
    -> Result<TrimmedUser, NanoServiceError> {
    let password = match data.password {
        Some(password) if !password.is_empty() => {
            Some(NewUser::hash_password(password)?)
        },
        _ => None
    };
    let user = T::update_one(data.id, data.email, password).await?;
    Ok(user.into())
}
