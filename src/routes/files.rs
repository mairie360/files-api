use actix_web::web;
use crate::handlers::files::{list_files, get_file_by_id, get_file_by_name, create_file, update_file, delete_file};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/files")
            .route("", web::get().to(list_files))
            .route("/{id}", web::get().to(get_file_by_id))
            .route("/name/{name}", web::get().to(get_file_by_name))
            .route("", web::post().to(create_file))
            .route("/{id}", web::put().to(update_file))
            .route("/{id}", web::delete().to(delete_file))
    );
}