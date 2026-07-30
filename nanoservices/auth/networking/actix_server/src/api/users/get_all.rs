use auth_dal::users::transactions::get::GetAll;
use auth_core::api::users::get_all::get_all as get_all_core;
use glue::errors::NanoServiceError;
use glue::token::HeaderToken;
use actix_web::HttpResponse;


pub async fn get_all<T: GetAll>(_token: HeaderToken)
    -> Result<HttpResponse, NanoServiceError> {
    let users = get_all_core::<T>().await?;
    Ok(HttpResponse::Ok().json(users))
}
