// use sea_orm::{DatabaseBackend, Database};
// use mockall::*
// use serde::{Deserialize, Serialize};

// use super::auth_service::*;

// use crate::microservices::{common::config::config_service::ConfigService, iam::enums::auth_error::AuthError};

// #[test]
// fn should_sign_jwt() {
//     let config = ConfigService::new();
//     let mock_connection = mock!({DatabaseConnection});
//     let auth_service = AuthSerivce::new(&config, &mock_connection);
//     #[derive(Debug, Serialize, Deserialize)]
//     struct Test {
//         a: String,
//     }

//     let data = Test {
//         a: String::from("test")
//     };
//     let jwt_result = auth_service.sign(data, 60, None);
//     assert_ne!(jwt_result.is_err(), true);
//     assert_ne!(jwt_result.unwrap(), "");
// }

// #[test]
// fn should_verify_jwt() {
//     let config = ConfigService::new();
//     let mock_connection = MockDatabase::new(DatabaseBackend::Postgres).into_connection();
//     let auth_service = AuthSerivce::new(&config, mock_connection);
//     #[derive(Debug, Serialize, Deserialize, Clone)]
//     struct Test {
//         a: String,
//     }

//     let data = Test {
//         a: String::from("test")
//     };
//     let jwt_result = auth_service.sign(data.clone(), 60, None);
//     assert_ne!(jwt_result.is_err(), true);
    
//     let jwt = jwt_result.unwrap();
//     assert_ne!(jwt.clone(), "");
    
//     let verify_result = auth_service.verify::<Test>(jwt, None);
//     assert_ne!(verify_result.is_err(), true);

//     let payload = verify_result.unwrap();
//     assert_eq!(payload.a, data.a);
// }

// #[test]
// fn should_reject_expired_tokens() {
//     let config = ConfigService::new();
//     let mock_connection = MockDatabase::new(DatabaseBackend::Postgres).into_connection();
//     let auth_service = AuthSerivce::new(&config, &mock_connection);
//     #[derive(Debug, Serialize, Deserialize, Clone)]
//     struct Test {
//         a: String,
//     }

//     let data = Test {
//         a: String::from("test")
//     };
//     let jwt_result = auth_service.sign(data.clone(), -1800, None);
//     assert_ne!(jwt_result.is_err(), true);
    
//     let jwt = jwt_result.unwrap();
//     assert_ne!(jwt.clone(), "");
//     println!("{}", jwt.clone());
    
//     let verify_result = auth_service.verify::<Test>(jwt, None);
//     assert_eq!(verify_result.is_err(), true);
//     match verify_result {
//         Err(e) => assert_eq!(e, AuthError::JWTExpirationError),
//         _ => ()
//     };
// }