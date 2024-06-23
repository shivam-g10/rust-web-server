use poem_openapi::Object;
use serde::{Deserialize, Serialize};

use crate::app::capabilities::common::inter_service_models::session_user::SessionUser;

#[derive(Debug, Serialize, Deserialize, Clone, Object)]
pub struct AuthBearer {
    pub token: String,
    pub session_user: Option<SessionUser>
}
