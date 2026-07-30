use auth_dal::users::transactions::{
    delete::DeleteOne,
    get::GetAll
};
use auth_core::api::users::{
    delete::delete as delete_core,
    get_all::get_all as get_all_core
};
use glue::errors::{NanoServiceError, NanoServiceErrorStatus};
use glue::token::HeaderToken;
use actix_web::{
    HttpResponse,
    HttpRequest
};


pub async fn delete_by_id<T: DeleteOne + GetAll>(
    _token: HeaderToken,
    req: HttpRequest
) -> Result<HttpResponse, NanoServiceError> {
    match req.match_info().get("id") {
        Some(id) => {
            let id = id.parse::<i32>().map_err(|_| {
                NanoServiceError::new(
                    "Invalid user id".to_string(),
                    NanoServiceErrorStatus::BadRequest
                )
            })?;
            delete_core::<T>(id).await?;
        },
        None => {
            return Err(NanoServiceError::new(
                "Id not provided".to_string(),
                NanoServiceErrorStatus::BadRequest
            ))
        }
    };
    Ok(HttpResponse::Ok().json(get_all_core::<T>().await?))
}
