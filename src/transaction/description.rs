#[derive(Debug, thiserror::Error)]
pub enum DescriptionError {
    #[error("Description cannot be empty")]
    EmptyDescription,
    #[error("Description is too long: {0} characters (maximum: {1})")]
    TooLong(usize, usize),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Description(String);

impl Description {
    pub fn new(value: String) -> Result<Self, DescriptionError> {
        if value.trim().is_empty() {
            return Err(DescriptionError::EmptyDescription);
        }

        const MAX_LENGTH: usize = 200;
        if value.len() > MAX_LENGTH {
            return Err(DescriptionError::TooLong(value.len(), MAX_LENGTH));
        }

        Ok(Self(value))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_description() {
        let description = Description::new("Grocery shopping at Walmart".to_string());
        assert!(description.is_ok());
        assert_eq!(description.unwrap().value(), "Grocery shopping at Walmart");
    }

    #[test]
    fn test_create_empty_description() {
        let description = Description::new("".to_string());
        assert!(description.is_err());
        assert!(matches!(
            description,
            Err(DescriptionError::EmptyDescription)
        ));
    }

    #[test]
    fn test_create_whitespace_description() {
        let description = Description::new("   ".to_string());
        assert!(description.is_err());
        assert!(matches!(
            description,
            Err(DescriptionError::EmptyDescription)
        ));
    }

    #[test]
    fn test_create_too_long_description() {
        let long_desc = "a".repeat(201);
        let description = Description::new(long_desc);
        assert!(description.is_err());
        assert!(matches!(
            description,
            Err(DescriptionError::TooLong(201, 200))
        ));
    }
}
