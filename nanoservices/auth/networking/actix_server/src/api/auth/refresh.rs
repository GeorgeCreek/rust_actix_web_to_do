use actix_web::{HttpResponse, web::Json};
use auth_dal::refresh_tokens::transactions::create::SaveRefreshToken;
use auth_dal::refresh_tokens::transactions::get::GetRefreshTokenByJti;
use auth_dal::refresh_tokens::transactions::revoke::RevokeRefreshToken;
use auth_core::api::auth::refresh::refresh as core_refresh;
use glue::errors::NanoServiceError;
use serde::Deserialize;


#[derive(Deserialize)]
pub struct RefreshRequest {
    pub refresh_token: String,
}


pub async fn refresh<T: GetRefreshTokenByJti + RevokeRefreshToken + SaveRefreshToken>(
    body: Json<RefreshRequest>,
) -> Result<HttpResponse, NanoServiceError> {
    let tokens = core_refresh::<T>(body.into_inner().refresh_token).await?;
    Ok(HttpResponse::Ok().json(tokens))
}
