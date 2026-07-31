use auth_dal::refresh_tokens::transactions::revoke::RevokeRefreshToken;
use glue::errors::NanoServiceError;
use glue::token::RefreshToken;


pub async fn logout<T: RevokeRefreshToken>(
    refresh_token: String,
) -> Result<String, NanoServiceError> {
    let claims = RefreshToken::decode(&refresh_token)?;
    T::revoke_by_jti(claims.jti).await?;
    Ok(claims.unique_id)
}
