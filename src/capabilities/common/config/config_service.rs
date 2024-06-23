use std::env;

#[derive(Debug, Clone, Copy)]
pub struct ConfigService;

impl ConfigService {
    pub fn new() -> ConfigService {
        dotenvy::dotenv().ok();
        ConfigService {}
    }
    pub fn get_env<T: std::str::FromStr + Default>(self, key: &str) -> T {
        let result = env::var(key);
        match result {
            Ok(s) => match s.parse() {
                Ok(val) => val,
                Err(_) => {
                    tracing::error!("Error parsing {}", key);
                    String::from("").parse().unwrap_or_default()
                }
            },
            Err(_) => String::from("").parse().unwrap_or_default()
        }
    }
}