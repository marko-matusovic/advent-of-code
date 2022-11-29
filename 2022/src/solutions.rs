mod day_00;

use day_00 as today;

use super::data_loader::data_for_day;

pub fn run(){
    let raw = data_for_day(today::day());
    let input = today::parse_input(&raw);
    today::part_1(&input);
    today::part_2(&input);
}
