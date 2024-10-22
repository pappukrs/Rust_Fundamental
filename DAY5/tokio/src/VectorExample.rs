fn main() {

    let mut friend: Vec<String> = Vec::new();
    friend.push(String::from("RK"));
    friend.push(String::from("Rajeev"));
    friend.push(String::from("Sanoj"));
    println!("{:?}", friend);


    friend.pop();
    println!("{:?}",&friend);
    println!("{}",friend.is_empty());
    println!("{}",friend.len());

 

    for x in &friend{
        println!("{}",x);
    }
}
