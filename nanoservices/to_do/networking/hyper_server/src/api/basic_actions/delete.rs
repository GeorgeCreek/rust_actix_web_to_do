use glue::errors::{NanoServiceError, NanoServiceErrorStatus};
use glue::safe_eject;
use hyper::body::Bytes;
use hyper::{header, Response};
use to_do_core::api::basic_actions::{
    delete::delete as delete_core,
    get::get_all as get_all_core
};
use http_body_util::Full;
use to_do_dal::to_do_items::transactions::{
    delete::DeleteOne,
    get::GetAll
};


/// Deletes a task by id.
/// 
/// # Arguments
/// * `id` - The numeric id of the task to delete.
/// 
/// # Returns
/// A `Response` with a body containing all the to-do items.
pub async fn delete_by_id<T: DeleteOne + GetAll>(id: i32) -> Result<Response<Full<Bytes>>, NanoServiceError> {
    let _ = delete_core::<T>(id).await?;
    let json_body = safe_eject!(
        serde_json::to_string(&get_all_core::<T>().await?),
        NanoServiceErrorStatus::Unknown
    )?;
    safe_eject!(
        Response::builder()
                .header(header::CONTENT_TYPE, "application/json")
                .body(Full::new(Bytes::from(json_body))),
        NanoServiceErrorStatus::Unknown
    )
}
