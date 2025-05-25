#[derive(Debug, thiserror::Error)]
pub enum CategoryError {
    #[error("Category cannot be empty")]
    EmptyCategory,
    #[error("Category name is too long: {0} characters (maximum: {1})")]
    TooLong(usize, usize),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Category(String);

impl Category {
    pub fn new(value: String) -> Result<Self, CategoryError> {
        if value.trim().is_empty() {
            return Err(CategoryError::EmptyCategory);
        }

        // オプション: 最大長の制限を追加
        const MAX_LENGTH: usize = 50;
        if value.len() > MAX_LENGTH {
            return Err(CategoryError::TooLong(value.len(), MAX_LENGTH));
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
    fn test_create_category() {
        let category = Category::new("Food".to_string());
        assert!(category.is_ok());
        assert_eq!(category.unwrap().value(), "Food");
    }

    #[test]
    fn test_create_empty_category() {
        let category = Category::new("".to_string());
        assert!(category.is_err());
        assert!(matches!(category, Err(CategoryError::EmptyCategory)));
    }

    #[test]
    fn test_create_whitespace_category() {
        let category = Category::new("   ".to_string());
        assert!(category.is_err());
        assert!(matches!(category, Err(CategoryError::EmptyCategory)));
    }

    #[test]
    fn test_create_too_long_category() {
        let long_name = "a".repeat(51);
        let category = Category::new(long_name);
        assert!(category.is_err());
        assert!(matches!(category, Err(CategoryError::TooLong(51, 50))));
    }
}
