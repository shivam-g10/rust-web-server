use crate::app::capabilities::{
    common::config::config_service::ConfigService, iam::enums::auth_error::AuthError,
};
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::Utc;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

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
}

impl AuthSerivce {
    /// Create new auth service instance
    pub fn new(config: &ConfigService) -> Self {
        return Self {
            config: *config,
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

    /// Create hash of string
    pub fn hash(&self, string: String) -> String {
        hash(string, DEFAULT_COST).unwrap()
    }

    /// verify a hash
    pub fn bcrypt_verify_hash(&self, string: String, hash: String) -> bool {
        match verify(string, &hash) {
            Ok(_) => true,
            Err(_) => false,
        }
    }
}
