#![allow(dead_code)]

mod data_loader;
mod solutions;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    solutions::run().await;
}
