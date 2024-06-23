use super::{entities::users::Model as UserModel, models::user_data::UserData};

use crate::app::capabilities::common::inter_service_models::session_user::SessionUser;


pub fn user_to_session(user: UserModel) -> SessionUser {
    SessionUser {
        pid: user.pid,
        first_name: user.first_name,
        last_name: user.last_name,
        email: user.email
    }
}

pub fn extract_user_api_data(user: UserModel) -> UserData {
    UserData {
        pid: user.pid,
        first_name: user.first_name,
        last_name: user.last_name,
        email: user.email,
        created_at: user.created_at.to_string(),
        updated_at: user.updated_at.to_string()
    }
}
