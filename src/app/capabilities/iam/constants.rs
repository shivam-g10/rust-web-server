use chrono::Duration;

#[derive(Clone)]
pub struct Constants {
    pub jwt_key_var: String,
    pub login_duration: i64,
    pub magic_link_duration: i64,
}
impl Constants {
    pub fn new() -> Constants {
        Constants { 
            jwt_key_var: "IAM_JWT_SECRET".to_string(),
            login_duration: Duration::days(7).num_seconds(),
            magic_link_duration: Duration::minutes(15).num_seconds(),
        }
    }
}
