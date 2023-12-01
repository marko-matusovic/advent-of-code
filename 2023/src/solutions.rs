mod day_trait;
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

use super::data_loader::data_for_day;
use crate::{data_loader::InputType, solutions::day_trait::Day};

#[derive(Eq, PartialEq)]
pub enum Part {
    ONE,
    TWO,
    ALL
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
        01 => Box::new(day_01::Day01{}),
        02 => Box::new(day_02::Day02{}),
        03 => Box::new(day_03::Day03{}),
        04 => Box::new(day_04::Day04{}),
        05 => Box::new(day_05::Day05{}),
        06 => Box::new(day_06::Day06{}),
        07 => Box::new(day_07::Day07{}),
        08 => Box::new(day_08::Day08{}),
        09 => Box::new(day_09::Day09{}),
        10 => Box::new(day_10::Day10{}),
        11 => Box::new(day_11::Day11{}),
        12 => Box::new(day_12::Day12{}),
        13 => Box::new(day_13::Day13{}),
        14 => Box::new(day_14::Day14{}),
        15 => Box::new(day_15::Day15{}),
        16 => Box::new(day_16::Day16{}),
        17 => Box::new(day_17::Day17{}),
        18 => Box::new(day_18::Day18{}),
        19 => Box::new(day_19::Day19{}),
        20 => Box::new(day_20::Day20{}),
        21 => Box::new(day_21::Day21{}),
        22 => Box::new(day_22::Day22{}),
        23 => Box::new(day_23::Day23{}),
        24 => Box::new(day_24::Day24{}),
        25 => Box::new(day_25::Day25{}),
        _ => panic!("Unexpected day!")
    };
    
    println!("Running Day {} for {} input.", today.day(), input_type.name().to_uppercase());
    let input = data_for_day(today.day(), &input_type).await;
    if part.is(Part::ONE) {
        today.part_1(&input);
    }
    if part.is(Part::TWO) {
        today.part_2(&input);
    }
}
