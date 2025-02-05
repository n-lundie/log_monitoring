use log_monitoring::{ingest_file, parse_csv};

fn main() -> Result<(), String> {
    let input = ingest_file()?;

    let data = parse_csv(input)?;

    return Ok(());
}
