use super::instruction::Instruction;
use std::collections::VecDeque;

#[derive(Debug)]
pub struct Cpu {
    current_cycle: u32,
    register: Register,
    current_instruction: Option<Instruction>,
    instructions: VecDeque<Instruction>,
    remaining_cycles: u32,
}

#[derive(Debug, Clone)]
pub struct Register {
    pub x: i64,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            current_cycle: 0,
            register: Register { x: 1 },
            current_instruction: None,
            instructions: VecDeque::new(),
            remaining_cycles: 0,
        }
    }

    pub fn add_instruction(&mut self, instruction: Instruction) {
        self.instructions.push_back(instruction);
    }

    pub fn cycle(&mut self) -> (u32, Register) {
        self.current_cycle += 1;

        // Start new instruction if none is currently running
        if self.current_instruction.is_none() && !self.instructions.is_empty() {
            if let Some(instruction) = self.instructions.pop_front() {
                self.begin(instruction);
            }
        }

        self.process();

        // Make a copy of the current register since we want to return the value during *this* cycle
        let current_register = self.register.clone();

        // Finish current instruction if done processing
        if self.remaining_cycles == 0 && self.current_instruction.is_some() {
            self.finish_current();
            self.current_instruction = None;
        }

        (self.current_cycle, current_register)
    }

    fn begin(&mut self, instruction: Instruction) {
        self.remaining_cycles = match instruction {
            Instruction::Noop => 1,
            Instruction::AddX(_) => 2,
        };
        self.current_instruction = Some(instruction);
    }

    fn process(&mut self) {
        if self.remaining_cycles > 0 {
            self.remaining_cycles -= 1;
        }
    }

    fn finish_current(&mut self) {
        let instruction = self.current_instruction.as_ref().unwrap();
        match instruction {
            Instruction::Noop => (),
            Instruction::AddX(value) => self.register.x += value,
        }
    }

    pub fn pending_instructions_len(&self) -> usize {
        self.instructions.len()
    }
}
