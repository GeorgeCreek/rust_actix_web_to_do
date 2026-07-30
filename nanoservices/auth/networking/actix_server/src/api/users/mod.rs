pub mod create;
pub mod get;
pub mod get_all;
pub mod update;
pub mod delete;
use auth_dal::users::descriptors::SqlxPostGresDescriptor;
use actix_web::web::{ServiceConfig, scope, post, get, put, delete};


pub fn users_factory(app: &mut ServiceConfig) {
    app.service(
        scope("/api/v1/users")
        .route("create", post().to(
            create::create::<SqlxPostGresDescriptor>)
        )
        .route("get", get().to(
            get::get_by_unique_id::<SqlxPostGresDescriptor>)
        )
        .route("get/all", get().to(
            get_all::get_all::<SqlxPostGresDescriptor>)
        )
        .route("update", put().to(
            update::update::<SqlxPostGresDescriptor>)
        )
        .route("delete/{id}", delete().to(
            delete::delete_by_id::<SqlxPostGresDescriptor>)
        )
    );
}
