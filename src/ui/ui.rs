use crate::transaction::{
    amount::Amount, category::Category, description::Description, model::Transaction,
};
use std::io;

#[derive(Debug, thiserror::Error)]
enum TransactionError {
    #[error("Transaction not found")]
    NotFound,
}

#[derive(Debug, Clone)]
struct TransactionCollection {
    transactions: Vec<Transaction>,
}

impl TransactionCollection {
    fn new() -> Self {
        Self {
            transactions: Vec::new(),
        }
    }

    fn add_transaction(&mut self, transaction: Transaction) {
        self.transactions.push(transaction);
    }

    fn get_all_transactions(&self) -> &Vec<Transaction> {
        &self.transactions
    }

    fn delete_transaction(&mut self, id: &str) -> Result<(), TransactionError> {
        let index = self.transactions.iter().position(|t| t.id == id);
        if let Some(index) = index {
            self.transactions.remove(index);
            Ok(())
        } else {
            Err(TransactionError::NotFound)
        }
    }
}

fn get_user_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn add_transaction(transactions: &mut TransactionCollection) {
    let s_amount = get_user_input("Enter the amount: ");
    let s_category = get_user_input("Enter the category: ");
    let s_date = get_user_input("Enter the date: ");
    let s_description = get_user_input("Enter the description: ");

    let i_amount = s_amount.parse::<i64>().unwrap();

    let o_amount = Amount::new(i_amount).unwrap();
    let o_category = Category::new(s_category).unwrap();
    let c_date = chrono::NaiveDate::parse_from_str(&s_date, "%Y-%m-%d").unwrap();
    let o_description = Description::new(s_description).unwrap();
    let s_id = "1".to_string();

    let transaction = Transaction::new(s_id, c_date, o_amount, o_category, o_description).unwrap();
    transactions.add_transaction(transaction.clone());

    println!("Transaction added: {:?}", transaction);
}

fn view_transactions(transactions: &TransactionCollection) {
    println!("Viewing transactions...");
    for transaction in transactions.get_all_transactions() {
        println!("Transaction: {:?}", transaction);
    }
}

fn delete_transaction(transactions: &mut TransactionCollection) {
    println!("Deleting transaction...");
    let s_id = get_user_input("Enter the ID of the transaction to delete: ");
    let result = transactions.delete_transaction(&s_id);
    match result {
        Ok(_) => println!("Transaction deleted"),
        Err(e) => println!("Error deleting transaction: {:?}", e),
    }
}

pub fn run() {
    let mut transactions = TransactionCollection::new();
    loop {
        println!("Welcome to the Transaction Manager");
        println!("1. Add Transaction");
        println!("2. View Transactions");
        println!("3. Delete Transaction");
        println!("4. Exit");

        let choice = get_user_input("Enter your choice: ");
        match choice.parse::<u8>() {
            Ok(1) => add_transaction(&mut transactions),
            Ok(2) => view_transactions(&transactions),
            Ok(3) => delete_transaction(&mut transactions),
            Ok(4) => break,
            _ => println!("Invalid choice"),
        }
    }
}
