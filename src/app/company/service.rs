use super::model::Company;
use actix_web::web::{self, ServiceConfig};

pub fn config(cfg: &mut ServiceConfig) {
  cfg.service(
    web::scope("/api/v1/companies")
      .route("", web::get().to(Company::list))
      .route("", web::post().to(Company::create))
      .route("/{id}", web::get().to(Company::detail))
      .route("/{id}", web::patch().to(Company::update))
      .route("/{id}", web::delete().to(Company::destroy)),
  );
}
