mod days;
mod input;

use days::day01::part2::run;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = parse_args(&args).expect("Problem parsing arguments");
    let result = run(file_path).expect(&format!("Problem running task for {file_path}"));
    println!("{result:?}");
}

fn parse_args(args: &[String]) -> Result<&str, &'static str> {
    if args.len() < 2 {
        return Err("Missing input file path");
    }
    let file_path = &args[1];
    Ok(file_path)
}
