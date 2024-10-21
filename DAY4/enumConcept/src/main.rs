struct Product {
    name: String,
    price: f64,
    in_stock: bool,
}

fn main() {
    let laptop = Product {
        name: String::from("Laptop"),
        price: 1499.99,
        in_stock: true,
    };

    // Access fields
    println!("Product: {}, Price: ${}, In Stock: {}", laptop.name, laptop.price, laptop.in_stock);
}
