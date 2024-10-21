// src/struct_with_impl.rs

pub struct Product {
    pub name: String,
    pub price: f64,
    pub in_stock: bool,
}

impl Product {
    // Make the new function public so it can be used outside the module
    pub fn new(name: String, price: f64, in_stock: bool) -> Self {
        Self { name, price, in_stock }
    }

    pub fn is_affordable(&self, budget: f64) -> bool {
        self.price <= budget
    }
}
