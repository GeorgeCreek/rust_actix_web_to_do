use auth_kernel::api::users::get::get_user_by_unique_id;
use glue::errors::NanoServiceError;
use std::future::Future;


/// Resolves a JWT `unique_id` to a numeric user id.
///
/// Extracted as a trait so networking tests can inject a mock
/// without hitting Postgres.
pub trait ResolveUserId {
    fn resolve_user_id(unique_id: String)
        -> impl Future<Output = Result<i32, NanoServiceError>> + Send;
}


/// Production user resolver via auth-kernel.
pub struct AuthKernelUser;

impl ResolveUserId for AuthKernelUser {
    fn resolve_user_id(unique_id: String)
        -> impl Future<Output = Result<i32, NanoServiceError>> + Send {
        async move {
            Ok(get_user_by_unique_id(unique_id).await?.id)
        }
    }
}
