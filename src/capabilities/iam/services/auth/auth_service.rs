use crate::capabilities::{
    common::config::config_service::ConfigService, iam::{entities::sessions, enums::auth_error::AuthError},
};
use chrono::Utc;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims<T> {
    pub payload: T,
    pub exp: i64,
}

/// Takes care of signing and verification of tokens
#[derive(Clone)]
pub struct AuthSerivce{
    /// Config service to extract auth config
    config: ConfigService,
    db: DatabaseConnection,
}

impl AuthSerivce {
    /// Create new auth service instance
    pub fn new(config: &ConfigService, db: DatabaseConnection) -> Self {
        return Self {
            config: *config,
            db,
        };
    }
    /// Create jwt from payload
    pub fn sign<T: Serialize + for<'b> Deserialize<'b>>(
        &self,
        payload: T,
        duration: i64,
        secret_key_var: Option<String>,
    ) -> Result<String, AuthError> {
        let key_var = secret_key_var.unwrap_or("IAM_JWT_SECRET".to_owned());
        let exp_result = Utc::now().checked_add_signed(chrono::Duration::seconds(duration));

        if exp_result.is_none() {
            return Err(AuthError::JWTDurationError);
        }
        let expiry = exp_result.unwrap().timestamp();
        let claims = Claims {
            payload,
            exp: expiry,
        };
        let header = Header::new(Algorithm::HS512);
        let secret: String = self.config.get_env(&key_var);
        let encoding_key = EncodingKey::from_secret(secret.as_bytes());
        match encode(&header, &claims, &encoding_key) {
            Ok(jwt) => return Ok(jwt),
            Err(e) => {
                tracing::error!("{}", e);
                return Err(AuthError::JWTSignError);
            }
        }
    }

    /// Verify and get data from
    pub fn verify<T: Serialize + for<'b> Deserialize<'b>>(
        &self,
        jwt: String,
        secret_key_var: Option<String>,
    ) -> Result<T, AuthError> {
        let key_var = secret_key_var.unwrap_or("IAM_JWT_SECRET".to_owned());
        let secret: String = self.config.get_env(&key_var);

        let mut validation = Validation::new(Algorithm::HS512);
        // force validation of expiry
        validation.validate_exp = true;

        let decode_result = decode::<Claims<T>>(
            &jwt,
            &DecodingKey::from_secret(secret.as_bytes()),
            &validation,
        );

        match decode_result {
            Ok(data) => return Ok(data.claims.payload),
            Err(e) => {
                tracing::error!("{}", e);
                if e.to_string() == "ExpiredSignature" {
                    return Err(AuthError::JWTExpirationError);
                } else {
                    return Err(AuthError::JWTVerificationError);
                }
            }
        }
    }

    /// Create session for user
    pub async fn create_session(&self, user_id: i32) -> Result<sessions::Model, AuthError> {
        let session = sessions::ActiveModel {
            session_id: Set(Uuid::new_v4()),
            user_id: Set(user_id),
            ..Default::default()
        };
        let insert_result = sessions::Entity::insert(session)
            .exec_with_returning(&self.db)
            .await;
        match insert_result {
            Ok(session) => Ok(session),
            Err(e) => {
                tracing::error!("{}", e);
                Err(AuthError::InternalServerError)
            }
        }
    }

    /// Delete single session
    pub async fn delete_session(&self, session_id: Uuid) -> Result<(), AuthError> {
        let session = sessions::ActiveModel {
            session_id: Set(session_id),
            ..Default::default()
        };
        let delete_result = sessions::Entity::delete(session).exec(&self.db).await;
        match delete_result {
            Ok(_) => Ok(()),
            Err(e) => {
                tracing::error!("{}", e);
                Err(AuthError::InternalServerError)
            }
        }
    }

    /// Log out all user sessions
    pub async fn delete_all_sessions(&self, user_id: i32) -> Result<(), AuthError> {
        let delete_result = sessions::Entity::delete_many()
            .filter(sessions::Column::UserId.eq(user_id))
            .exec(&self.db)
            .await;
        match delete_result {
            Ok(_) => Ok(()),
            Err(e) => {
                tracing::error!("{}", e);
                Err(AuthError::InternalServerError)
            }
        }
    }
}
