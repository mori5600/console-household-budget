#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Amount(i64);

impl Amount {
    pub fn new(value: i64) -> Result<Self, String> {
        if value < 0 {
            return Err("Amount cannot be negative".to_string());
        }

        Ok(Amount(value))
    }

    pub fn value(&self) -> i64 {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_amount() {
        let amount = Amount::new(10);
        assert!(amount.is_ok());
        assert_eq!(amount.unwrap().value(), 10);
    }

    #[test]
    fn test_create_negative_amount() {
        let amount = Amount::new(-5);
        assert!(amount.is_err());
        assert_eq!(amount.err().unwrap(), "Amount cannot be negative");
    }

    #[test]
    fn ordering() {
        let a = Amount::new(5).unwrap();
        let b = Amount::new(10).unwrap();
        assert!(a < b);
    }

    #[test]
    fn test_amount_value() {
        let amount = Amount::new(20).unwrap();
        assert_eq!(amount.value(), 20);
    }
}
