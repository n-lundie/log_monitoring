use std::{env, fs};

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
