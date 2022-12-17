use super::{instruction_error::InstructionError, rope::Rope};

pub struct Instruction {
    direction: (i8, i8),
    distance: u32,
}

impl Instruction {
    pub fn new(direction: (i8, i8), distance: u32) -> Self {
        Self {
            direction,
            distance,
        }
    }

    pub fn apply(&self, rope: &mut Rope) {
        let (x, y) = self.direction;
        for _ in 0..self.distance {
            rope.travel(x, y);
        }
    }
}

impl TryFrom<&str> for Instruction {
    type Error = InstructionError;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let (direction, distance) = s
            .split_once(" ")
            .ok_or(InstructionError::InvalidInstruction(s.to_string()))?;
        let distance = distance
            .parse()
            .map_err(|_| InstructionError::InvalidDistance(distance.to_string()))?;
        let direction = match direction {
            "R" => (1, 0),
            "L" => (-1, 0),
            "U" => (0, -1),
            "D" => (0, 1),
            _ => return Err(InstructionError::InvalidDirection(direction.to_string())),
        };
        Ok(Self::new(direction, distance))
    }
}
