use poem::web::Data;
use poem_openapi::{payload::Json, ApiResponse, OpenApi, Tags};

use crate::app::capabilities::{
    common::global_model::{api_error::ApiError, app_state::AppState, session_user::SessionUser},
    iam::{helpers, models::user_data::UserData},
};

use poem::Request;
use poem_openapi::{auth::Bearer, SecurityScheme};

/// Bearer authorization
#[derive(SecurityScheme)]
#[oai(
    ty = "bearer",
    checker = "api_checker"
)]
pub struct JWTAuth(SessionUser);
pub async fn api_checker(req: &Request, bearer: Bearer) -> Option<SessionUser> {
    let state = req.data::<AppState>().unwrap();
    match state.services.iam.verify_token(bearer.token).await {
        Ok(session_user) => return Some(session_user),
        _ => return None,
    }
}


#[derive(ApiResponse)]
pub enum GetUserResponse {
    #[oai(status = 200)]
    Ok(Json<UserData>),
    #[oai(status = 500)]
    InternalServerError(Json<ApiError>),
}


#[derive(Tags)]
enum ApiTags {
    /// Operations about user
    GetUser
}

#[derive(Default)]
pub struct API;

#[OpenApi]
impl API {
    #[oai(path = "/users/me", method = "get", tag = "ApiTags::GetUser")]
    pub async fn get_user(&self, state: Data<&AppState>, session_user: JWTAuth) -> GetUserResponse {
        match state.services.iam.get_user(session_user.0).await {
            Err(e) => return GetUserResponse::InternalServerError(Json(ApiError::new(format!("{:?}", e)))),
            Ok(user) => { 
                // TODO: Trigger send email
                return GetUserResponse::Ok(Json(helpers::extract_user_api_data(user)));
            },
        }
    }
}
