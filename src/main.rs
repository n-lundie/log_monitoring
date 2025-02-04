use std::{env, fs};

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return Err("must provide a path to input file".into());
    }

    let input = fs::read_to_string(&args[1]);

    if let Err(_) = input {
        return Err("invalid input path".into());
    }

    return Ok(());
}
