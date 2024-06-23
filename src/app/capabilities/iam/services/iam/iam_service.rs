use std::fmt::Display;
use std::path::Path;

use crate::capabilities::common::*;
use crate::capabilities::notifications::models::notification_builder::{NotificationConfigType, UserNotificationType};
use crate::capabilities::notifications::services::notification_service::NotificationService;
use config::config_service::ConfigService;
use constants::Constants;
use entities::users::{Entity as UserEntity, Model as UserModel};
use enums::auth_error::AuthError;
use inter_service_models::session_user::SessionUser;
use sea_orm::DatabaseConnection;

use super::super::super::*;
use models::auth_bearer::AuthBearer;
use services::auth::auth_service::AuthSerivce;
use services::users::users::*;


pub enum IAMError {
    InternalServerError
}

#[derive(Debug)]
pub enum IAMNotifications {
    MagicLink
}

impl Display for IAMNotifications {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone)]
pub struct IAMService {
    users: UserService,
    auth: AuthSerivce,
    iam_constants: Constants,
    notification: NotificationService
}

impl IAMService {
    pub fn new(db: DatabaseConnection, config: &ConfigService, mut notification: NotificationService) -> Self {
        let mut magic_link_builder = notification.get_builder(IAMNotifications::MagicLink.to_string(), NotificationConfigType::Email);
        magic_link_builder.set_subject_template(helpers::get_subject_template_path("magic_link".to_owned()));
        magic_link_builder.set_render_template(helpers::get_render_template_path("magic_link".to_owned()));
        magic_link_builder.add_replace_settings("link".to_owned(), "".to_owned(), false);
        magic_link_builder.set_notification_type(UserNotificationType::LoginLink);
        let magic_link_notif = magic_link_builder.build().unwrap();
        notification.register(magic_link_notif);
        Self { 
            users: UserService::new(db.clone()),
            auth: AuthSerivce::new(config, db.clone()),
            iam_constants: Constants::new(),
            notification
        }
    }
    /// Execute login logic
    pub async fn login(&self, magic_jwt: String) -> Result<AuthBearer, AuthError> {
        let verify_result = self.auth.verify::<SessionUser>(magic_jwt, Some(self.iam_constants.jwt_key_var.clone()));
        match verify_result {
            Ok(session_user) => {
                match self.users.find_user_by_pid(session_user.pid).await {
                    Ok(user_result) => {
                        match user_result {
                            Some(user) => {
                                return self.create_session_for_user(user).await;
                            },
                            None => {
                                return Err(AuthError::NotFound);
                            }
                        }
                    },
                    Err(_) => {
                        return Err(AuthError::InternalServerError);
                    }
                }
            },
            Err(e) => {
                Err(e)
            }
        }
    }

    /// Send Login Link
    pub async fn send_login_link(&self, email: String) -> Result<(), AuthError> {
        match self.users.find_active_user_by_email(email.clone()).await {
            Ok(Some(_)) => {
                // TODO: Send Login Link by Email
                return Ok(());
            },
            Err(e) => {
                tracing::error!("{}", e);
                return Err(AuthError::InternalServerError);
            },
            _ => return Ok(()),
        };
    }

    /// Send Login Link
    pub async fn send_verification_link(&self, email: String) -> Result<(), AuthError> {
        match self.users.find_active_user_by_email(email.clone()).await {
            Ok(Some(_)) => {
                // TODO: Send Login Link by Email
                return Ok(());
            },
            Err(e) => {
                tracing::error!("{}", e);
                return Err(AuthError::InternalServerError);
            },
            _ => return Err(AuthError::NotFound),
        };
    }

    /// Execute register logic
    pub async fn register(&self, email: String, first_name: String, last_name: String) -> Result<AuthBearer, AuthError> {
        match self.users.find_active_user_by_email(email.clone()).await {
            Ok(Some(_)) => {
                return Err(AuthError::Conflict);
            },
            Err(e) => {
                tracing::error!("{}", e);
                return Err(AuthError::InternalServerError);
            },
            _ => {();},
        };

        match self.users.create_user(email, first_name, last_name).await {
            Ok(Some(user)) => {
                return self.create_session_for_user(user).await;
            },
            Err(e) => {
                tracing::error!("{}", e);
                return Err(AuthError::InternalServerError);
            },
            _ => {
                tracing::error!("User not created");
                return Err(AuthError::InternalServerError);
            }
        }
    }

    /// Verify Auth Session
    pub async fn verify_token() -> Result<SessionUser, AuthError> {
        Err(AuthError::JWTVerificationError)
    }

    /// Get user data from session
    pub async fn get_user() -> Result<UserEntity, IAMError> {
        Err(IAMError::InternalServerError)
    }

    /// Delete/Close user account
    pub async fn close_account() -> Result<UserEntity, IAMError> {
        Err(IAMError::InternalServerError)
    }

    /// create auth bearer from user
    async fn create_session_for_user(&self, user: UserModel) -> Result<AuthBearer, AuthError> {
        match self.auth.create_session(user.id).await {
            Ok(session) => {
                let session_user = SessionUser {
                    pid: user.pid,
                    email: user.email,
                    first_name: user.first_name,
                    last_name: user.last_name,
                    session_id: session.session_id.clone(),
                };
                match self.auth.sign(
                    session_user.clone(), 
                    self.iam_constants.login_duration, 
                    Some(self.iam_constants.jwt_key_var.clone())
                ) {
                    Ok(jwt) => return Ok(AuthBearer {
                        token: jwt,
                        session_user: Some(session_user)
                    }),
                    Err(e) => return Err(e),
                }
            },
            Err(e) => return Err(e),
        };
    }
}