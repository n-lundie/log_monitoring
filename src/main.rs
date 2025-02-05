use log_monitoring::{generate_report, ingest_file, parse_csv};
use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let input = ingest_file()?;

    let data = parse_csv(input)?;

    let report = generate_report(&data)?;

    let mut advisory = "no issues detected";

    if report.rows.len() > 0 {
        let output = report
            .rows
            .iter()
            .map(|row| format!("{},{},{}", row.0, row.1, row.2))
            .collect::<Vec<String>>()
            .join("\n");

        fs::write("./report.log", output)?;

        advisory = "issues detected, check report.log for details";
    }

    println!(
        "result: {} started; {} completed; {}",
        report.processes_started, report.processes_completed, advisory
    );

    return Ok(());
}
