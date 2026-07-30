use to_do_core::api::basic_actions::{
    update::update as update_core,
    get::get_all as get_all_core
};
use glue::errors::NanoServiceError;
use glue::token::HeaderToken;
use rocket::serde::json::Json;
use to_do_dal::to_do_items::descriptors::SqlxPostGresDescriptor;
use to_do_dal::to_do_items::schema::{AllToDOItems, ToDoItem};


/// Updates an item in the to-do list by id.
/// 
/// # Arguments
/// - `id` - The numeric id of the item to update.
/// - `body` - The JSON body containing the item fields.
/// 
/// # Returns
/// All of the items in the to-do list.
#[patch("/update/<id>", data = "<body>")]
pub async fn update(
    token: HeaderToken,
    id: i32,
    body: Json<ToDoItem>
) -> Result<Json<AllToDOItems>, NanoServiceError> {
    let _ = token;
    let mut item = body.into_inner();
    item.id = id;
    let _ = update_core::<SqlxPostGresDescriptor>(item).await?;
    Ok(Json(get_all_core::<SqlxPostGresDescriptor>().await?))
}
