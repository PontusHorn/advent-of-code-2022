use super::{instruction::Instruction, rope::Rope, rope_error::RopeError};
use crate::input::read_lines;

pub fn run(file_path: &str) -> Result<usize, RopeError> {
    let mut rope = Rope::new(0, 0);
    let lines = read_lines(file_path)?;
    for line in lines {
        let instruction: Instruction = line?.as_str().try_into()?;
        instruction.apply(&mut rope);
    }
    Ok(rope.tail_visited().len())
}
