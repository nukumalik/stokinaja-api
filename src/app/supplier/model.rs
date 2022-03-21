use crate::schema::suppliers;
use chrono::NaiveDateTime;
use diesel::{AsChangeset, Insertable};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Debug)]
pub struct Supplier {
  pub id: String,
  pub name: String,
  pub email: String,
  pub password: String,
  pub phone: Option<String>,
  pub company_id: String,
  pub created_at: NaiveDateTime,
  pub updated_at: NaiveDateTime,
}

#[derive(Insertable, PartialEq, Deserialize, AsChangeset)]
#[table_name = "suppliers"]
pub struct SupplierForm {
  pub name: String,
  pub email: String,
  pub password: String,
  pub phone: Option<String>,
  pub company_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct TokenPayload {
  pub id: String,
  pub name: String,
  pub email: String,
  pub phone: Option<String>,
  pub company_id: String,
  pub created_at: NaiveDateTime,
  pub updated_at: NaiveDateTime,
}
