mod traits;
mod other;

use traits::Taxable;
use other::{Income, Bonus};

fn main() {
    let mut income = Income { amount: 50000.50 };
    let mut bonus = Bonus { amount: 10000.23 };

    println!("Total tax owed on my income: ${:.2}", income.tax_bill());
    println!("Bonus tax owed: ${:.2}", bonus.tax_bill());
    income.double_amount();
    bonus.double_amount();
    println!("Total tax owed on my income: ${:.2}", income.tax_bill());
    println!("Bonus tax owed: ${:.2}", bonus.tax_bill());
}