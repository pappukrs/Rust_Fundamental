use rayon::prelude::*;

fn main() {
    let numbers: Vec<i32> = (1..10).collect();
    let sum: i32 = numbers.par_iter().sum();
    println!("Parallel sum: {}", sum);
}
