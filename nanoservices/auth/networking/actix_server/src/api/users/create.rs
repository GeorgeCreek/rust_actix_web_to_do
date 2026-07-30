use auth_dal::users::transactions::create::SaveOne;
use auth_core::api::users::create::{
    create as create_core,
    CreateUser
};
use auth_dal::users::schema::TrimmedUser;
use glue::errors::NanoServiceError;
use actix_web::{
    HttpResponse,
    web::Json
};


pub async fn create<T: SaveOne>(body: Json<CreateUser>) 
    -> Result<HttpResponse, NanoServiceError> {
    let user = create_core::<T>(body.into_inner()).await?;
    let trimmed: TrimmedUser = user.into();
    Ok(HttpResponse::Created().json(trimmed))
}

