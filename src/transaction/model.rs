use crate::transaction::amount::Amount;
use crate::transaction::category::Category;
use crate::transaction::description::Description;
use chrono::NaiveDate;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Transaction {
    pub date: NaiveDate,
    pub amount: Amount,
    pub category: Category,
    pub description: Description,
}

impl Transaction {
    pub fn new(
        date: NaiveDate,
        amount: Amount,
        category: Category,
        description: Description,
    ) -> Self {
        Self {
            date,
            amount,
            category,
            description,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_transaction() {
        let date = NaiveDate::from_ymd_opt(2024, 3, 15).unwrap();
        let amount = Amount::new(1000).unwrap();
        let category = Category::new("Food".to_string()).unwrap();
        let description = Description::new("Grocery shopping".to_string()).unwrap();

        let transaction = Transaction::new(date, amount, category, description);

        assert_eq!(
            transaction.date,
            NaiveDate::from_ymd_opt(2024, 3, 15).unwrap()
        );
        assert_eq!(transaction.amount.value(), 1000);
        assert_eq!(transaction.category.value(), "Food");
        assert_eq!(transaction.description.value(), "Grocery shopping");
    }
}
