use std::time::Duration;

use migration::{sea_orm, MigratorTrait};
use poem::{middleware::AddDataEndpoint, EndpointExt, Route};
use poem_openapi::OpenApiService;
use sea_orm::DatabaseConnection;

use crate::app::capabilities::{
    common::{
        config::config_service::ConfigService,
        inter_service_models::app_state::{AppState, ServiceList},
    },
    iam::{controllers::authentication::auth_controllers, services::iam::iam_service::IAMService},
};

use super::{capabilities::iam::controllers::users::users_controller, routes};

pub async fn build_app() -> AddDataEndpoint<Route, AppState> {

    let state = make_app_state().await;

    let api_list = (
        routes::base::Api::default(),
        auth_controllers::API::default(),
        users_controller::API::default()
    );
    let all_apis = OpenApiService::new(api_list, "Prod APIs", "1.0").url_prefix("/api");
    let base_apis = OpenApiService::new(routes::base::Api::default(), "Base", "1.0");
    let ui = all_apis.swagger_ui();
    Route::new()
        .nest("/", base_apis)
        .nest("/api", all_apis)
        .nest("/swagger", ui)
        .data(state)
}

async fn make_app_state() -> AppState {
    let db = make_db_connection().await;
    tracing::debug!("DB Connection Created");
    let config = ConfigService::new();
    AppState {
        db: db.clone(),
        services: ServiceList {
            iam: IAMService::new(db, &config.clone()),
        },
    }
}

async fn make_db_connection() -> DatabaseConnection {
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let auto_migrate = std::env::var("AUTO_MIGRATE").expect("AUTO_MIGRATE is not set in .env file");

    let mut opts = sea_orm::ConnectOptions::new(db_url);
    opts.max_connections(10)
        .min_connections(1)
        .connect_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true);

    let conn = sea_orm::Database::connect(opts)
        .await
        .expect("Database connection failed");

    if auto_migrate == "TRUE" {
        match migration::Migrator::up(&conn, None).await {
            Ok(_) => tracing::debug!("Migrations successful"),
            Err(e) => {
                tracing::error!("Migrations failed");
                panic!("{}", e);
            }
        }
    }
    conn
}
