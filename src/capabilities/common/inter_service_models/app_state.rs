use sea_orm::DatabaseConnection;

use crate::capabilities::*;
#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub services: ServiceList,
}

#[derive(Clone)]
pub struct ServiceList {
    pub iam: iam::services::iam::iam_service::IAMService,
    pub notifications: notifications::services::notification_service::NotificationService
}

