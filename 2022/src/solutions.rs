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

use day_11 as today;

use super::data_loader::data_for_day;

pub async fn run(){
    let raw = data_for_day(today::day()).await;
    let input = today::parse_input(&raw);
    today::part_1(&input);
    today::part_2(&input);
}
