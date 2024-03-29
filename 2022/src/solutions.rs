mod day_00;
mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;
mod day_10;
mod day_11;
mod day_12;
mod day_13;
mod day_14;
mod day_15;
mod day_16;
mod day_17;
mod day_18;
mod day_19;
mod day_20;
mod day_21;
mod day_22;
mod day_23;
mod day_24;
mod day_25;

use day_25 as today;

use super::data_loader::data_for_day;
use crate::data_loader::InputType;

pub async fn run(args: &Vec<String>){
    
    let input_type: InputType = match args.get(2).or_else(|| args.get(1)).map(|s| s.as_str()) {
        Some("-ex") => InputType::EXAMPLE,
        Some("-my") => InputType::MY,
        _ => InputType::MY,
    };
    println!("Running Day {} for {} input.", today::day(), input_type.name().to_uppercase());
    let raw = data_for_day(today::day(), &input_type).await;
    let input = today::parse_input(&raw);
    if !args.contains(&String::from("-p2")) {
        today::part_1(&input);
    }
    if !args.contains(&String::from("-p1")) {
        today::part_2(&input);
    }
}
