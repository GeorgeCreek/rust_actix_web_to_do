use auth_dal::refresh_tokens::schema::NewRefreshToken;
use auth_dal::refresh_tokens::transactions::create::SaveRefreshToken;
use auth_dal::refresh_tokens::transactions::revoke::RevokeRefreshToken;
use glue::errors::NanoServiceError;
use glue::token::{AuthTokenPair, HeaderToken, RefreshToken};
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;


fn unix_now() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}


pub async fn issue_token_pair<T: SaveRefreshToken + RevokeRefreshToken>(
    user_id: i32,
    unique_id: String,
) -> Result<AuthTokenPair, NanoServiceError> {
    // One active refresh session per user: revoke older rows first.
    T::revoke_all_active_for_user(user_id).await?;

    let access_token = HeaderToken::new(unique_id.clone()).encode()?;
    let jti = Uuid::new_v4().to_string();
    let refresh_claims = RefreshToken::new(unique_id, jti.clone());
    let expires_at = refresh_claims.expires_at();
    let refresh_token = refresh_claims.encode()?;

    T::save_refresh_token(NewRefreshToken {
        user_id,
        jti,
        expires_at,
        created_at: unix_now(),
    }).await?;

    Ok(AuthTokenPair {
        access_token,
        refresh_token,
    })
}
