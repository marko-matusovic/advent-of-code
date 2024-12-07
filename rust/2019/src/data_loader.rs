use std::{env, fs, io::Write};

const YEAR: &str = "2019";

pub async fn data_for_day(day: u8) -> String {
    match read_day(day) {
        Ok(data) => {
            println!("Read input from file.");
            data
        }
        _ => fetch_day(day).await,
    }
}

fn read_day(day: u8) -> Result<String, std::io::Error> {
    let file_path = format!("./input/day_{:02}.d", &day);
    println!("filepath: {}", &file_path);
    fs::read_to_string(file_path)
}

async fn fetch_day(day: u8) -> String {
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
    let file_path = format!("./input/day_{:02}.d", &day);
    if let Ok(mut file) = fs::File::create(file_path) {
        file.write(body.as_bytes()).unwrap_or_default();
        file.flush().unwrap_or_default();
    }
}
