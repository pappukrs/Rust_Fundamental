fn main() {
    let x = 10;
    println!("Non-mutable x: {}", x);

    let mut y = 10;
    y = 20;
    println!("Mutable y after change: {}", y);

    let z = 5;
    let z = z + 1;
    println!("Shadowed z: {}", z);

    let mut a = 30;
    modify_value(&mut a);
    println!("Modified a: {}", a);

    let b = 50;
    print_value(&b);
}

fn modify_value(val: &mut i32) {
    *val += 10;
}

fn print_value(val: &i32) {
    println!("Immutable reference value: {}", val);
}
