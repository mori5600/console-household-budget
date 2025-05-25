use crate::transaction::amount::Amount;
use crate::transaction::category::Category;
use crate::transaction::description::Description;
use chrono::NaiveDate;

#[derive(Debug, thiserror::Error)]
pub enum TransactionError {
    #[error("Invalid ID")]
    InvalidId,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Transaction {
    pub id: String,
    pub date: NaiveDate,
    pub amount: Amount,
    pub category: Category,
    pub description: Description,
}

impl Transaction {
    pub fn new(
        id: String,
        date: NaiveDate,
        amount: Amount,
        category: Category,
        description: Description,
    ) -> Result<Self, TransactionError> {
        if id.is_empty() {
            return Err(TransactionError::InvalidId);
        }

        Ok(Self {
            id,
            date,
            amount,
            category,
            description,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_transaction() {
        let id = "test-123".to_string();
        let date = NaiveDate::from_ymd_opt(2024, 3, 15).unwrap();
        let amount = Amount::new(1000).unwrap();
        let category = Category::new("Food".to_string()).unwrap();
        let description = Description::new("Grocery shopping".to_string()).unwrap();

        let transaction = Transaction::new(id, date, amount, category, description).unwrap();

        assert_eq!(
            transaction.date,
            NaiveDate::from_ymd_opt(2024, 3, 15).unwrap()
        );
        assert_eq!(transaction.amount.value(), 1000);
        assert_eq!(transaction.category.value(), "Food");
        assert_eq!(transaction.description.value(), "Grocery shopping");
    }
}
