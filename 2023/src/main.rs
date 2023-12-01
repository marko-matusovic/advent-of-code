#![allow(dead_code)]

use std::env;

use data_loader::InputType;
use solutions::Part;

mod data_loader;
mod solutions;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let args: Vec<String> = env::args().collect();
    
    let day: u32 = args[2].parse().expect("Not a valid number");
    
    let input_type: InputType = if args.contains(&String::from("--ex")) {
        InputType::EXAMPLE
    } else {
        InputType::MY
    };

    let part: Part = if args.contains(&String::from("--P1")) {
        Part::ONE
    } else if args.contains(&String::from("--P2")) {
        Part::TWO
    } else {
        Part::ALL
    };

    solutions::run(day, input_type, part).await;
}
