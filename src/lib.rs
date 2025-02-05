use chrono::{DateTime, TimeZone, Utc};
use std::{env, fs};

pub fn ingest_file() -> Result<String, String> {
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
/// Represents a row in a csv file
///
/// # Fields
///
/// * `0` the time the log was created
/// * `1` a description of the process
/// * `2` the status of the process ("START" or "END")
/// * `3` the process id
pub struct CsvRow(DateTime<Utc>, String, String, String);

pub fn parse_csv(input: String) -> Result<Vec<CsvRow>, String> {
    let mut data = vec![];

    for (i, e) in input.lines().enumerate() {
        let input_columns: Vec<&str> = e.split(",").map(|e| e.trim()).collect();

        if input_columns.len() > 4 {
            return Err(format!("unexpected column, line {}", i + 1));
        }

        // seperate timestamp into segments and try to parse to u32 otherwise return error
        let time_segments: Vec<u32> = input_columns[0]
            .split(":")
            .map(|e| {
                e.parse::<u32>()
                    .map_err(|_| format!("invalid timestamp, line {}", i + 1))
            })
            .collect::<Result<Vec<u32>, String>>()?;

        let timestamp = get_timestamp(time_segments[0], time_segments[1], time_segments[2]);

        let status = input_columns[2].to_string();

        if status.ne("START") && status.ne("END") {
            return Err(format!("invalid status, line {}", i + 1));
        }

        data.push(CsvRow(
            timestamp,
            input_columns[1].to_string(),
            status,
            input_columns[3].to_string(),
        ));
    }

    return Ok(data);
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

    #[test]
    fn should_return_error_on_invalid_time() {
        let input = String::from("11:35:23,scheduled task 032, START,37980\n11:35:5d,scheduled task 032, END,37980\n11:36:11,scheduled task 796, START,57672");
        let result = parse_csv(String::from(input));

        match result {
            Ok(_) => panic!("Expected Err, receive Ok"),
            Err(e) => assert_eq!(e, "invalid timestamp, line 2"),
        }
    }

    #[test]
    fn should_return_error_on_unexpected_column() {
        let input = String::from("11:35:23,scheduled task 032, START,37980\n11:35:56,scheduled task 032, END,37980\n11:36:11,scheduled task 796, START,57672,bad column");
        let result = parse_csv(String::from(input));

        match result {
            Ok(_) => panic!("Expected Err, receive Ok"),
            Err(e) => assert_eq!(e, "unexpected column, line 3"),
        }
    }

    #[test]
    fn should_return_error_on_invalid_status() {
        let input = String::from("11:35:23,scheduled task 032, START,37980\n11:35:56,scheduled task 032,not a status,37980\n11:36:11,scheduled task 796, START,57672");
        let result = parse_csv(String::from(input));

        match result {
            Ok(_) => {
                panic!("Expected Err, receive Ok")
            }
            Err(e) => assert_eq!(e, "invalid status, line 2"),
        }
    }
}

fn get_timestamp(hour: u32, minute: u32, second: u32) -> DateTime<Utc> {
    // logs do not provide year, month or day so we provide defaults
    return Utc.with_ymd_and_hms(0, 1, 1, hour, minute, second).unwrap();
}

#[derive(PartialEq, Debug)]
struct Report {
    processes_started: u32,
    processes_completed: u32,
    rows: Vec<ReportRow>,
}

#[derive(PartialEq, Debug)]
/// Represents a row to be included in report  
///
/// # Fields
///
/// * `0` the process id
/// * `1` the level of alert ("WARNING" or "ERROR")
/// * `2` duration of process in seconds
struct ReportRow(String, String, i64);

fn generate_report(data: &Vec<CsvRow>) -> Result<Report, String> {}

#[cfg(test)]
mod generate_report {
    use super::*;

    #[test]
    fn should_return_report_on_valid_data() {
        let input = vec![
            CsvRow(
                get_timestamp(11, 35, 23),
                String::from("some description"),
                String::from("START"),
                String::from("37980"),
            ),
            CsvRow(
                get_timestamp(11, 35, 56),
                String::from("some description"),
                String::from("END"),
                String::from("37980"),
            ),
            CsvRow(
                get_timestamp(11, 36, 11),
                String::from("some description"),
                String::from("START"),
                String::from("57672"),
            ),
            CsvRow(
                get_timestamp(11, 36, 58),
                String::from("some description"),
                String::from("START"),
                String::from("81258"),
            ),
            CsvRow(
                get_timestamp(11, 42, 18),
                String::from("some description"),
                String::from("END"),
                String::from("57672"),
            ),
            CsvRow(
                get_timestamp(11, 43, 58),
                String::from("some description"),
                String::from("START"),
                String::from("39547"),
            ),
            CsvRow(
                get_timestamp(11, 56, 58),
                String::from("some description"),
                String::from("END"),
                String::from("81258"),
            ),
        ];

        let result = generate_report(&input);

        match result {
            Ok(report) => {
                assert_eq!(report.processes_started, 4);
                assert_eq!(report.processes_completed, 3);
                assert_eq!(report.rows.len(), 2);
            }
            Err(e) => panic!("Expected Ok, received: {}", e),
        }
    }
}
