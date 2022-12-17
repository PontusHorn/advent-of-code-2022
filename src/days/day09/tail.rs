use super::head::Head;
use std::collections::HashSet;

pub struct Tail {
    pub x: i32,
    pub y: i32,
    visited: HashSet<(i32, i32)>,
}

impl Tail {
    pub fn new(x: i32, y: i32) -> Self {
        let mut visited = HashSet::new();
        visited.insert((x, y));
        Self { x, y, visited }
    }

    pub fn follow(&mut self, head: &Head) {
        if self.is_touching(head) {
            return;
        }

        let (x, y) = (head.x - self.x, head.y - self.y);
        self.x += x.signum();
        self.y += y.signum();

        self.visited.insert((self.x, self.y));
    }

    fn is_touching(&self, head: &Head) -> bool {
        (head.x - 1..=head.x + 1).contains(&self.x) && (head.y - 1..=head.y + 1).contains(&self.y)
    }

    pub fn visited(&self) -> &HashSet<(i32, i32)> {
        &self.visited
    }
}
