use crate::models::{Food, Order, OrderItem};
use once_cell::sync::Lazy;
use std::sync::Mutex;

#[derive(Debug)]
pub struct MockDB {
    pub foods: Vec<Food>,
    pub orders: Vec<Order>,
    pub order_items: Vec<OrderItem>,
}

// สร้าง static database
pub static DATABASE: Lazy<Mutex<MockDB>> = Lazy::new(|| {
    Mutex::new(MockDB {
        foods: vec![
            Food { id: 1, name: "Apple".to_string(), price: 12.99 },
            Food { id: 2, name: "Burger".to_string(), price: 49.99 },
            Food { id: 3, name: "Pizza".to_string(), price: 199.00 },
        ],
        orders: vec![
            Order { id: 1, total: 62.98 },
            Order { id: 2, total: 199.00 },
        ],
        order_items: vec![
            OrderItem { order_id: 1, food_id: 1 },  // Apple in Order 1
            OrderItem { order_id: 1, food_id: 2 },  // Burger in Order 1
            OrderItem { order_id: 2, food_id: 3 },  // Pizza in Order 2
        ],
    })
});
