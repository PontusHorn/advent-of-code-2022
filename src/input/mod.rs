use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

pub type Lines = io::Lines<io::BufReader<File>>;

pub fn read_lines<P>(filename: P) -> io::Result<Lines>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
