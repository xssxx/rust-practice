use crate::models::expense::Expense;

pub trait DataSoruce {
    fn read_expense(&self) -> Vec<Expense>;
    fn write_expense(&self, expenses: &[Expense]);
}