// src/main.rs

mod classic_struct;
mod tuple_struct;
mod unit_like_struct;
mod struct_with_impl;
mod generics_struct;

use classic_struct::Product as ClassicProduct;
use tuple_struct::Color;
use unit_like_struct::Placeholder;
use struct_with_impl::Product as ImplProduct;
use generics_struct::Point;

fn main() {
    // Using Classic Struct
    let classic_product = ClassicProduct::new(String::from("Laptop"), 1499.99, true);
    println!("Product: {}, Price: ${}, In Stock: {}", 
        classic_product.name, classic_product.price, classic_product.in_stock);

    // Using Tuple Struct
    let red = Color(255, 0, 0);
    println!("Color: ({}, {}, {})", red.0, red.1, red.2);

    // Using Unit-Like Struct
    let _placeholder = Placeholder;
    println!("Unit-like struct instance created.");

    // Using Struct with `impl` Block
    let impl_product = ImplProduct::new(String::from("Tablet"), 999.99, true);
    if impl_product.is_affordable(1000.0) {
        println!("You can afford this product!");
    } else {
        println!("This product is too expensive.");
    }

    // Using Struct with Generics
    let integer_point = Point { x: 5, y: 10 };
    let float_point = Point { x: 1.2, y: 3.4 };
    println!("Integer Point: ({}, {})", integer_point.x, integer_point.y);
    println!("Float Point: ({}, {})", float_point.x, float_point.y);
}
