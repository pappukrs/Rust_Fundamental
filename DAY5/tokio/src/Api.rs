use reqwest::Error;

async fn fetch_data() -> Result<String, Error> {
    let url = "https://jsonplaceholder.typicode.com/posts/1";
    let response = reqwest::get(url).await?;
    let body = response.text().await?;
    Ok(body)
}

#[tokio::main]
async fn main() {
    match fetch_data().await {
        Ok(data) => println!("Received data: {}", data),
        Err(e) => println!("Error occurred: {}", e),
    }
}
