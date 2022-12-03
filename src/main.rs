mod day01;

use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = parse_args(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    let result = day01::part1(file_path).unwrap_or_else(|err| {
        eprintln!("Problem running task for {file_path}: {err}");
        process::exit(1);
    });
    println!("{result}");
}

fn parse_args(args: &[String]) -> Result<&str, &'static str> {
    if args.len() < 2 {
        return Err("Missing input file path");
    }
    let file_path = &args[1];
    Ok(file_path)
}
