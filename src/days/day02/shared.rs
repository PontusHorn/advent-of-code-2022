#[derive(Debug)]
pub enum Move {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
pub enum Outcome {
    Win,
    Loss,
    Draw,
}

pub fn get_score(own_move: &Move, outcome: &Outcome) -> u32 {
    let move_score = match own_move {
        Move::Rock => 1,
        Move::Paper => 2,
        Move::Scissors => 3,
    };
    let outcome_score = match outcome {
        Outcome::Loss => 0,
        Outcome::Draw => 3,
        Outcome::Win => 6,
    };

    move_score + outcome_score
}
