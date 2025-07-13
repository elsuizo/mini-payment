use crate::user;
use crate::user::DocumentNumber;
use crate::user::{CreateUserError, User};
use std::collections::HashMap;
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, serde::Deserialize)]
pub struct Database {
    users: HashMap<User, Uuid>,
}

impl Database {
    pub fn new() -> Self {
        Self {
            users: HashMap::new(),
        }
    }
    // TODO(elsuizo: 2025-07-12): get rid of this clone
    pub fn insert_new_user(&mut self, user: &User) -> Result<Uuid, CreateUserError> {
        if self.users.contains_key(user) {
            Err(CreateUserError::InvalidDocumentNumber(
                user.get_document_number(),
            ))
        } else {
            let id = Uuid::new_v4();
            self.users.insert(user.clone(), id);
            Ok(id)
        }
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
