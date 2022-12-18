use super::instruction_error::InstructionError;

#[derive(Debug)]
pub enum Instruction {
    Noop,
    AddX(i64),
}

impl TryFrom<&str> for Instruction {
    type Error = InstructionError;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s.split_whitespace().collect::<Vec<&str>>()[..] {
            ["noop"] => Ok(Instruction::Noop),
            ["addx", value] => {
                let value = value
                    .parse()
                    .map_err(|_| InstructionError::InvalidAddXValue(value.to_string()))?;
                Ok(Instruction::AddX(value))
            }
            _ => Err(InstructionError::InvalidInstruction(s.to_string())),
        }
    }
}
