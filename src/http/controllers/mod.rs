mod main_controller;
mod movie_controller;

use actix_web::web::ServiceConfig;
use crate::http::controllers::main_controller::main_controller;
use crate::http::controllers::movie_controller::movie_controller;
use crate::http::middlewares::auth_middleware::Auth;

#[derive(Clone)]
pub struct Controller {
    pub path: String,
    pub handler: fn(cfg: &mut ServiceConfig),
}

#[derive(Clone)]
pub struct Route<T> {
    pub prefix: String,
    pub auth: Option<T>,
    pub controllers: Vec<Controller>,
}

pub fn routes() -> Vec<Route<Auth>> {
    vec![
        Route {
            auth: None,
            prefix: String::from(""),
            controllers: vec![Controller {
                path: String::from(""),
                handler: main_controller,
            }],
        },
        Route {
            auth: None,
            prefix: String::from("/api/v1"),
            controllers: vec![Controller {
                path: String::from("/movies"),
                handler: movie_controller,
            }],
        },
    ]
}