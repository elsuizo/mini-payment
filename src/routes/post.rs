use crate::local_database::Database;
use crate::user::{CountryName, CreateUserError, DatabaseError, DocumentNumber, User, UserName};
use actix_web::HttpResponse;
use actix_web::Responder;
use actix_web::web;
use chrono::NaiveDate;
use log::info;
use rust_decimal::Decimal;
use std::error::Error;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

//-------------------------------------------------------------------------
//                        new user handler
//-------------------------------------------------------------------------
#[derive(serde::Deserialize, Debug, Clone)]
pub struct UserData {
    client_name: String,
    birth_date: String,
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
    let bird_date = NaiveDate::parse_from_str(&data.birth_date, "%Y-%m-%d")
        .expect("Error parsing the date, use the format: Y-m-d");
    let document_number = DocumentNumber::parse_and_validate(data.document_number)?;
    let country = CountryName::parse_and_validate(&data.country)?;

    let user = User::new(user_name, bird_date, document_number, country);

    // TODO(elsuizo: 2025-07-12): no se que hacer con ese unwrap
    let id = database.lock().unwrap().insert_new_user(&user)?;

    info!("database state: {:?}", database);

    Ok(web::Json(Out { client_id: id }))
}

//-------------------------------------------------------------------------
//                        /new_credit_transaction
//-------------------------------------------------------------------------
#[derive(serde::Deserialize, Debug, Clone)]
pub struct BalancePlusMinus {
    client_id: Uuid,
    credit_amount: Decimal,
}

#[derive(serde::Serialize, Debug, Clone)]
pub struct BalanceOut {
    actual_balance: Decimal,
}

pub async fn increase_balance(
    data: web::Json<BalancePlusMinus>,
    database: web::Data<Arc<Mutex<Database>>>,
) -> Result<web::Json<BalanceOut>, DatabaseError> {
    let new_balance = database
        .lock()
        .unwrap()
        .find_user_and_increase_balance(data.client_id, data.credit_amount)?;

    Ok(web::Json(BalanceOut {
        actual_balance: new_balance,
    }))
}

//-------------------------------------------------------------------------
//                        /new_debit_transaction
//-------------------------------------------------------------------------
pub async fn decrease_balance(
    data: web::Json<BalancePlusMinus>,
    database: web::Data<Arc<Mutex<Database>>>,
) -> Result<web::Json<BalanceOut>, DatabaseError> {
    let new_balance = database
        .lock()
        .unwrap()
        .find_user_and_decrease_balance(data.client_id, data.credit_amount)?;

    Ok(web::Json(BalanceOut {
        actual_balance: new_balance,
    }))
}

//-------------------------------------------------------------------------
//                        /store_balances
//-------------------------------------------------------------------------
pub async fn store_balances(
    database: web::Data<Arc<Mutex<Database>>>,
) -> Result<(), Box<dyn Error>> {
    info!("saving balances");
    database.lock().unwrap().store_balances()?;
    Ok(())
}

//-------------------------------------------------------------------------
//                        dummy handler
//-------------------------------------------------------------------------
/// debug dummy handler
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}
