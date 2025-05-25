#[derive(Debug, thiserror::Error)]
pub enum AmountError {
    #[error("Amount cannot be negative")]
    NegativeAmount(i64),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Amount(i64);

impl Amount {
    pub fn new(value: i64) -> Result<Self, AmountError> {
        if value < 0 {
            return Err(AmountError::NegativeAmount(value));
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
        assert!(matches!(amount, Err(AmountError::NegativeAmount(_))));
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
