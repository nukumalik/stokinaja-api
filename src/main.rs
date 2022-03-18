#[macro_use]
extern crate diesel;
extern crate dotenv;

mod app;
mod config;
mod schema;
mod structs;
mod traits;

use actix_web::*;

#[main]
async fn main() -> std::io::Result<()> {
  println!("Server running on port 3000");

  HttpServer::new(|| App::new().configure(app::seller::config))
    .bind("localhost:3000")?
    .run()
    .await
}
