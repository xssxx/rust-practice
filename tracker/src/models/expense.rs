pub struct Expense {
    pub id: i32,
    pub amount: f64,
    pub description: String,
    pub category: String,
    pub date: String,
}

impl Expense {
    pub fn new(id: i32, amount: f64, description: String, category: String, date: String) -> Self {
        Expense {
            id,
            amount,
            description,
            category,
            date,
        }
    }

    pub fn display(&self) {
        println!(
            "ID: {}, Amount: {}, Description: {}, Category: {}, Date: {}",
            self.id, self.amount, self.description, self.category, self.date
        );
    }
}