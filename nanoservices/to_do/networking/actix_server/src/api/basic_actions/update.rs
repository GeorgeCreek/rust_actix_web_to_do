use to_do_core::api::basic_actions::{
    update::update as update_core,
    get::get_all as get_all_core
};
use glue::{
    errors::{NanoServiceError, NanoServiceErrorStatus},
    token::HeaderToken
};
use actix_web::{
    HttpResponse,
    web::Json
};
use to_do_dal::to_do_items::transactions::{
    update::UpdateOne,
    get::GetAll
};
use to_do_dal::to_do_items::schema::ToDoItem;
use super::resolve_user::ResolveUserId;


pub use super::resolve_user::AuthKernelUser;


/// Update an item in the to do list.
/// 
/// # Arguments
/// * `body` - The item to update.
/// 
/// # Returns
/// A JSON response containing all items in the to do list.
pub async fn update<T, X>(
    token: HeaderToken, 
    body: Json<ToDoItem>
) -> Result<HttpResponse, NanoServiceError>
where
    T: UpdateOne + GetAll,
    X: ResolveUserId
{
    let user_id = X::resolve_user_id(token.unique_id).await?;
    let _ = update_core::<T>(body.into_inner(), user_id).await?;
    Ok(HttpResponse::Ok().json(get_all_core::<T>(
        user_id
    ).await?))
}


#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{
        body::MessageBody,
        http::header::ContentType,
        test::{call_service, init_service, TestRequest},
        web, App,
    };
    use actix_http::Request;
    use actix_web::dev::ServiceResponse;
    use to_do_dal::to_do_items::transactions::update::UpdateOneResponse;
    use to_do_dal::to_do_items::schema::AllToDOItems;
    use std::future::Future;


    fn generate_to_do_item() -> ToDoItem {
        ToDoItem {
            id: 1,
            title: "coding".to_string(),
            status: "PENDING".to_string(),
        }
    }

    fn generate_get_all_return() -> Vec<ToDoItem> {
        vec![
            generate_to_do_item()
        ]
    }

    struct MockUserHandle;

    impl ResolveUserId for MockUserHandle {
        fn resolve_user_id(unique_id: String)
            -> impl Future<Output = Result<i32, NanoServiceError>> + Send {
            async move {
                if unique_id == "break" {
                    return Err(NanoServiceError::new(
                        "User not found".to_string(),
                        NanoServiceErrorStatus::NotFound,
                    ));
                }
                if unique_id == "2" {
                    return Ok(2);
                }
                Ok(1)
            }
        }
    }

    struct MockDbHandle;

    impl UpdateOne for MockDbHandle {
        fn update_one(item: ToDoItem, _user_id: i32)
            -> impl Future<Output = UpdateOneResponse> + Send {
            async move {
                if item.title == "coding" {
                    return Ok(item);
                }
                Err(NanoServiceError::new(
                    "Item not found".to_string(),
                    NanoServiceErrorStatus::NotFound,
                ))
            }
        }
    }

    impl GetAll for MockDbHandle {
        fn get_all(user_id: i32)
            -> impl Future<Output = Result<Vec<ToDoItem>, NanoServiceError>> + Send {
            async move {
                if user_id == 2 {
                    return Err(NanoServiceError::new(
                        "error getting items got get all".to_string(),
                        NanoServiceErrorStatus::Unknown,
                    ));
                }
                Ok(generate_get_all_return())
            }
        }
    }

    async fn run_request(req: Request) -> ServiceResponse {
        let service = update::<MockDbHandle, MockUserHandle>;
        let app = init_service(App::new().route(
            "/update",
            web::put().to(service)
        )).await;
        call_service(&app, req).await
    }

    #[tokio::test]
    async fn test_update_ok() {
        std::env::set_var("JWT_SECRET", "secret");
        let req = TestRequest::put()
            .insert_header(ContentType::json())
            .insert_header((
                "token",
                HeaderToken::new("test_id".to_string()).encode().unwrap()
            ))
            .set_json(generate_to_do_item())
            .uri("/update")
            .to_request();
        let resp = run_request(req).await;
        let status = resp.status().as_u16();
        let raw_body = resp.into_body().try_into_bytes().unwrap();
        let body_str = std::str::from_utf8(&raw_body).unwrap();
        let body: AllToDOItems = serde_json::from_str(body_str).unwrap();

        assert_eq!(status, 200);
        assert_eq!(
            body,
            AllToDOItems::from_vec(generate_get_all_return()).unwrap()
        );
    }

    #[tokio::test]
    async fn test_update_invalid_token() {
        std::env::set_var("JWT_SECRET", "secret");

        let req = TestRequest::put()
            .insert_header(ContentType::json())
            .insert_header(("token", "test"))
            .set_json(generate_to_do_item())
            .uri("/update")
            .to_request();
        let resp = run_request(req).await;

        let status = resp.status().as_u16();
        let raw_body = resp.into_body().try_into_bytes().unwrap();
        let body_str = std::str::from_utf8(&raw_body).unwrap();

        assert_eq!(status, 401);
        assert_eq!(body_str, "\"token not a valid string\"");
    }

    #[tokio::test]
    async fn test_update_user_not_found() {
        std::env::set_var("JWT_SECRET", "secret");

        let req = TestRequest::put()
            .insert_header(ContentType::json())
            .insert_header((
                "token",
                HeaderToken::new("break".to_string()).encode().unwrap()
            ))
            .set_json(generate_to_do_item())
            .uri("/update")
            .to_request();
        let resp = run_request(req).await;

        let status = resp.status().as_u16();
        let raw_body = resp.into_body().try_into_bytes().unwrap();
        let body_str = std::str::from_utf8(&raw_body).unwrap();

        assert_eq!(status, 404);
        assert_eq!(body_str, "\"User not found\"");
    }

    #[tokio::test]
    async fn test_update_item_not_found() {
        std::env::set_var("JWT_SECRET", "secret");

        let mut item = generate_to_do_item();
        item.title = "break".to_string();

        let req = TestRequest::put()
            .insert_header(ContentType::json())
            .insert_header((
                "token",
                HeaderToken::new("test_id".to_string()).encode().unwrap()
            ))
            .set_json(item)
            .uri("/update")
            .to_request();
        let resp = run_request(req).await;

        let status = resp.status().as_u16();
        let raw_body = resp.into_body().try_into_bytes().unwrap();
        let body_str = std::str::from_utf8(&raw_body).unwrap();

        assert_eq!(status, 404);
        assert_eq!(body_str, "\"Item not found\"");
    }

    #[tokio::test]
    async fn test_update_get_all_error() {
        std::env::set_var("JWT_SECRET", "secret");

        let req = TestRequest::put()
            .insert_header(ContentType::json())
            .insert_header((
                "token",
                HeaderToken::new("2".to_string()).encode().unwrap()
            ))
            .set_json(generate_to_do_item())
            .uri("/update")
            .to_request();
        let resp = run_request(req).await;

        let status = resp.status().as_u16();
        let raw_body = resp.into_body().try_into_bytes().unwrap();
        let body_str = std::str::from_utf8(&raw_body).unwrap();

        assert_eq!(status, 500);
        assert_eq!(body_str, "\"error getting items got get all\"");
    }
}
