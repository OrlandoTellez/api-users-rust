use crate::models::response::ApiResponse;

pub fn success_response<T>(data: T, message: &str) -> ApiResponse<T> {
    ApiResponse {
        success: true,
        message: message.to_string(),
        data,
    }
}
