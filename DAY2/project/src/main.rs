fn main() {
    // Ownership Example
    let s1 = String::from("Hello, Rust!");
    let s2 = s1; // Ownership of the String moves to `s2`
    // println!("{}", s1); // This will cause an error, because `s1` no longer owns the String
    println!("s2: {}", s2); // `s2` is now the owner

    // Borrowing and References Example
    let s3 = String::from("Borrowed String");
    print_length(&s3); // Borrow `s3` without transferring ownership
    println!("s3 is still accessible: {}", s3); // `s3` can still be used here

    // Mutable References Example
    let mut text = String::from("Hello");
    append_world(&mut text); // Mutably borrow `text` to modify it
    println!("Modified text: {}", text); // `text` now has been modified

    // Combining Borrowing and Ownership Transfer
    let mut a = 5;
    let mut b = 10;
    println!("Before swap: a = {}, b = {}", a, b);
    swap_values(&mut a, &mut b); // Mutably borrow `a` and `b`
    println!("After swap: a = {}, b = {}", a, b); // Values have been swapped
}

// Function to demonstrate borrowing
fn print_length(s: &String) {
    println!("The length of '{}' is {}", s, s.len());
}

// Function to demonstrate mutable references
fn append_world(s: &mut String) {
    s.push_str(", world!");
}

// Function to demonstrate mutable references with multiple variables
fn swap_values(x: &mut i32, y: &mut i32) {
    let temp = *x; // Dereference `x` to get the value
    *x = *y;       // Assign value of `y` to `x`
    *y = temp;     // Assign `temp` to `y`
}
