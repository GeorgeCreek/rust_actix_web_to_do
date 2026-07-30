use to_do_core::api::basic_actions::{
    delete::delete as delete_core,
    get::get_all as get_all_core
};
use axum::{
    response::IntoResponse,
    extract::Path,
    http::StatusCode,
    Json,
};
use glue::errors::NanoServiceError;
use to_do_dal::to_do_items::transactions::{
    delete::DeleteOne,
    get::GetAll
};


/// Deletes an item from the to-do list by id.
/// 
/// # Arguments
/// - `id` - The numeric id of the item to delete.
/// 
/// # Returns
/// All of the items in the to-do list.
pub async fn delete_by_id<T: DeleteOne + GetAll>(
    Path(id): Path<i32>
) -> Result<impl IntoResponse, NanoServiceError> {
    let _ = delete_core::<T>(id).await?;
    Ok((StatusCode::OK, Json(get_all_core::<T>().await?)).into_response())
}
