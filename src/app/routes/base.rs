use poem::web::Data;
use poem_openapi::{payload::Json, ApiResponse, Object, OpenApi, Tags};

use crate::app::capabilities::common::global_model::app_state::AppState;

#[derive(Debug, Object, Clone, Eq, PartialEq)]
pub struct Ping {
    up: bool
}
#[derive(Debug, Object, Clone, Eq, PartialEq)]
pub struct Health {
    db: bool,
}

#[derive(ApiResponse)]
pub enum PingResponse {
    #[oai(status = 200)]
    Ok(Json<Ping>)
}


#[derive(ApiResponse)]
pub enum HealthResponse {
    #[oai(status = 200)]
    Ok(Json<Health>)
}


#[derive(Tags)]
enum ApiTags {
    /// Operations about user
    PingResponse,
    HealthResponse
}

#[derive(Default)]
pub struct Api;


#[OpenApi]
impl Api {
    #[oai(path = "/ping", method = "get", tag = "ApiTags::PingResponse")]
    pub async fn ping(&self) -> PingResponse {
        PingResponse::Ok(Json(Ping { up: true })) 
    }

    #[oai(path = "/health", method = "get", tag = "ApiTags::HealthResponse")]
    pub async fn health(&self, state: Data<&AppState>) -> HealthResponse {
        let mut db_state = false;
        match state.db.ping().await {
            Ok(_) => db_state = true,
            Err(e) => tracing::error!("{}", e)
        }
        HealthResponse::Ok(Json(Health {
            db: db_state,
        }))
    }
}
