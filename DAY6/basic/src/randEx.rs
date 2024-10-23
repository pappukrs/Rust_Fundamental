use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let random_number: u8 = rng.gen();
    println!("Random number: {}", random_number);
}
