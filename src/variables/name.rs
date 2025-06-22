//! This module defines the name of a variable.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Name(String);

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum NameError {
    #[error("The name cannot be empty")]
    Empty,
    #[error("The name must have only teh following allowed characters: alphanumeric, underscore")]
    InvalidCharacters,
}

impl Name {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl PartialEq<&str> for &Name {
    fn eq(&self, other: &&str) -> bool {
        self.0 == *other
    }
}

impl PartialEq<&str> for Name {
    fn eq(&self, other: &&str) -> bool {
        self == *other
    }
}

impl PartialEq<String> for Name {
    fn eq(&self, other: &String) -> bool {
        self == other.as_str()
    }
}

impl TryFrom<&str> for Name {
    type Error = NameError;

    fn try_from(name: &str) -> Result<Self, Self::Error> {
        if name.is_empty() {
            return Err(NameError::Empty);
        }

        let all_valid_chars = name.chars().all(|c| c.is_alphanumeric() || c == '_');
        if !all_valid_chars {
            return Err(NameError::InvalidCharacters);
        }

        Ok(Name(String::from(name)))
    }
}

impl TryFrom<String> for Name {
    type Error = NameError;

    fn try_from(name: String) -> Result<Self, Self::Error> {
        Name::try_from(name.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    fn test_try_from_valid_name_str() {
        let name = Name::try_from("x1_valid").unwrap();
        assert_eq!(name, "x1_valid");
        assert_eq!(name.as_str(), "x1_valid");
    }

    #[test]
    fn test_try_from_valid_name_string() {
        let name = Name::try_from("x".to_string()).unwrap();
        assert_eq!(name, "x");
    }

    #[test]
    fn test_try_from_empty_string_failure() {
        let err = Name::try_from("").unwrap_err();
        assert_eq!(err, NameError::Empty);
    }

    #[test]
    fn test_try_from_invalid_string_failure() {
        let err = Name::try_from("a?").unwrap_err();
        assert_eq!(err, NameError::InvalidCharacters);
    }

    #[test]
    fn test_ordering() {
        let name1 = Name::try_from("a").unwrap();
        let name2 = Name::try_from("b").unwrap();

        assert_eq!(name1, name1);
        assert_eq!(name2, name2);
        assert_ne!(name1, name2);

        assert!(name1 < name2);
        assert!(name2 > name1);
        assert!(name2 >= name1);

        assert!(name1 >= name1);
        assert!(name1 <= name1);

        assert!(name2 >= name2);
        assert!(name2 <= name2);
    }
}
