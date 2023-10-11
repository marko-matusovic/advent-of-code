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

use day_16 as today;

use super::data_loader::data_for_day;
use crate::data_loader::InputType;

pub async fn run(args: &Vec<String>){
    let input_type: InputType = match args[1].as_str() {
        "-ex" => InputType::EXAMPLE,
        "-my" => InputType::MY,
        _ => InputType::MY,
    };
    println!("Running Day {} for {} input.", today::day(), input_type.name().to_uppercase());
    let raw = data_for_day(today::day(), &input_type).await;
    let input = today::parse_input(&raw);
    today::part_1(&input);
    today::part_2(&input);
}
