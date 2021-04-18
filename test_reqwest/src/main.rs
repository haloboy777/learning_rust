use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://httpbin.org/headers")
        .await?
        .json()
        .await?;
    println!("{:?}", resp);
    Ok(())
}