use std::future::Future;
use glue::errors::NanoServiceError;
use crate::user_session::descriptors::RedisSessionDescriptor;
use cache_client::logout as cache_logout;


pub trait LogoutUserSession {
    fn logout_user_session(
        address: &str,
        user_id: &str
    ) -> impl Future<Output = Result<(), NanoServiceError>>;
}


impl LogoutUserSession for RedisSessionDescriptor {
    fn logout_user_session(
        address: &str,
        user_id: &str
    ) -> impl Future<Output = Result<(), NanoServiceError>> {
        async move {
            cache_logout(address, user_id).await?;
            Ok(())
        }
    }
}
