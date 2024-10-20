fn stack_variables_example() {
    let num1 = 5;
    let num2 = 10;
    println!("Sum: {}", add(num1, num2));
}

fn add(a: i32, b: i32) -> i32 {
    let result = a + b;
    result
}

fn scope_example() {
    let outer = 100;
    {
        let inner = 200;
        println!("Inner value: {}", inner);
    }
    println!("Outer value: {}", outer);
}

fn ownership_move() {
    let heap_str1 = String::from("hello");
    let heap_str2 = heap_str1;
    println!("Heap String 2: {}", heap_str2);
}

fn pass_heap_to_function() {
    let heap_string = String::from("ownership test");
    consume_string(heap_string);
}

fn consume_string(input: String) {
    println!("Consumed String: {}", input);
}

fn clone_example() {
    let original = String::from("rusty");
    let copy = original.clone();
    println!("Original: {}", original);
    println!("Copy: {}", copy);
}

fn return_ownership() {
    let owned_str = String::from("return me");
    let new_owner = take_and_return(owned_str);
    println!("Returned String: {}", new_owner);
}

fn take_and_return(data: String) -> String {
    println!("Processing String: {}", data);
    data
}

fn mutable_reference_example() {
    let mut mutable_str = String::from("mutable");
    append_to_string(&mut mutable_str);
    println!("Updated String: {}", mutable_str);
}

fn append_to_string(s: &mut String) {
    s.push_str(" reference");
}

fn multiple_immutable_refs() {
    let base_str = String::from("immutable");
    let ref1 = &base_str;
    let ref2 = &base_str;
    println!("Reference 1: {}", ref1);
    println!("Reference 2: {}", ref2);
}

fn single_mutable_ref() {
    let mut changeable_str = String::from("change");
    let ref1 = &mut changeable_str;
    println!("Mutable Reference: {}", ref1);
}

fn mixed_refs() {
    let hybrid_str = String::from("hybrid");
    let immutable_ref = &hybrid_str;
    println!("Immutable Reference: {}", immutable_ref);
}

fn main() {
    println!("\nExample #1 - Stack Variables in Functions:");
    stack_variables_example();

    println!("\nExample #2 - Scoping Variables:");
    scope_example();

    println!("\nHeap Example #1 - Ownership Move:");
    ownership_move();

    println!("\nPassing Heap Variables to Functions:");
    pass_heap_to_function();

    println!("\nFix Ownership Issue with Cloning:");
    clone_example();

    println!("\nReturn Ownership to Main:");
    return_ownership();

    println!("\nUsing Mutable References:");
    mutable_reference_example();

    println!("\nAllow Multiple Immutable References:");
    multiple_immutable_refs();

    println!("\nRestrict to Single Mutable Reference:");
    single_mutable_ref();

    println!("\nNo Mixing of Mutable and Immutable References:");
    mixed_refs();
}
