use reqwest; // Make sure `reqwest` is imported.
use tokio; // For asynchronous runtime.

#[tokio::main] // Marks the main function as asynchronous.
async fn main() -> Result<(), reqwest::Error> {
    // Make an HTTP request
    let response = reqwest::get("https://httpbin.org/ip")
        .await?
        .text()
        .await?;

    println!("Response: {}", response);
    Ok(())
}
