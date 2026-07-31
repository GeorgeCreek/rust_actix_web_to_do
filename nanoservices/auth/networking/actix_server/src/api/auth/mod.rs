pub mod login;
pub mod logout;
pub mod refresh;
use actix_web::web::{ServiceConfig, get, post, scope};
use auth_dal::users::descriptors::SqlxPostGresDescriptor;
use auth_kernel::user_session::descriptors::RedisSessionDescriptor;


pub fn auth_factory(app: &mut ServiceConfig) {
    app.service(
        scope("/api/v1/auth")
        .route("login", get().to(
            login::login::<SqlxPostGresDescriptor, RedisSessionDescriptor>)
        )
        .route("refresh", post().to(
            refresh::refresh::<SqlxPostGresDescriptor>)
        )
        .route("logout", post().to(
            logout::logout::<SqlxPostGresDescriptor, RedisSessionDescriptor>)
        )
    );
}
