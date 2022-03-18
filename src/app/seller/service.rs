use super::model::Seller;
use actix_web::{web, web::ServiceConfig};

pub fn config(cfg: &mut ServiceConfig) -> () {
  cfg.service(
    web::scope("/api/v1/sellers")
      .route("/login", web::post().to(Seller::login))
      .route("/register", web::post().to(Seller::register))
      .route("", web::get().to(Seller::list))
      .route("/{id}", web::get().to(Seller::detail))
      .route("/{id}", web::patch().to(Seller::update))
      .route("/{id}", web::delete().to(Seller::destroy)),
  );
}
