use auth_dal::refresh_tokens::transactions::create::SaveRefreshToken;
use auth_dal::refresh_tokens::transactions::get::GetRefreshTokenByJti;
use auth_dal::refresh_tokens::transactions::revoke::RevokeRefreshToken;
use glue::errors::{NanoServiceError, NanoServiceErrorStatus};
use glue::token::{AuthTokenPair, RefreshToken};
use super::tokens::issue_token_pair;
use std::time::{SystemTime, UNIX_EPOCH};


fn unix_now() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}


pub async fn refresh<T: GetRefreshTokenByJti + RevokeRefreshToken + SaveRefreshToken>(
    refresh_token: String,
) -> Result<AuthTokenPair, NanoServiceError> {
    let claims = RefreshToken::decode(&refresh_token)?;
    let record = T::get_by_jti(claims.jti.clone()).await?;

    if record.revoked {
        return Err(NanoServiceError::new(
            "refresh token revoked".to_string(),
            NanoServiceErrorStatus::Unauthorized
        ));
    }
    if record.expires_at < unix_now() {
        return Err(NanoServiceError::new(
            "refresh token expired".to_string(),
            NanoServiceErrorStatus::Unauthorized
        ));
    }

    // Rotate: revoke current refresh token, then issue a new pair.
    // issue_token_pair also revokes any other active rows for the user.
    T::revoke_by_jti(claims.jti).await?;
    issue_token_pair::<T>(record.user_id, claims.unique_id).await
}
