use rocket::serde::json::Json;
use to_do_core::api::basic_actions::{
    delete::delete as delete_core,
    get::get_all as get_all_core
};
use to_do_dal::to_do_items::schema::AllToDOItems;
use glue::errors::NanoServiceError;
use to_do_dal::to_do_items::descriptors::SqlxPostGresDescriptor;


/// Deletes a task by id.
/// 
/// # Arguments
/// * `id` - The numeric id of the task to delete.
/// 
/// # Returns
/// A `Json` response with all the to-do items.
#[delete("/delete/<id>")]
pub async fn delete_by_id(id: i32) -> Result<Json<AllToDOItems>, NanoServiceError> {
    let _ = delete_core::<SqlxPostGresDescriptor>(id).await?;
    Ok(Json(get_all_core::<SqlxPostGresDescriptor>().await?))
}
