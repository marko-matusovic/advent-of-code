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
mod day_99;
mod day_trait;

use super::data_loader::data_for_day;
use crate::{data_loader::InputType, solutions::day_trait::Day};

#[derive(Eq, PartialEq)]
pub enum Part {
    ONE,
    TWO,
    ALL,
}

impl Part {
    fn is(&self, p: Part) -> bool {
        match *self {
            Part::ALL => true,
            _ => p == *self,
        }
    }
}

pub async fn run(day: u32, input_type: InputType, part: Part) {
    let today: Box<dyn Day> = match day {
        01 => Box::new(day_01::Day01 {}),
        02 => Box::new(day_02::Day02 {}),
        03 => Box::new(day_03::Day03 {}),
        04 => Box::new(day_04::Day04 {}),
        05 => Box::new(day_05::Day05 {}),
        06 => Box::new(day_06::Day06 {}),
        07 => Box::new(day_07::Day07 {}),
        08 => Box::new(day_08::Day08 {}),
        09 => Box::new(day_09::Day09 {}),
        10 => Box::new(day_10::Day10 {}),
        11 => Box::new(day_11::Day11 {}),
        12 => Box::new(day_12::Day12 {}),
        99 => Box::new(day_99::DayPicnic {}),
        _ => panic!("Unexpected day!"),
    };

    println!(
        "Running Day {} for {} input.",
        today.day(),
        input_type.name().to_uppercase()
    );
    let input = data_for_day(today.day(), &input_type).await;
    if part.is(Part::ONE) {
        today.part_1(&input);
    }
    if part.is(Part::TWO) {
        today.part_2(&input);
    }
}
