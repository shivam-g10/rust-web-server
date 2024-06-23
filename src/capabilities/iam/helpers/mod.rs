use std::path::Path;

use super::*;
use entities::users::Model as UserModel;
use uuid::Uuid;

use crate::capabilities::common::inter_service_models::session_user::SessionUser;


pub fn user_to_session(user: UserModel, session_id: Uuid) -> SessionUser {
    SessionUser {
        session_id,
        pid: user.pid,
        first_name: user.first_name,
        last_name: user.last_name,
        email: user.email
    }
}

pub fn get_render_template_path(template: String) -> String {
    let path_str = format!("{}/src/capabilities/iam/templates/{}.html", env!("CARGO_MANIFEST_DIR"), template);
    let path = Path::new(&path_str.to_string()).to_str().unwrap().to_string();
    path
}

pub fn get_subject_template_path(template: String) -> String {
    let path_str = format!("{}/src/capabilities/iam/templates/{}_subject.html", env!("CARGO_MANIFEST_DIR"), template);
    let path = Path::new(&path_str.to_string()).to_str().unwrap().to_string();
    path
}