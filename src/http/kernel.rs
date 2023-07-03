use std::env;
use std::rc::Rc;

use actix_cors::Cors;
use actix_web::http::header;
use actix_web::web;
use actix_web::web::ServiceConfig;

use crate::http::controllers::routes;

pub fn register_routes(actix_config: &mut ServiceConfig) {
    log::debug!("Discovering routes...");

    for route in routes() {
        let route = Rc::new(route);
        for controller in &route.controllers {
            let path = route.prefix.as_str().to_owned() + controller.path.as_str();
            log::debug!(
                "Route Group: {}",
                if path.is_empty() { "/" } else { path.as_str() }
            );

            if path.is_empty() {
                actix_config.configure(controller.handler);
            } else if route.auth.is_some() {
                actix_config.service(
                    web::scope(path.as_str())
                        .wrap(route.auth.as_ref().cloned().unwrap())
                        .configure(controller.handler),
                );
            } else {
                actix_config.service(web::scope(path.as_str()).configure(controller.handler));
            }
        }
    }

    log::debug!("Route discovery finished :)");
}

pub fn setup_cors() -> Cors {
    let url_str = env::var("FRONTEND_ADDRESS").unwrap();
    let urls = url_str.as_str().split(",");

    let mut cors = Cors::default();
    for url in urls {
        cors = cors.allowed_origin(url);
    }

    cors
        .allowed_methods(vec!["GET", "POST", "PUT", "PATCH", "DELETE"])
        .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
        .allowed_header(header::CONTENT_TYPE)
        .max_age(3600)
}
