use super::model::{Company, CompanyForm};
use crate::{
  config::connection,
  diesel::{delete, insert_into, result::Error, update, OptionalExtension, QueryDsl, RunQueryDsl},
  schema::companies::{self, dsl},
  structs::JsonResult,
};
use actix_web::{
  web::{Form, Json},
  HttpRequest, Responder,
};
use serde_json::json;

impl Company {
  pub async fn list() -> impl Responder {
    let companies = dsl::companies.load::<Self>(&connection());

    match companies {
      Ok(res) => Json(json!(JsonResult {
        code: 200,
        data: Some(res),
        message: String::from("Success to get company list")
      })),
      Err(_error) => Json(json!(JsonResult {
        code: 400,
        data: None::<()>,
        message: String::from("Failed to get company list")
      })),
    }
  }

  pub async fn detail(req: HttpRequest) -> impl Responder {
    let id = req.match_info().get("id").unwrap();
    let company = dsl::companies.find(&id).first::<Self>(&connection()).optional();

    match company {
      Ok(res) => Json(json!(JsonResult {
        code: 200,
        data: res,
        message: String::from("Success to get company detail")
      })),
      Err(Error::NotFound) => Json(json!(JsonResult {
        code: 404,
        data: None::<()>,
        message: String::from("Company not found")
      })),
      Err(_) => Json(json!(JsonResult {
        code: 400,
        data: None::<()>,
        message: String::from("Failed to get company detail")
      })),
    }
  }

  pub async fn create(form: Form<CompanyForm>) -> impl Responder {
    let data = CompanyForm {
      address: form.address.clone(),
      description: form.description.clone(),
      name: form.name.clone(),
      email: form.email.clone(),
      phone: form.phone.clone(),
    };

    let created = insert_into(companies::table).values(data).execute(&connection());

    match created {
      Ok(res) => Json(json!(JsonResult {
        code: 200,
        data: Some(res),
        message: String::from("Success to create company")
      })),
      Err(_) => Json(json!(JsonResult {
        code: 400,
        data: None::<()>,
        message: String::from("Failed to create company")
      })),
    }
  }

  pub async fn update(req: HttpRequest, form: Form<CompanyForm>) -> impl Responder {
    let id = req.match_info().get("id").unwrap();
    let company: Result<Option<Self>, _> = dsl::companies.find(&id).first::<Self>(&connection()).optional();

    match company {
      Ok(res) => {
        let tmp = res.unwrap();
        let data = CompanyForm {
          address: if form.address.is_some() {
            form.address.clone()
          } else {
            tmp.address.clone()
          },
          description: if form.description.is_some() {
            form.description.clone()
          } else {
            tmp.description.clone()
          },
          email: if form.email.is_empty() {
            tmp.email.clone()
          } else {
            form.email.clone()
          },
          name: if form.name.is_empty() {
            tmp.name.clone()
          } else {
            form.name.clone()
          },
          phone: if form.phone.is_empty() {
            tmp.phone.clone()
          } else {
            form.phone.clone()
          },
        };

        let updated: Result<_, _> = update(dsl::companies.find(id)).set(data).execute(&connection());

        match updated {
          Ok(_) => Json(json!(JsonResult {
            code: 200,
            data: None::<()>,
            message: String::from("Success to update company")
          })),
          Err(_) => Json(json!(JsonResult {
            code: 400,
            data: None::<()>,
            message: String::from("Failed to update company")
          })),
        }
      }
      Err(Error::NotFound) => {
        return Json(json!(JsonResult {
          code: 404,
          data: None::<()>,
          message: String::from("Company not found")
        }));
      }
      Err(_) => {
        return Json(json!(JsonResult {
          code: 400,
          data: None::<()>,
          message: String::from("Failed to get company detail")
        }));
      }
    }
  }

  pub async fn destroy(req: HttpRequest) -> impl Responder {
    let id = req.match_info().get("id").unwrap();
    let company = dsl::companies
      .find(id)
      .first::<Self>(&connection())
      .optional()
      .expect("Failed to get company");

    let is_exists = match company {
      Some(res) => Ok(res),
      None => Err("Company not found"),
    };

    match is_exists {
      Ok(_res) => {
        delete(dsl::companies.find(id)).execute(&connection()).unwrap();
        return Json(json!(JsonResult {
          code: 200,
          data: None::<()>,
          message: String::from("Success to delete company")
        }));
      }
      Err(error) => Json(json!(JsonResult {
        code: 400,
        data: None::<()>,
        message: format!("{}", error)
      })),
    }
  }
}
