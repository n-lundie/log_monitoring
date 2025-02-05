use std::{env, fs};

use chrono::{DateTime, TimeZone, Utc};

fn main() -> Result<(), String> {
    let input = ingest_file()?;

    return Ok(());
}

fn ingest_file() -> Result<String, String> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return Err("must provide a path to input file".into());
    }

    let input = fs::read_to_string(&args[1]);

    match input {
        Ok(data) => Ok(data),
        Err(_) => Err("invalid input path".into()),
    }
}

#[derive(PartialEq, Debug)]
struct CsvRow(DateTime<Utc>, String, String, String);

fn parse_csv(input: String) -> Result<Vec<CsvRow>, String> {
    return Ok(vec![]);
}

#[cfg(test)]
mod parse_csv {
    use super::*;

    #[test]
    fn should_parse_valid_input() {
        let input = String::from("11:35:23,scheduled task 032, START,37980\n11:35:56,scheduled task 032, END,37980\n11:36:11,scheduled task 796, START,57672");
        let result = parse_csv(String::from(input));

        let expected_value = CsvRow(
            get_timestamp(11, 35, 23),
            String::from("scheduled task 032"),
            String::from("START"),
            String::from("37980"),
        );

        match result {
            Ok(e) => assert_eq!(e[0], expected_value),
            Err(e) => panic!("Expected Ok, received: {}", e),
        }
    }
}

fn get_timestamp(hour: u32, minute: u32, second: u32) -> DateTime<Utc> {
    // logs do not provide year, month or date so we provide defaults
    return Utc.with_ymd_and_hms(0, 1, 1, hour, minute, second).unwrap();
}
