use super::knot::Knot;
use std::collections::HashSet;

pub struct Rope {
    knots: Vec<Knot>,
    tail_visited: HashSet<(i32, i32)>,
}

impl Rope {
    pub fn new(knot_count: u8) -> Self {
        let mut knots = Vec::with_capacity(knot_count.into());
        for _ in 0..knot_count {
            knots.push(Knot::new(0, 0));
        }

        let mut tail_visited = HashSet::new();
        tail_visited.insert((0, 0));

        Self {
            knots,
            tail_visited,
        }
    }

    pub fn travel(&mut self, x: i8, y: i8) -> Option<()> {
        let (head, tail) = self.knots.split_first_mut()?;

        head.travel(x, y);

        let mut previous = &*head;
        for knot in tail.iter_mut() {
            knot.follow(previous);
            previous = knot;
        }

        self.tail_visited.insert(previous.position());

        Some(())
    }

    pub fn tail_visited(&self) -> &HashSet<(i32, i32)> {
        &self.tail_visited
    }
}
