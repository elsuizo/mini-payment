use crate::local_database::Database;
use crate::user::{DatabaseError, User, UserName};
use actix_web::web;
use rust_decimal::Decimal;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct UserIn {
    client_id: Uuid,
}

#[derive(serde::Serialize, Debug, Clone)]
pub struct Out {
    client_id: Uuid,
    balance: Decimal,
    client_name: UserName,
}

/// extract form data using serde
/// this handler gets called only if the content type is *x-www-form-urlencoded*
pub async fn get_balance(
    data: web::Form<UserIn>,
    database: web::Data<Arc<Mutex<Database>>>,
) -> Result<web::Json<Out>, DatabaseError> {
    let user = database.lock().unwrap().get_user(data.client_id)?;
    Ok(web::Json(Out {
        client_id: data.client_id,
        balance: user.get_actual_credit(),
        client_name: user.client_name,
    }))
}
