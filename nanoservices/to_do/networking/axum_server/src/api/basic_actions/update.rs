use to_do_core::api::basic_actions::{
    update::update as update_core,
    get::get_all as get_all_core
};
use to_do_dal::to_do_items::schema::ToDoItem;
use glue::errors::NanoServiceError;
use axum::{
    extract::{Json, Path},
    http::StatusCode,
    response::IntoResponse,
};
use glue::token::HeaderToken;
use to_do_dal::to_do_items::transactions::{
    update::UpdateOne,
    get::GetAll
};


/// Updates an item in the to-do list by id.
/// 
/// # Arguments
/// - `id` - The numeric id of the item to update.
/// - `body` - The JSON body containing the item fields.
/// 
/// # Returns
/// All of the items in the to-do list.
pub async fn update<T: UpdateOne + GetAll>(
    token: HeaderToken,
    Path(id): Path<i32>,
    Json(mut body): Json<ToDoItem>
) -> Result<impl IntoResponse, NanoServiceError> {
    let _ = token;
    body.id = id;
    let _ = update_core::<T>(body).await?;
    let all_items = get_all_core::<T>().await?;
    Ok((StatusCode::OK, Json(all_items)))
}
