use std::fs;

pub fn data_for_day(day: u8) -> String {
    match read_day(day) {
        Ok(data) => data,
        _ => fetch_day(day),
    }
}

fn read_day(day: u8) -> Result<String, std::io::Error> {
    let file_path = format!("./input/day_{:02}.d", &day);
    println!("filepath: {}", &file_path);
    fs::read_to_string(file_path)
}

fn fetch_day(day: u8) -> String {
    // https://adventofcode.com/{}/day/{}/input
    format!("download data for day {}", &day)
}
