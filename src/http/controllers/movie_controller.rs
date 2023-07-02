use actix_web::{get, HttpRequest, HttpResponse};
use actix_web::web::{Data, Path, ServiceConfig};
use base64::{engine, Engine};

use crate::core::directory_listing::DirectoryListing;
use crate::core::helpers::ffmpeg::generate_thumbnail;
use crate::core::helpers::responder::{json_error_message, json_success};

pub(crate) fn movie_controller(cfg: &mut ServiceConfig) {
    cfg.service(listing);
    cfg.service(watch);
    cfg.service(listing_thumbnail);
}

#[get("listing")]
async fn listing(listing: Data<DirectoryListing>) -> HttpResponse {
    json_success(listing.clone())
}

#[get("listing/{path}/thumbnail")]
async fn listing_thumbnail(req: HttpRequest, path: Path<String>) -> HttpResponse {
    let item_path_result = engine::general_purpose::URL_SAFE.decode(path.into_inner());
    if item_path_result.is_err() {
        return json_error_message("invalid path");
    }

    let item_path = String::from_utf8(item_path_result.unwrap()).unwrap();
    let thumbnail = generate_thumbnail(item_path).unwrap();

    let file = actix_files::NamedFile::open_async(thumbnail).await.unwrap();
    file.into_response(&req)
}

#[get("{path}/watch")]
async fn watch(req: HttpRequest, path: Path<String>) -> HttpResponse {
    let item_path_result = engine::general_purpose::URL_SAFE.decode(path.into_inner());
    if item_path_result.is_err() {
        return json_error_message("invalid path");
    }

    let item_path = String::from_utf8(item_path_result.unwrap()).unwrap();
    let file = actix_files::NamedFile::open_async(item_path).await.unwrap();
    file.into_response(&req)
}
