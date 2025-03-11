use crate::{db::DATABASE, models::{Order, OrderItem}, request::FoodOrderReq};

pub struct OrderService;

impl OrderService {
    pub fn new() -> Self {
        Self
    }

    pub fn add_order(&self, req: FoodOrderReq) -> Order {
        let mut db = DATABASE.lock().unwrap();

        let mut total = 0.0;
        let order_id = db.orders.len() as i32 + 1;

        let mut order_items: Vec<OrderItem> = Vec::new();
        for food_req in &req.req {
            if let Some(food) = db.foods.iter().find(|f| f.id == food_req.id) {
                total += food.price * food_req.qty as f64;

                order_items.push(OrderItem {
                    order_id,
                    food_id: food.id,
                });
            }
        }
        
        db.order_items.extend(order_items);

        let new_order = Order { id: order_id, total };
        db.orders.push(new_order.clone());

        new_order
    }
}