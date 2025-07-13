use crate::local_database::Database;
use crate::user::CountryName;
use crate::user::CreateUserError;
use crate::user::DocumentNumber;
use crate::user::User;
use crate::user::UserName;
use actix_web::HttpResponse;
use actix_web::Responder;
use actix_web::middleware::Logger;
use actix_web::web;
use chrono::NaiveDate;
use log::info;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

//-------------------------------------------------------------------------
//                        new user handler
//-------------------------------------------------------------------------
#[derive(serde::Deserialize, Debug, Clone)]
pub struct UserData {
    client_name: String,
    bird_date: String,
    document_number: usize,
    country: String,
}

#[derive(serde::Serialize, Debug, Clone)]
pub struct Out {
    client_id: Uuid,
}

pub async fn client_creation(
    data: web::Json<UserData>,
    database: web::Data<Arc<Mutex<Database>>>,
) -> Result<web::Json<Out>, CreateUserError> {
    let user_name = UserName::parse_and_validate(&data.client_name)?;
    // TODO(elsuizo: 2025-07-13): better error for parsing `bird_date`
    let bird_date =
        NaiveDate::parse_from_str(&data.bird_date, "%Y-%m-%d").expect("Error parsing the date");
    let document_number = DocumentNumber::parse_and_validate(data.document_number)?;
    let country = CountryName::parse_and_validate(&data.country)?;

    let user = User::new(user_name, bird_date, document_number, country);

    // TODO(elsuizo: 2025-07-12): no se que hacer con ese unwrap
    let id = database.lock().unwrap().insert_new_user(&user)?;

    info!("database state: {:?}", database);

    Ok(web::Json(Out { client_id: id }))
}

/// debug dummy handler
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}
