fn main() {
    println!("Borrowing concept");
    string_borrowing();
}

fn string_borrowing(){
    let mut  str = String::from("hello Alka");
    // let c_str =&str;
    // println!("{}",c_str);
    // println!("{}",str);

    update_str(&mut str);
    
}

// fn takes_ownership(str:&String){
//     println!("{}",str);
// }

fn update_str(word:& mut String){
    word.push_str(" pappu ");
    println!("{}",word);
}