use super::{cpu::Cpu, cpu_error::CpuError, instruction::Instruction};
use crate::{days::day10::cpu::Register, input::read_lines};

pub fn run(file_path: &str) -> Result<i64, CpuError> {
    let mut cpu = Cpu::new();
    let mut signal_strengths = Vec::new();
    let lines = read_lines(file_path)?;
    for line in lines {
        let instruction: Instruction = line?.as_str().try_into()?;
        cpu.add_instruction(instruction);
        while cpu.pending_instructions_len() > 0 {
            let (cycle_count, Register { x }) = cpu.cycle();
            if [20, 60, 100, 140, 180, 220].contains(&cycle_count) {
                signal_strengths.push(x * cycle_count as i64);
            }
        }
    }
    Ok(signal_strengths.iter().sum())
}
