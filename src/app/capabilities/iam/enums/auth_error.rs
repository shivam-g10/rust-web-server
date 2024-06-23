use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum AuthError {
    JWTDurationError,
    JWTExpirationError,
    JWTSignError,
    JWTVerificationError,
    NotFound,
    InternalServerError,
    Conflict,
    BadRequest,
}
