#[derive(Debug)]
pub struct FoodOrderReq {
    pub req: Vec<FoodReq>
}

#[derive(Debug)]
pub struct FoodReq {
    pub id: i32,
    pub qty: i32
}