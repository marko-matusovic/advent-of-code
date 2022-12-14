mod day_01;
mod day_02;
mod day_03;

use day_03 as today;

use super::data_loader::data_for_day;

pub async fn run(){
    let raw = data_for_day(today::day()).await;
    let input = today::parse_input(&raw);
    today::part_1(&input);
    today::part_2(&input);
}
