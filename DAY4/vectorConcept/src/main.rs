fn main() {

    // 1. Creating a Vector
    let mut numbers: Vec<i32> = Vec::new(); // Creating an empty vector
    numbers.push(10); // Adding elements
    numbers.push(20);
    numbers.push(30);

    // Using the vec! macro for creating a vector with initial values
    let mut fruits = vec!["Apple", "Banana", "Cherry"];
    println!("Fruits: {:?}", fruits);

    // 2. Accessing Elements
    println!("First number: {}", numbers[0]); // Using indexing
    match numbers.get(1) {
        Some(value) => println!("Second number: {}", value),
        None => println!("No value at this index"),
    }

    // 3. Modifying Elements
    numbers[1] = 25; // Changing the value at index 1
    println!("Modified numbers: {:?}", numbers);

    // 4. Removing Elements
    fruits.pop(); // Removes the last element
    println!("Fruits after pop: {:?}", fruits);

    // Remove an element at a specific index
    if fruits.len() > 1 {
        fruits.remove(1); // Removes "Banana"
        println!("Fruits after removing index 1: {:?}", fruits);
    }

    // 5. Iterating Over Elements
    for num in &numbers {
        println!("Number: {}", num);
    }

    for fruit in &fruits {
        println!("Fruit: {}", fruit);
    }

    // 6. Using Vectors with Enums
    enum Direction {
        Up,
        Down,
        Left,
        Right,
    }

    let movements = vec![Direction::Up, Direction::Down, Direction::Left,Direction::Right];
    for movement in &movements {
        match movement {
            Direction::Up => println!("Moving up!"),
            Direction::Down => println!("Moving down!"),
            Direction::Left => println!("Moving left!"),
            Direction::Right => println!("Moving right!"),
        }
    }

    // 7. Vector of Structs
    struct User {
        username: String,
        age: u32,
    }

    let user1 = User {
        username: String::from("Alice"),
        age: 30,
    };
    let user2 = User {
        username: String::from("Bob"),
        age: 25,
    };

    let users = vec![user1, user2];
    for user in &users {
        println!("User: {}, Age: {}", user.username, user.age);
    }

    // 8. Using .iter(), .iter_mut(), and .into_iter()
    // .iter() borrows each element of the vector
    for num in numbers.iter() {
        println!("Read-only: {}", num);
    }

    // .iter_mut() allows modifying the elements
    for num in numbers.iter_mut() {
        *num += 5;
    }
    println!("Numbers after modification: {:?}", numbers);

    // .into_iter() consumes the vector, transferring ownership of elements
    let sum: i32 = numbers.into_iter().sum(); // `numbers` can no longer be used after this
    println!("Sum of numbers: {}", sum);
}
