mod http;
mod core;

use std::env;
use actix_files::Files;
use actix_web::{App, HttpServer, web};
use actix_web::middleware::Logger;
use env_logger::Env;
use crate::core::helpers::responder::json_not_found_response;
use crate::http::kernel::{register_routes, setup_cors};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let host: String = env::var("HOST").unwrap();
    let port: u16 = env::var("PORT").unwrap().parse().unwrap();

    env::set_var("RUST_LOG", "debug");
    env::set_var("RUST_BACKTRACE", "1");
    env::set_var("CURRENT_DIRECTORY", env::current_dir().unwrap().to_str().unwrap());

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    log::info!("Server started at http://localhost:{}", port);

    HttpServer::new(move || {
        App::new()
            .service(Files::new("/static", "./static"))
            .configure(register_routes)
            .wrap(Logger::default())
            .wrap(setup_cors())
            .default_service(web::to(|| async {json_not_found_response(None)}))
    })
        .shutdown_timeout(1)
        .bind((host, port))?
        .run()
        .await
}
