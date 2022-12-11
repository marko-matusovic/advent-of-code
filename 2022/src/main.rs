#![allow(dead_code)]

use std::env;

mod data_loader;
mod solutions;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let args: Vec<String> = env::args().collect();
    solutions::run(&args).await;
}
