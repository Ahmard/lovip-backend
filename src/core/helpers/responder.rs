use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct JsonResponse<T: Serialize> {
    success: bool,
    data: T,
    status: u16,
}

#[derive(Serialize, Deserialize)]
pub struct JsonSuccessMessageResponse {
    message: String,
}

pub fn json<T: Serialize>(data: T, status: StatusCode) -> HttpResponse {
    HttpResponse::build(status).json(data)
}

pub fn json_success<T: Serialize>(data: T) -> HttpResponse {
    json(
        JsonResponse {
            success: true,
            status: 200,
            data,
        },
        StatusCode::OK,
    )
}

pub fn json_error<T: Serialize>(data: T, status: StatusCode) -> HttpResponse {
    json(
        JsonResponse {
            success: false,
            status: status.as_u16(),
            data,
        },
        status,
    )
}

pub fn json_error_message(message: &str) -> HttpResponse {
    json_error_message_status(message, StatusCode::BAD_REQUEST)
}

pub fn json_error_message_status(message: &str, status: StatusCode) -> HttpResponse {
    json_error(
        JsonSuccessMessageResponse {
            message: message.to_string(),
        },
        status,
    )
}

pub fn json_success_message(message: &str) -> HttpResponse {
    json_success(JsonSuccessMessageResponse {
        message: message.to_string(),
    })
}

#[allow(dead_code)]
pub fn json_unauthorized_message(message: &str) -> HttpResponse {
    json_error_message_status(message, StatusCode::UNAUTHORIZED)
}

pub fn json_not_found_response(message: Option<&str>) -> HttpResponse {
    json_error_message_status(message.unwrap_or("Not Found"), StatusCode::NOT_FOUND)
}
