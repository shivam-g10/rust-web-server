use std::time::Duration;

use migration::{sea_orm, MigratorTrait};
use poem::{middleware::AddDataEndpoint, EndpointExt, Route};
use poem_openapi::OpenApiService;
use sea_orm::DatabaseConnection;

use crate::capabilities::{
    common::{
        config::config_service::ConfigService,
        inter_service_models::app_state::{AppState, ServiceList},
        mailer::{MailUser, Mailer},
    },
    iam::{controllers::authentication::auth_controllers, services::iam::iam_service::IAMService},
    notifications::services::notification_service::NotificationService,
};

use super::routes;

pub fn build_app(state: AppState) -> AddDataEndpoint<Route, AppState> {
    let api_list = (
        routes::base::Api::default(),
        auth_controllers::API::default(),
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

pub async fn make_app_state() -> AppState {
    let db = make_db_connection().await;
    tracing::debug!("DB Connection Created");
    let config = ConfigService::new();
    let mailer = make_mailer();
    tracing::debug!("Mailer Created");
    let notification = NotificationService::new(config, mailer);
    AppState {
        db: db.clone(),
        services: ServiceList {
            iam: IAMService::new(db, &config.clone(), notification.clone()),
            notifications: notification,
        },
    }
}

pub async fn make_db_connection() -> DatabaseConnection {
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

pub fn make_mailer() -> Mailer {
    let default_from = MailUser::new(
        poem_openapi::types::Email(
            std::env::var("DEFAULT_SENDER_EMAIL")
                .expect("DEFAULT_SENDER_EMAIL is not set in .env file"),
        ),
        Some(
            std::env::var("DEFAULT_SENDER_EMAIL_NAME")
                .expect("DEFAULT_SENDER_EMAIL_NAME is not set in .env file"),
        ),
    );

    let default_reply_to = MailUser::new(
        poem_openapi::types::Email(
            std::env::var("DEFAULT_REPLY_TO_EMAIL")
                .expect("DEFAULT_REPLY_TO_EMAIL is not set in .env file"),
        ),
        Some(
            std::env::var("DEFAULT_REPLY_TO_NAME")
                .expect("DEFAULT_REPLY_TO_NAME is not set in .env file"),
        ),
    );
    let username = std::env::var("SMTP_SERVER_USERNAME")
        .expect("SMTP_SERVER_USERNAME is not set in .env file");

    let password = std::env::var("SMTP_SERVER_PASSWORD")
        .expect("SMTP_SERVER_PASSWORD is not set in .env file");

    let smtp = std::env::var("SMTP_SERVER").expect("SMTP_SERVER is not set in .env file");
    Mailer::new(username, password, smtp, default_from, default_reply_to)
}
