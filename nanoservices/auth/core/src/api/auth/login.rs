use auth_dal::users::transactions::get::GetByEmail;
use auth_dal::refresh_tokens::transactions::create::SaveRefreshToken;
use auth_dal::refresh_tokens::transactions::revoke::RevokeRefreshToken;
use glue::errors::{NanoServiceError, NanoServiceErrorStatus};
use glue::token::AuthTokenPair;
use super::tokens::issue_token_pair;


pub async fn login<T: GetByEmail + SaveRefreshToken + RevokeRefreshToken>(
    email: String, 
    password: String
) -> Result<AuthTokenPair, NanoServiceError> {
    let user = T::get_by_email(email).await?;
    let outcome = user.verify_password(password)?;
    if outcome {
        issue_token_pair::<T>(user.id, user.unique_id).await
    } else {
        Err(NanoServiceError::new(
            "Invalid password".to_string(),
            NanoServiceErrorStatus::Unauthorized
        ))
    }
}
