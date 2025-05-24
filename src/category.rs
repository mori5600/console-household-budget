#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Category(String);

impl Category {
    pub fn new(value: String) -> Result<Self, String> {
        if value.trim().is_empty() {
            return Err("Category cannot be empty".to_string());
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
        assert_eq!(category.err().unwrap(), "Category cannot be empty");
    }

    #[test]
    fn test_create_whitespace_category() {
        let category = Category::new("   ".to_string());
        assert!(category.is_err());
        assert_eq!(category.err().unwrap(), "Category cannot be empty");
    }
}
