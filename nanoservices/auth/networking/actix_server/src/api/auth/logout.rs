use actix_web::{HttpResponse, web::Json};
use auth_dal::refresh_tokens::transactions::revoke::RevokeRefreshToken;
use auth_core::api::auth::logout::logout as core_logout;
use auth_kernel::user_session::transactions::logout::LogoutUserSession;
use glue::errors::NanoServiceError;
use serde::Deserialize;


#[derive(Deserialize)]
pub struct LogoutRequest {
    pub refresh_token: String,
}


pub async fn logout<T, X>(
    body: Json<LogoutRequest>,
) -> Result<HttpResponse, NanoServiceError>
where
    T: RevokeRefreshToken,
    X: LogoutUserSession,
{
    let unique_id = core_logout::<T>(body.into_inner().refresh_token).await?;
    if let Ok(url) = std::env::var("CACHE_API_URL") {
        let _ = X::logout_user_session(&url, &unique_id).await;
    }
    Ok(HttpResponse::NoContent().finish())
}
