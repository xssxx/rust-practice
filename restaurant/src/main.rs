use restaurant::{db::DATABASE, request::{self, FoodOrderReq}, service::OrderService};

fn main() {
    let order_service = OrderService::new();

    let food_order_req = FoodOrderReq {
        req: vec![
            request::FoodReq { id: 1, qty: 3 },
            request::FoodReq { id: 3, qty: 1 },
        ],
    };

    let new_order = order_service.add_order(food_order_req);

    println!("New Order: {:?}", new_order);

    let db = DATABASE.lock().unwrap();
    println!("Orders in DB: {:?}", db.orders);
    println!("Order Items in DB: {:?}", db.order_items);
}
