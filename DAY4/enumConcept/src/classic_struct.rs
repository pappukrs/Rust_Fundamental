// src/classic_struct.rs

pub struct Product {
    pub name: String,
    pub price: f64,
    pub in_stock: bool,
}

impl Product {
    pub fn new(name: String, price: f64, in_stock: bool) -> Self {
        Self { name, price, in_stock }
    }
}
