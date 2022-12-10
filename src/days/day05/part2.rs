use super::shared::{get_stacks, lift_crates, place_crates, Crate, Move, Result, Stack, Stacks};
use crate::input::read_lines;

pub fn run(file_path: &str) -> Result<String> {
    let stacks = get_stacks(file_path)?;
    let Stacks(rearranged_stacks) = move_crates(file_path, stacks)?;
    let top_crates: Vec<&Crate> = rearranged_stacks
        .iter()
        .map(|Stack { crates, .. }| crates.last().unwrap_or(&Crate(' ')))
        .collect();
    let answer: String = top_crates
        .iter()
        .map(|Crate(crate_char)| crate_char)
        .collect();

    Ok(answer)
}

fn move_crates(file_path: &str, Stacks(stacks): Stacks) -> Result<Stacks> {
    let instruction_lines = read_lines(file_path)?
        .skip_while(|line| line.as_ref().map_or(false, |line| !line.is_empty()))
        .skip(1);
    let mut rearranged_stacks = Stacks(Vec::new());
    for Stack { name, crates } in stacks {
        rearranged_stacks.0.push(Stack {
            name,
            crates: crates.to_vec(),
        });
    }

    for line in instruction_lines {
        let Move { count, from, to } = line?.parse::<Move>()?;
        let (new_stacks, moved_crates) = lift_crates(&rearranged_stacks, &from, &count)?;
        rearranged_stacks = place_crates(&new_stacks, &to, moved_crates)?;
    }

    Ok(rearranged_stacks)
}
