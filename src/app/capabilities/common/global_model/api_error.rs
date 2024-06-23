use poem_openapi::Object;

#[derive(Object, Clone, Debug, Default)]
pub struct ApiError {
    message: String
}

impl ApiError {
    pub fn new(message: String) -> ApiError {
        ApiError {
            message
        }
    } 
}
