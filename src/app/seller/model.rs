use crate::schema::sellers;
use chrono::NaiveDateTime;
use diesel::{AsChangeset, Insertable};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Debug)]
pub struct Seller {
  pub id: String,
  pub name: String,
  pub address: Option<String>,
  pub email: String,
  pub password: String,
  pub phone: Option<String>,
  pub created_at: NaiveDateTime,
  pub updated_at: NaiveDateTime,
}

#[derive(Insertable, PartialEq, Deserialize, AsChangeset)]
#[table_name = "sellers"]
pub struct SellerForm {
  pub name: String,
  pub address: Option<String>,
  pub email: String,
  pub password: String,
  pub phone: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct TokenPayload {
  pub id: String,
  pub name: String,
  pub address: Option<String>,
  pub email: String,
  pub phone: Option<String>,
  pub created_at: NaiveDateTime,
  pub updated_at: NaiveDateTime,
}
