use super::model::Supplier;
use actix_web::{web, web::ServiceConfig};

pub fn config(cfg: &mut ServiceConfig) -> () {
  cfg.service(
    web::scope("/api/v1/suppliers")
      .route("/login", web::post().to(Supplier::login))
      .route("/register", web::post().to(Supplier::register))
      .route("", web::get().to(Supplier::list))
      .route("/{id}", web::get().to(Supplier::detail))
      .route("/{id}", web::patch().to(Supplier::update))
      .route("/{id}", web::delete().to(Supplier::destroy)),
  );
}
