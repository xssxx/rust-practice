use chrono::Local;

use crate::models::expense::Expense;
use crate::services::datasource::DataSoruce;

pub struct ExpenseService<T: DataSoruce> {
    pub datasource: T,
}

impl<T: DataSoruce> ExpenseService<T> {
    pub fn list(&self) -> Vec<Expense> {
        let expenses = self.datasource.read_expense();

        for expense in &expenses {
            expense.display();
        }

        expenses
    }

    pub fn add(&self, description: String, amount: f64, category: String) {
        let mut expenses = self.datasource.read_expense();
        let new_id = expenses.len() as i32 + 1;
        expenses.push(Expense { 
            id: new_id,
            amount: amount, 
            description: description, 
            category: category, 
            date: Local::now().format("%Y-%m-%d").to_string(),
        });

        self.datasource.write_expense(&expenses);
    }
}