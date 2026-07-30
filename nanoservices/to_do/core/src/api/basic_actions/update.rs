use to_do_dal::to_do_items::schema::ToDoItem;
use glue::errors::NanoServiceError;
use to_do_dal::to_do_items::transactions::update::UpdateOne;


/// Update an item in the to do list.
/// 
/// # Arguments
/// * `item` - The item to update.
pub async fn update<T: UpdateOne>(item: ToDoItem, user_id: i32) 
    -> Result<(), NanoServiceError> {
    let _ = T::update_one(item, user_id).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use to_do_dal::to_do_items::transactions::update::UpdateOneResponse;
    use std::future::Future;

    fn sample_item() -> ToDoItem {
        ToDoItem {
            id: 1,
            title: "coding".to_string(),
            status: "PENDING".to_string(),
        }
    }

    #[tokio::test]
    async fn test_update_ok() {
        struct ReturnOneMock;

        impl UpdateOne for ReturnOneMock {
            fn update_one(item: ToDoItem, user_id: i32)
                -> impl Future<Output = UpdateOneResponse> + Send {
                if item.title != "coding" {
                    panic!("Invalid title");
                }
                if user_id != 1 {
                    panic!("Invalid user_id");
                }
                async {
                    Ok(ToDoItem {
                        id: 1,
                        title: "coding".to_string(),
                        status: "DONE".to_string(),
                    })
                }
            }
        }
        let result = update::<ReturnOneMock>(sample_item(), 1).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_update_err() {
        struct ReturnErrorMock;

        impl UpdateOne for ReturnErrorMock {
            fn update_one(item: ToDoItem, user_id: i32)
                -> impl Future<Output = UpdateOneResponse> + Send {
                if item.title != "coding" {
                    panic!("Invalid title");
                }
                if user_id != 1 {
                    panic!("Invalid user_id");
                }
                async {
                    Err(NanoServiceError::new(
                        "Some Error".to_string(),
                        glue::errors::NanoServiceErrorStatus::NotFound,
                    ))
                }
            }
        }
        let result = update::<ReturnErrorMock>(sample_item(), 1).await;
        match result {
            Ok(_) => panic!("Expected error"),
            Err(e) => {
                assert_eq!(
                    e.status,
                    glue::errors::NanoServiceErrorStatus::NotFound
                );
                assert_eq!(e.message, "Some Error");
            }
        }
    }

    #[tokio::test]
    async fn test_update_err_invalid_title() {
        struct ReturnBadRequestMock;

        impl UpdateOne for ReturnBadRequestMock {
            fn update_one(item: ToDoItem, _user_id: i32)
                -> impl Future<Output = UpdateOneResponse> + Send {
                async move {
                    if item.title != "coding" {
                        return Err(NanoServiceError::new(
                            "Invalid title".to_string(),
                            glue::errors::NanoServiceErrorStatus::BadRequest,
                        ));
                    }
                    Ok(item)
                }
            }
        }
        let mut item = sample_item();
        item.title = "invalid".to_string();
        let result = update::<ReturnBadRequestMock>(item, 1).await;
        match result {
            Ok(_) => panic!("Expected error"),
            Err(e) => assert_eq!(
                e.status,
                glue::errors::NanoServiceErrorStatus::BadRequest
            ),
        }
    }

    #[tokio::test]
    async fn test_update_err_invalid_user_id() {
        struct ReturnBadRequestMock;

        impl UpdateOne for ReturnBadRequestMock {
            fn update_one(item: ToDoItem, user_id: i32)
                -> impl Future<Output = UpdateOneResponse> + Send {
                async move {
                    if user_id != 1 {
                        return Err(NanoServiceError::new(
                            "Invalid user_id".to_string(),
                            glue::errors::NanoServiceErrorStatus::BadRequest,
                        ));
                    }
                    Ok(item)
                }
            }
        }
        let result = update::<ReturnBadRequestMock>(sample_item(), 0).await;
        match result {
            Ok(_) => panic!("Expected error"),
            Err(e) => assert_eq!(
                e.status,
                glue::errors::NanoServiceErrorStatus::BadRequest
            ),
        }
    }
}

