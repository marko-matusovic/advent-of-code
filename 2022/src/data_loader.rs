use std::{env, fs, io::Write};

const YEAR: &str = "2022";

pub enum InputType {
    MY,
    EXAMPLE,
}

impl InputType {
    fn dir(&self) -> &str {
        match self {
            InputType::MY => "my",
            InputType::EXAMPLE => "example",
        }
    }

    pub fn name(&self) -> &str {
        match self {
            InputType::MY => "my",
            InputType::EXAMPLE => "example",
        }
    }
}

pub async fn data_for_day(day: u8, input_type: &InputType) -> String {
    if let Ok(data) = read_day(day, input_type) {
        println!("Read input from file.");
        return data;
    }
    if let InputType::MY = input_type {
        return fetch_personal_input_for_day(day).await;
    }
    panic!("Data cannot be loaded.");
}

fn read_day(day: u8, input_type: &InputType) -> Result<String, std::io::Error> {
    let file_path = format!("./input/{}/day_{:02}", input_type.dir(), &day);
    fs::read_to_string(file_path)
}

async fn fetch_personal_input_for_day(day: u8) -> String {
    println!("Downloading input from AoC.");
    let url = format!("https://adventofcode.com/{}/day/{}/input", YEAR, day);

    let client = reqwest::Client::new();
    let body = client
        .get(url)
        .header(
            "Cookie",
            format!(
                "session={}",
                env::var("SESSION_TOKEN").expect("No session token for AoC found!")
            ),
        )
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap()
        .trim_end()
        .to_owned();

    save_day(day, &body);

    return body;
}

fn save_day(day: u8, body: &str) {
    let file_path = format!("./input/my/day_{:02}", &day);
    if let Ok(mut file) = fs::File::create(file_path) {
        file.write(body.as_bytes()).unwrap_or_default();
        file.flush().unwrap_or_default();
    }
}
