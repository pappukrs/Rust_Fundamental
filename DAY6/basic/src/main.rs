use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    description: String,
    done: bool,
}

fn main() {
    // Serialize
    let task = Task { description: "Learn Serde".to_string(), done: true };
    let json = serde_json::to_string(&task).unwrap();
    println!("Serialized to JSON: {}", json);
    //response -->   Serialized to JSON: {"description":"Learn Serde","done":true}


    // Deserialize
    let json_str = r#"{"description":"Learn Serde","done":false}"#;
    let deserialized_task: Task = serde_json::from_str(json_str).unwrap();
    println!("Deserialized: {:?}", deserialized_task);
    //response --> Deserialized: Task { description: "Learn Serde", done: true}
}
