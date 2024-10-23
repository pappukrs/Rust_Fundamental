use chrono::prelude::*;

fn main() {
    let now = Utc::now();
    println!("Current time: {}", now);

    // Use the new method `with_ymd_and_hms` and match on the result
    match Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0) {
        chrono::LocalResult::Single(date) => {
            println!("New Year 2024: {}", date);
        }
        chrono::LocalResult::None => {
            println!("Invalid date or time");
        }
        chrono::LocalResult::Ambiguous(_, _) => {
            println!("Ambiguous date/time");
        }
    }
}
