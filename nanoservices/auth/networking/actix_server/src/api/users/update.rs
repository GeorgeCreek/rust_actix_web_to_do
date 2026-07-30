use auth_dal::users::transactions::{
    update::UpdateOne,
    get::GetAll
};
use auth_core::api::users::{
    update::{update as update_core, UpdateUser},
    get_all::get_all as get_all_core
};
use glue::errors::NanoServiceError;
use glue::token::HeaderToken;
use actix_web::{
    HttpResponse,
    web::Json
};


pub async fn update<T: UpdateOne + GetAll>(
    _token: HeaderToken,
    body: Json<UpdateUser>
) -> Result<HttpResponse, NanoServiceError> {
    let _ = update_core::<T>(body.into_inner()).await?;
    Ok(HttpResponse::Ok().json(get_all_core::<T>().await?))
}
