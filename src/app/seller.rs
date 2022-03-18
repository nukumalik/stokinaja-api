use crate::{
  config::connection,
  schema::{
    sellers,
    sellers::{dsl, dsl::email},
  },
  structs::{AuthForm, JsonResult, Token},
};
use actix_web::{
  web,
  web::{Form, Json, ServiceConfig},
  HttpRequest, Responder,
};
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::NaiveDateTime;
use diesel::{delete, insert_into, prelude::*, update, AsChangeset, Insertable};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use serde_json::json;

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

impl Seller {
  async fn login(form: Form<AuthForm>) -> impl Responder {
    let seller = dsl::sellers
      .filter(email.eq(form.email.clone()))
      .first::<Self>(&connection());

    match seller {
      Ok(res) => {
        let is_match = verify(form.password.clone(), res.password.as_str());

        match is_match {
          Ok(_value) => {
            let token_payload = TokenPayload {
              id: res.id.clone(),
              address: res.address.clone(),
              name: res.name.clone(),
              email: res.email.clone(),
              phone: res.phone.clone(),
              created_at: res.created_at.clone(),
              updated_at: res.updated_at.clone(),
            };

            let token = encode(
              &Header::default(),
              &token_payload,
              &EncodingKey::from_secret("secret".as_ref()),
            )
            .unwrap();

            Json(json!(JsonResult {
              code: 200,
              data: Some(json!(Token { token })),
              message: String::from("Success to login")
            }))
          }
          Err(_error) => Json(json!(JsonResult {
            code: 400,
            data: None::<i32>,
            message: String::from("Password is invalid")
          })),
        }
      }
      Err(_error) => Json(json!(JsonResult {
        code: 400,
        data: None::<i32>,
        message: String::from("Failed to login")
      })),
    }
  }

  async fn register(form: Form<SellerForm>) -> impl Responder {
    let seller = dsl::sellers
      .filter(email.eq(form.email.clone()))
      .first::<Self>(&connection());

    match seller {
      Ok(_value) => Json(json!(JsonResult {
        code: 401,
        data: None::<i32>,
        message: String::from("Email was registered")
      })),
      Err(_err) => {
        let new_seller = SellerForm {
          name: form.name.clone(),
          address: if form.address.is_some() {
            Some(form.address.clone().unwrap())
          } else {
            None
          },
          email: form.email.clone(),
          password: hash(form.password.clone(), DEFAULT_COST).unwrap(),
          phone: if form.phone.is_some() {
            Some(form.phone.clone().unwrap())
          } else {
            None
          },
        };

        let registered = insert_into(sellers::table)
          .values(new_seller)
          .execute(&connection());

        match registered {
          Ok(res) => Json(json!(JsonResult {
            code: 200,
            data: Some(res),
            message: String::from("Success to register new seller")
          })),
          Err(error) => Json(json!(JsonResult {
            code: 401,
            data: None::<i32>,
            message: format!("{}", error)
          })),
        }
      }
    }
  }

  async fn list() -> impl Responder {
    let sellers = dsl::sellers.load::<Self>(&connection());

    match sellers {
      Ok(res) => Json(json!(JsonResult {
        code: 200,
        data: Some(res),
        message: String::from("Success to get seller list")
      })),
      Err(error) => Json(json!(JsonResult {
        code: 400,
        data: None::<i32>,
        message: format!("{}", error)
      })),
    }
  }

  async fn detail(req: HttpRequest) -> impl Responder {
    let id = req.match_info().get("id").unwrap();
    let seller = dsl::sellers
      .find(id)
      .first::<Self>(&connection())
      .optional()
      .unwrap();

    match seller {
      Some(res) => Json(json!(JsonResult {
        code: 200,
        data: Some(res),
        message: String::from("Success to get seller detail")
      })),
      None => Json(json!(JsonResult {
        code: 400,
        data: None::<i32>,
        message: String::from("Failed to get seller detail")
      })),
    }
  }

  async fn update(req: HttpRequest, form: Form<SellerForm>) -> impl Responder {
    let id = req.match_info().get("id").unwrap();
    let data = SellerForm {
      address: form.address.clone(),
      email: form.email.clone(),
      name: form.name.clone(),
      password: form.password.clone(),
      phone: form.phone.clone(),
    };

    let updated = update(dsl::sellers.find(id))
      .set(data)
      .execute(&connection());

    match updated {
      Ok(res) => Json(json!(JsonResult {
        code: 200,
        data: Some(res),
        message: String::from("Success to update seller")
      })),
      Err(error) => Json(json!(JsonResult {
        code: 400,
        data: None::<i32>,
        message: format!("{}", error)
      })),
    }
  }

  async fn destroy(req: HttpRequest) -> impl Responder {
    let id = req.match_info().get("id").unwrap();
    let seller = dsl::sellers
      .find(id)
      .first::<Self>(&connection())
      .optional()
      .expect("Failed to get seller");

    let is_exists = match seller {
      Some(res) => Ok(res),
      None => Err("Seller not found"),
    };

    match is_exists {
      Ok(_res) => {
        delete(dsl::sellers.find(id))
          .execute(&connection())
          .unwrap();
        return Json(json!(JsonResult {
          code: 200,
          data: None::<i32>,
          message: String::from("Success to delete seller")
        }));
      }
      Err(error) => Json(json!(JsonResult {
        code: 400,
        data: None::<i32>,
        message: format!("{}", error)
      })),
    }
  }
}

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
