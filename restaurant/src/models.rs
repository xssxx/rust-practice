#[derive(Clone, Debug)]
pub struct Food {
    pub id: i32,
    pub name: String,
    pub price: f64,
}

#[derive(Clone, Debug)]
pub struct Order {
    pub id: i32,
    pub total: f64,
}

#[derive(Clone, Debug)]
pub struct OrderItem {
    pub order_id: i32,
    pub food_id: i32,
}