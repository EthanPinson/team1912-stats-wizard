use dotenv::dotenv;
use std::env::var;

pub fn yeah() {
    dotenv().ok();




    let api_key = var("API_KEY").expect("API_KEY not set");
    println!("API Key: {}", api_key);
}