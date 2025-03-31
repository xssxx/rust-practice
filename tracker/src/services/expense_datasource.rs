use crate::models::expense::Expense;
use crate::services::datasource::DataSoruce;

use std::fs::File;
use std::io::{BufReader, BufWriter, Write, BufRead};

pub struct ExpenseDataSource {
    pub path: String,
    pub name: String
}

impl ExpenseDataSource {
    pub fn new(path: String, name: String) -> Self {
        ExpenseDataSource {
            path,
            name
        }
    }
}

impl DataSoruce for ExpenseDataSource {
    fn read_expense(&self) -> Vec<Expense> {
        let mut expenses = Vec::new();

        let file_path = format!("{}/{}", self.path, self.name);
        let file = File::open(file_path).expect("Unable to open file");

        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line.expect("Unable to read line");
            // split by comma
            let parts: Vec<&str> = line.split(',').collect(); 
            // check if the line has 5 parts
            if parts.len() == 5 {
                let id: i32 = parts[0].parse().expect("Unable to parse ID");
                let amount: f64 = parts[1].parse().expect("Unable to parse amount");
                let description = parts[2].to_string();
                let category = parts[3].to_string();
                let date = parts[4].to_string();

                expenses.push(Expense::new(id, amount, description, category, date));
            }
        }

        expenses
    }

    fn write_expense(&self, expenses: &[Expense]) {
        let file_path = format!("{}/{}", self.path, self.name);
        let file = File::create(file_path).expect("Unable to create file");

        let mut writer = BufWriter::new(file);

        for expense in expenses {
            // write csv format
            writeln!(
                writer,
                "{},{},{},{},{}",
                expense.id, expense.amount, expense.description, expense.category, expense.date
            )
            .expect("Unable to write to file");
        }
    }
}