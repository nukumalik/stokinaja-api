use diesel::{mysql::MysqlConnection, Connection};
use dotenv::dotenv;
use std::env;

pub fn connection() -> MysqlConnection {
  dotenv().ok();

  let database_url = env::var("DATABASE_URL").expect("DATABASE_URL undefined");

  MysqlConnection::establish(&database_url).expect("Database connection failed")
}
