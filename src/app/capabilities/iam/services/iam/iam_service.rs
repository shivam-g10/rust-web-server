use crate::app::capabilities::common::*;
use config::config_service::ConfigService;
use constants::Constants;
use entities::users::Model as UserModel;
use enums::auth_error::AuthError;
use global_model::session_user::SessionUser;
use sea_orm::DatabaseConnection;

use super::super::super::*;
use models::auth_bearer::AuthBearer;
use services::auth::auth_service::AuthSerivce;
use services::users::users::*;


#[derive(Debug)]
pub enum IAMError {
    InternalServerError
}

#[derive(Clone)]
pub struct IAMService {
    users: UserService,
    auth: AuthSerivce,
    iam_constants: Constants,
}

impl IAMService {
    pub fn new(db: DatabaseConnection, config: &ConfigService) -> Self {
        Self { 
            users: UserService::new(db),
            auth: AuthSerivce::new(config),
            iam_constants: Constants::new(),
        }
    }
    /// Execute login logic
    pub async fn login(&self, email: String, password: String) -> Result<AuthBearer, AuthError> {
        match self.users.find_user_by_email(email).await {
            Ok(Some(user)) => {
                if self.auth.bcrypt_verify_hash(password, user.password.clone().unwrap()) {
                    return self.create_session_for_user(user).await;
                } else {
                    return Err(AuthError::NotFound);
                }
            },
            _ => {
                return Err(AuthError::NotFound);
            }
        }
    }

    /// Execute register logic
    pub async fn register(&self, email: String, first_name: String, last_name: String, password: String) -> Result<AuthBearer, AuthError> {
        match self.users.find_user_by_email(email.clone()).await {
            Ok(Some(_)) => {
                return Err(AuthError::Conflict);
            },
            Err(e) => {
                tracing::error!("{}", e);
                return Err(AuthError::InternalServerError);
            },
            _ => {();},
        };

        match self.users.create_user(email, first_name, last_name, self.auth.hash(password)).await {
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
    pub async fn verify_token(&self, jwt: String) -> Result<SessionUser, AuthError> {
        match self.auth.verify::<SessionUser>(jwt, Some(self.iam_constants.jwt_key_var.clone())) {
            Ok(session_user) => Ok(session_user),
            Err(e) =>  Err(e),
        }
       
    }

    /// Get user data from session
    pub async fn get_user(&self, session_user: SessionUser) -> Result<UserModel, IAMError> {
        match self.users.find_user_by_pid(session_user.pid).await {
            Ok(Some(mut user)) => {
                user.password = None;
                return Ok(user);
            },
            Err(e) => { 
                tracing::error!("{}", e);
                return Err(IAMError::InternalServerError);
            },
            _ => return Err(IAMError::InternalServerError)
        }
    }

    /// create auth bearer from user
    async fn create_session_for_user(&self, user: UserModel) -> Result<AuthBearer, AuthError> {
        let session_user = helpers::user_to_session(user);
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
    }
}
