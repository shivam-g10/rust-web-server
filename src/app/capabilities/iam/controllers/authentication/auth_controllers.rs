use poem::web::Data;
use poem_openapi::{payload::Json, types::Email, ApiResponse, Object, OpenApi, Tags};

use crate::app::capabilities::{
    common::inter_service_models::{api_error::ApiError, app_state::AppState},
    iam::{enums::auth_error::AuthError, models::auth_bearer::AuthBearer},
};

#[derive(Debug, Object, Clone, Eq, PartialEq)]
pub struct CreateUser {
    #[oai(validator(min_length = 2))]
    first_name: String,
    #[oai(validator(min_length = 2))]
    last_name: String,
    #[oai(validator(min_length = 8))]
    password: String,
    email: Email,
}


#[derive(Debug, Object, Clone, Eq, PartialEq)]
pub struct Login {
    email: String,
    password: String,
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

#[derive(ApiResponse)]
pub enum LoginResponse {
    #[oai(status = 200)]
    Ok(Json<AuthBearer>),
    #[oai(status = 404)]
    NotFound,
    #[oai(status = 500)]
    InternalServerError(Json<ApiError>),
}

#[derive(Tags)]
enum ApiTags {
    /// Operations about user
    CreateUser,
    Login,
}

#[derive(Default)]
pub struct API;

#[OpenApi]
impl API {
    #[oai(path = "/auth/login", method = "post", tag = "ApiTags::Login")]
    pub async fn login(&self, state: Data<&AppState>, payload: Json<Login>) -> LoginResponse {
        match state.services.iam.login(payload.email.clone(), payload.password.clone()).await {
            Err(e) => match e {
                AuthError::NotFound => return LoginResponse::NotFound,
                _ => LoginResponse::InternalServerError(Json(ApiError::new(format!("{:?}", e)))),
            },
            Ok(ab) => { 
                // TODO: Trigger send email
                return LoginResponse::Ok(Json(ab));
            },
        }
    }
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
                payload.password.clone(),
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
}
