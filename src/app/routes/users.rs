use entity::users::Model as User;
use poem::web::Data;
use poem_openapi::{param::Path, payload::Json, types::Email, ApiResponse, Object, OpenApi, Tags};
use tera::Context;
use uuid::Uuid;

use crate::server::{app::AppState, mailer::MailUser, services, types::errors::ApiError};

#[derive(Debug, Object, Clone, Eq, PartialEq)]
pub struct CreateUser {
    #[oai(validator(min_length = 2))]
    name: String,
    email: Email,
}

#[derive(ApiResponse)]
enum CreateUserResponse {
    #[oai(status = 200)]
    Ok(Json<User>),
    #[oai(status = 409)]
    Conflict,
    #[oai(status = 500)]
    InternalServerError(Json<ApiError>),
}

#[derive(ApiResponse)]
enum GetUserResponse {
    #[oai(status = 200)]
    Ok(Json<User>),
    #[oai(status = 404)]
    NotFound,
    #[oai(status = 500)]
    InternalServerError(Json<ApiError>),
}

#[derive(Tags)]
enum ApiTags {
    /// Operations about user
    GetUserResponse,
    CreateUser,
}

#[derive(Default)]
pub struct Api;

#[OpenApi]
impl Api {
    #[oai(path = "/users/register", method = "post", tag = "ApiTags::CreateUser")]
    async fn register(
        &self,
        state: Data<&AppState>,
        payload: Json<CreateUser>,
    ) -> CreateUserResponse {
        match services::users::find_user_by_email(state.db.clone(), payload.email.to_string()).await
        {
            Ok(Some(_)) => return CreateUserResponse::Conflict,
            Err(e) => {
                return CreateUserResponse::InternalServerError(Json(ApiError::new(format!(
                    "{}",
                    e
                ))))
            }
            _ => (),
        }
        match services::users::create_user(
            state.db.clone(),
            payload.email.to_string(),
            payload.name.to_owned(),
        )
        .await
        {
            Err(e) => {
                CreateUserResponse::InternalServerError(Json(ApiError::new(format!("{}", e))))
            }
            Ok(Some(user)) => {
                let receiver = MailUser::new(Email(user.email.clone()), Some(user.name.clone()));
                let mut ctx = Context::new();
                ctx.insert("name", &user.name.clone().to_owned());
                ctx.insert("verification_link", "test");
                ctx.insert("sender", "Shivam Mathur");

                state.mailer.send_email(
                    "Welcome to KodingKorp!".to_string(), 
                    "verify_email.template".to_string(),
                    ctx,
                    receiver,
                    None
                ).await;
                CreateUserResponse::Ok(Json(user))
            },
            _ => CreateUserResponse::InternalServerError(Json(ApiError::new(
                "User not created".to_string(),
            ))),
        }
    }

    #[oai(path = "/users/:pid", method = "get", tag = "ApiTags::GetUserResponse")]
    async fn get(&self, state: Data<&AppState>, Path(pid): Path<Uuid>) -> GetUserResponse {
        match services::users::find_user_by_pid(state.db.clone(), pid).await {
            Ok(Some(user)) => GetUserResponse::Ok(Json(user)),
            Ok(None) => GetUserResponse::NotFound,
            Err(e) => GetUserResponse::InternalServerError(Json(ApiError::new(format!("{}", e)))),
        }
    }
}
