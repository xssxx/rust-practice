mod services;
mod models;

use services::expense_datasource::ExpenseDataSource;
use services::expense_service::ExpenseService;

fn main() {
    let datasource = ExpenseDataSource::new(
        "src/data".to_string(), 
        "expenses.csv".to_string()
    );

    let service = ExpenseService {
        datasource: datasource,
    };

    service.list();
    service.add(
        "Test".to_string(), 
        100.0, 
        "Test".to_string()
    );
    println!("After adding a new expense:");
    service.list();
}
