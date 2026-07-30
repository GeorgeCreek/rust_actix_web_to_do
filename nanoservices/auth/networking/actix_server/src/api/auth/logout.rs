use actix_web::HttpResponse;
use glue::errors::{NanoServiceError, NanoServiceErrorStatus};
use glue::token::HeaderToken;
use auth_kernel::user_session::transactions::logout::LogoutUserSession;


pub async fn logout<X: LogoutUserSession>(token: HeaderToken)
    -> Result<HttpResponse, NanoServiceError>
{
    let url = std::env::var("CACHE_API_URL").map_err(|e|{
        NanoServiceError::new(
            e.to_string(),
            NanoServiceErrorStatus::Unknown
        )
    })?;
    X::logout_user_session(&url, &token.unique_id).await?;
    Ok(HttpResponse::Ok().finish())
}
