use actix_web::{get, HttpResponse};
use actix_web::web::ServiceConfig;
use crate::core::helpers::responder::json_success_message;

pub(crate) fn main_controller(cfg: &mut ServiceConfig) {
    cfg.service(index);
}

#[get("/")]
async fn index() -> HttpResponse {
    json_success_message("welcome :)")
}