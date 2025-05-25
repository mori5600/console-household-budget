mod amount;
mod category;

fn main() {
    let amount = amount::Amount::new(10);
    match amount {
        Ok(a) => println!("Amount created: {}", a.value()),
        Err(e) => println!("Error creating amount: {}", e),
    }
}
