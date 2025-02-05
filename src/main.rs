use log_monitoring::{generate_report, ingest_file, parse_csv};

fn main() -> Result<(), String> {
    let input = ingest_file()?;

    let data = parse_csv(input)?;

    let report = generate_report(&data)?;

    return Ok(());
}
