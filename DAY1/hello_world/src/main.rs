fn set_up(name:&str)->String{
    format!("hello,{}",name)
}

fn main() {
    let msg =set_up("Papa Mummy");
    println!("{}",msg);
    let is = check_adult(69);
    println!("{}",is);
    println!("Hello, world!");
}


fn check_adult(age:i32)->bool{
  
    if age>=18{
        return true;
    }
    return false;
}
