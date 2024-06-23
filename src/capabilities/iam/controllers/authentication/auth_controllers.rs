use poem::web::Data;
use poem_openapi::{payload::Json, types::Email, ApiResponse, Object, OpenApi, Tags};

use crate::capabilities::{
    common::inter_service_models::{api_error::ApiError, app_state::AppState},
    iam::{enums::auth_error::AuthError, models::auth_bearer::AuthBearer},
};

#[derive(Debug, Object, Clone, Eq, PartialEq)]
pub struct CreateUser {
    #[oai(validator(min_length = 2))]
    first_name: String,
    #[oai(validator(min_length = 2))]
    last_name: String,
    email: Email,
}

#[derive(Debug, Object, Clone, Eq, PartialEq)]
pub struct SendLoginLink {
    email: Email,
}

#[derive(Debug, Object, Clone, Eq, PartialEq)]
pub struct Login {
    token: String,
}


#[derive(ApiResponse)]
pub enum RegisterResponse {
    #[oai(status = 200)]
    Ok(Json<AuthBearer>),
    #[oai(status = 404)]
    NotFound,
    #[oai(status = 409)]
    Conflict,
    #[oai(status = 500)]
    InternalServerError(Json<ApiError>),
}

#[derive(Tags)]
enum ApiTags {
    /// Operations about user
    SendLoginLink,
    CreateUser,
    Login,
}

#[derive(Default)]
pub struct API;

#[OpenApi]
impl API {
    /// Login
    #[oai(path = "/auth/send_login_link", method = "post", tag = "ApiTags::SendLoginLink")]
    pub async fn send_login_link(&self, state: Data<&AppState>, payload: Json<SendLoginLink>) {}

    #[oai(path = "/auth/login", method = "post", tag = "ApiTags::Login")]
    pub async fn login(&self, state: Data<&AppState>, payload: Json<Login>) {}
    /// Create and return new user
    #[oai(path = "/auth/register", method = "post", tag = "ApiTags::CreateUser")]
    pub async fn register(
        &self,
        state: Data<&AppState>,
        payload: Json<CreateUser>,
    ) -> RegisterResponse {
        match state
            .services
            .iam
            .register(
                payload.email.to_string(),
                payload.first_name.clone(),
                payload.last_name.clone(),
            )
            .await
        {
            Err(e) => match e {
                AuthError::Conflict => return RegisterResponse::Conflict,
                _ => RegisterResponse::InternalServerError(Json(ApiError::new(format!("{:?}", e)))),
            },
            Ok(ab) => { 
                // TODO: Trigger send email
                return RegisterResponse::Ok(Json(ab));
            },
        }
    }
    /// Clear session
    pub async fn logout() {}
}
