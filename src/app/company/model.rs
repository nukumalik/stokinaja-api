use crate::schema::companies;
use chrono::NaiveDateTime;
use diesel::{AsChangeset, Insertable};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Debug)]
pub struct Company {
  pub id: String,
  pub name: String,
  pub address: Option<String>,
  pub description: Option<String>,
  pub email: String,
  pub phone: String,
  pub created_at: NaiveDateTime,
  pub updated_at: NaiveDateTime,
}

#[derive(Insertable, PartialEq, Deserialize, AsChangeset)]
#[table_name = "companies"]
pub struct CompanyForm {
  pub name: String,
  pub address: Option<String>,
  pub description: Option<String>,
  pub email: String,
  pub phone: String,
}
