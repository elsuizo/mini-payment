use crate::user::{CreateUserError, DatabaseError, User};
use chrono::{DateTime, Datelike, Local};
use rust_decimal::Decimal;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use uuid::Uuid;

#[derive(Debug, serde::Deserialize)]
pub struct Database {
    users: HashMap<Uuid, User>,
    files_generate: usize,
}

impl Database {
    pub fn new() -> Self {
        Self {
            users: HashMap::new(),
            files_generate: 0,
        }
    }
    // TODO(elsuizo: 2025-07-12): get rid of this clone
    pub fn insert_new_user(&mut self, new_user: &User) -> Result<Uuid, CreateUserError> {
        if self.users.values().any(|user| user == new_user) {
            Err(CreateUserError::InvalidDocumentNumber(
                new_user.get_document_number(),
            ))
        } else {
            let id = Uuid::new_v4();
            self.users.insert(id, new_user.clone());
            Ok(id)
        }
    }

    pub fn find_user_and_increase_balance(
        &mut self,
        id: Uuid,
        amount: Decimal,
    ) -> Result<Decimal, DatabaseError> {
        if let Some(user) = self.users.get_mut(&id) {
            user.increase_credit(amount);
            Ok(user.get_actual_credit())
        } else {
            Err(DatabaseError::UnknownUser(id))
        }
    }

    pub fn find_user_and_decrease_balance(
        &mut self,
        id: Uuid,
        amount: Decimal,
    ) -> Result<Decimal, DatabaseError> {
        if let Some(user) = self.users.get_mut(&id) {
            user.decrease_credit(amount)?;
            Ok(user.get_actual_credit())
        } else {
            Err(DatabaseError::UnknownUser(id))
        }
    }

    pub fn get_user(&self, id: Uuid) -> Result<User, DatabaseError> {
        if let Some(user) = self.users.get(&id) {
            // TODO(elsuizo: 2025-07-13): no clone pleaseee...
            Ok(user.clone())
        } else {
            Err(DatabaseError::UnknownUser(id))
        }
    }

    pub fn store_balances(&mut self) -> Result<(), Box<dyn Error>> {
        self.files_generate += 1;
        let local: DateTime<Local> = Local::now();
        let year = local.year();
        let month = local.month();
        let day = local.day();
        let mut content = String::new();
        let mut file = File::create(format!(
            "{}{}{}_{}.DAT",
            day, month, year, self.files_generate
        ))?;

        for (k, v) in self.users.iter_mut() {
            content += &format!("{} {}\n", k, v.get_actual_credit()).to_string();
            v.reset_credit();
        }
        file.write_all(content.as_bytes())?;
        Ok(())
    }
}

//-------------------------------------------------------------------------
//                        unit tests
//-------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use crate::local_database::Database;
    use crate::user::CountryName;
    use crate::user::DocumentNumber;
    use crate::user::User;
    use crate::user::UserName;
    use chrono::NaiveDate;
    use claims::{assert_err, assert_ok};

    #[test]
    fn insert_new_user() {
        let name1 = UserName::parse_and_validate("Martin Noblia").expect("error parsing name");
        let date1 = NaiveDate::parse_from_str("1982-9-27", "%Y-%m-%d").expect("error parsing date");
        let doc1 = DocumentNumber::parse_and_validate(29653164).expect("error parsing doc number");
        let country1 = CountryName::parse_and_validate("Argentina").expect("error parsing country");
        let user1 = User::new(name1, date1, doc1, country1);

        let name2 = UserName::parse_and_validate("Juan Perez").expect("error parsing name");
        let date2 = NaiveDate::parse_from_str("1982-9-27", "%Y-%m-%d").expect("error parsing date");
        let doc2 = DocumentNumber::parse_and_validate(29653164).expect("error parsing doc number");
        let country2 = CountryName::parse_and_validate("Chile").expect("error parsing country");
        let user2 = User::new(name2, date2, doc2, country2);

        let mut db = Database::new();

        let result1 = db.insert_new_user(&user1);
        let result2 = db.insert_new_user(&user2);

        println!("{result1:?}");
        println!("{result2:?}");

        assert!(result1.is_ok());
        assert!(result2.is_err());
    }
}
