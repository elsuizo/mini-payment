use chrono::NaiveDate;
use chrono::Utc;
use rust_decimal::Decimal;
use std::cell::LazyCell;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::path::is_separator;
use thiserror::Error;
use unicode_segmentation::UnicodeSegmentation;
use uuid::Uuid;
// TODO(elsuizo: 2025-07-12): all this wrapper type should be encoded in a trait to avoid code
// repetition

//-------------------------------------------------------------------------
//                        errors
//-------------------------------------------------------------------------
#[derive(Error, Debug)]
pub enum CreateUserError {
    #[error("invalid name: {0}")]
    InvalidName(String),
    #[error("invalid county name: {0}")]
    InvalidCountryName(String),
    #[error("Invalid Document number: {0}")]
    InvalidDocumentNumber(usize),
    #[error("a user with this document number {0:?}, already exists!!!")]
    UserAlreadyExistsError(DocumentNumber),
}

#[derive(Debug, Clone, Eq, serde::Deserialize)]
pub struct User {
    client_name: UserName,
    bird_date: NaiveDate,
    document_number: DocumentNumber,
    country: CountryName,
    //client_id: Uuid,
    credit: Decimal,
}

impl Hash for User {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.document_number.hash(state);
    }
}

impl User {
    pub fn new(
        client_name: UserName,
        bird_date: NaiveDate,
        document_number: DocumentNumber,
        country: CountryName,
    ) -> Self {
        Self {
            client_name,
            bird_date,
            document_number,
            country,
            //client_id: Uuid::new_v4(),
            credit: 0.into(),
        }
    }

    // pub fn get_id(&self) -> Uuid {
    //     self.client_id
    // }

    pub fn get_document_number(&self) -> usize {
        self.document_number.0
    }
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.document_number == other.document_number
    }
}

#[derive(Debug, Clone, Hash, PartialEq, PartialOrd, Eq, serde::Deserialize)]
pub struct CountryName(String);

impl CountryName {
    // NOTE(elsuizo: 2025-07-12): this simulate the database of valid countrys
    const VALID_COUNTRY: [&str; 7] = [
        "Argentina",
        "Brazil",
        "Chile",
        "Ecuador",
        "Paraguay",
        "Uruguay",
        "Peru",
    ];

    pub fn inner(self) -> String {
        self.0
    }

    pub fn inner_ref(&self) -> &str {
        &self.0
    }

    pub fn parse_and_validate(s: &str) -> Result<Self, CreateUserError> {
        let is_empty_or_whitespace = s.trim().is_empty();

        let is_valid_country = Self::VALID_COUNTRY
            .iter()
            .any(|&country_name| country_name == s);

        if is_empty_or_whitespace || is_valid_country {
            Ok(Self(s.to_string()))
        } else {
            Err(CreateUserError::InvalidCountryName(s.to_string()))
        }
    }
}

impl AsRef<str> for CountryName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, PartialOrd, Eq, serde::Deserialize)]
pub struct DocumentNumber(usize);

impl DocumentNumber {
    const UPPER_LIMIT: usize = 100000000;

    pub fn inner(self) -> usize {
        self.0
    }

    // TODO(elsuizo: 2025-07-12): this should be for every country...
    pub fn parse_and_validate(raw_number: usize) -> Result<DocumentNumber, CreateUserError> {
        if raw_number > Self::UPPER_LIMIT {
            Err(CreateUserError::InvalidDocumentNumber(raw_number))
        } else {
            Ok(Self(raw_number))
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, PartialOrd, Eq, serde::Deserialize)]
pub struct UserName(String);

// TODO(elsuizo: 2025-07-12): sacar todos los comentarios en espaniol
impl UserName {
    /// name lenght upper limit threshold
    const UPPER_LIMIT: usize = 256;

    pub fn inner(self) -> String {
        self.0
    }

    pub fn inner_ref(&self) -> &str {
        &self.0
    }

    /// return a valid UserName or Error
    pub fn parse_and_validate(s: &str) -> Result<Self, CreateUserError> {
        let is_empty_or_whitespace = s.trim().is_empty();

        // un grafeno retorna un iterador sobre los grafenos en la entrada `s`, el parametro `true`
        // significa que queremos usar el conjunto de grafenos extendidos(que es el recomendado)
        let is_too_long = s.graphemes(true).count() > Self::UPPER_LIMIT;

        let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];

        let contains_forbiddden_characteres = s.chars().any(|g| forbidden_characters.contains(&g));

        if is_empty_or_whitespace || is_too_long || contains_forbiddden_characteres {
            Err(CreateUserError::InvalidName(s.to_string()))
        } else {
            Ok(Self(s.to_string()))
        }
    }
}

impl AsRef<str> for UserName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

//-------------------------------------------------------------------------
//                        unit tests
//-------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use crate::user::UserName;
    use claims::{assert_err, assert_ok};

    #[test]
    fn a_256_grapheme_long_name_is_valid() {
        let name = "a̐".repeat(256);
        assert_ok!(UserName::parse_and_validate(&name));
    }

    #[test]
    fn whitespace_only_names_are_rejected() {
        let name = "".to_string();
        assert_err!(UserName::parse_and_validate(&name));
    }

    #[test]
    fn a_name_longer_than_256_graphemes_is_rejected() {
        let name = "a".repeat(257);
        assert_err!(UserName::parse_and_validate(&name));
    }

    #[test]
    fn names_containing_an_invalid_character_are_rejected() {
        for name in &['/', '(', ')', '"', '<', '>', '\\', '{', '}'] {
            let name = name.to_string();
            assert_err!(UserName::parse_and_validate(&name));
        }
    }

    #[test]
    fn a_valid_name_is_parsed_successfully() {
        let name = "Martin Noblia".to_string();
        assert_ok!(UserName::parse_and_validate(&name));
    }
}
