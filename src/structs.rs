use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct AuthForm {
  pub email: String,
  pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct Token {
  pub token: String,
}

#[derive(Serialize, Deserialize)]
pub struct JsonResult<T> {
  pub code: i32,
  pub data: Option<T>,
  pub message: String,
}
