use poem_openapi::Object;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, Object)]
pub struct SessionUser {
    pub session_id: Uuid,
    pub pid: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}
