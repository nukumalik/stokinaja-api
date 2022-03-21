use super::model::{Supplier, SupplierForm, TokenPayload};
use crate::{
  config::connection,
  schema::{
    suppliers,
    suppliers::{dsl, dsl::email},
  },
  structs::{AuthForm, JsonResult, Token},
};
use actix_web::{
  web::{Form, Json},
  HttpRequest, Responder,
};
use bcrypt::{hash, verify, DEFAULT_COST};
use diesel::{delete, insert_into, prelude::*, update};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde_json::json;

impl Supplier {
  pub async fn login(form: Form<AuthForm>) -> impl Responder {
    let supplier = dsl::suppliers
      .filter(email.eq(form.email.clone()))
      .first::<Self>(&connection());

    match supplier {
      Ok(res) => {
        let is_match = verify(form.password.clone(), res.password.as_str());

        match is_match {
          Ok(_value) => {
            let token_payload = TokenPayload {
              id: res.id.clone(),
              name: res.name.clone(),
              email: res.email.clone(),
              phone: res.phone.clone(),
              company_id: res.company_id.clone(),
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
            data: None::<()>,
            message: String::from("Password is invalid")
          })),
        }
      }
      Err(_error) => Json(json!(JsonResult {
        code: 400,
        data: None::<()>,
        message: String::from("Failed to login")
      })),
    }
  }

  pub async fn register(form: Form<SupplierForm>) -> impl Responder {
    let supplier = dsl::suppliers
      .filter(email.eq(form.email.clone()))
      .first::<Self>(&connection());

    match supplier {
      Ok(_value) => Json(json!(JsonResult {
        code: 401,
        data: None::<()>,
        message: String::from("Email was registered")
      })),
      Err(_err) => {
        let new_supplier = SupplierForm {
          name: form.name.clone(),
          email: form.email.clone(),
          password: hash(form.password.clone(), DEFAULT_COST).unwrap(),
          phone: if form.phone.is_some() {
            Some(form.phone.clone().unwrap())
          } else {
            None
          },
          company_id: form.company_id.clone(),
        };

        let registered = insert_into(suppliers::table).values(new_supplier).execute(&connection());

        match registered {
          Ok(res) => Json(json!(JsonResult {
            code: 200,
            data: Some(res),
            message: String::from("Success to register new supplier")
          })),
          Err(error) => Json(json!(JsonResult {
            code: 401,
            data: None::<()>,
            message: format!("{}", error)
          })),
        }
      }
    }
  }

  pub async fn list() -> impl Responder {
    let suppliers = dsl::suppliers.load::<Self>(&connection());

    match suppliers {
      Ok(res) => Json(json!(JsonResult {
        code: 200,
        data: Some(res),
        message: String::from("Success to get supplier list")
      })),
      Err(error) => Json(json!(JsonResult {
        code: 400,
        data: None::<()>,
        message: format!("{}", error)
      })),
    }
  }

  pub async fn detail(req: HttpRequest) -> impl Responder {
    let id = req.match_info().get("id").unwrap();
    let supplier = dsl::suppliers.find(id).first::<Self>(&connection()).optional().unwrap();

    match supplier {
      Some(res) => Json(json!(JsonResult {
        code: 200,
        data: Some(res),
        message: String::from("Success to get supplier detail")
      })),
      None => Json(json!(JsonResult {
        code: 400,
        data: None::<()>,
        message: String::from("Failed to get supplier detail")
      })),
    }
  }

  pub async fn update(req: HttpRequest, form: Form<SupplierForm>) -> impl Responder {
    let id = req.match_info().get("id").unwrap();
    let data = SupplierForm {
      email: form.email.clone(),
      name: form.name.clone(),
      password: form.password.clone(),
      company_id: form.company_id.clone(),
      phone: form.phone.clone(),
    };

    let updated = update(dsl::suppliers.find(id)).set(data).execute(&connection());

    match updated {
      Ok(res) => Json(json!(JsonResult {
        code: 200,
        data: Some(res),
        message: String::from("Success to update supplier")
      })),
      Err(error) => Json(json!(JsonResult {
        code: 400,
        data: None::<()>,
        message: format!("{}", error)
      })),
    }
  }

  pub async fn destroy(req: HttpRequest) -> impl Responder {
    let id = req.match_info().get("id").unwrap();
    let supplier = dsl::suppliers
      .find(id)
      .first::<Self>(&connection())
      .optional()
      .expect("Failed to get supplier");

    match supplier {
      Some(_res) => {
        delete(dsl::suppliers.find(id)).execute(&connection()).unwrap();
        return Json(json!(JsonResult {
          code: 200,
          data: None::<()>,
          message: String::from("Success to delete supplier")
        }));
      }
      None => Json(json!(JsonResult {
        code: 400,
        data: None::<()>,
        message: String::from("Supplier not found")
      })),
    }
  }
}
