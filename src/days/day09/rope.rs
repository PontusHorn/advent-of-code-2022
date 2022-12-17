use super::{head::Head, tail::Tail};
use std::collections::HashSet;

pub struct Rope {
    head: Head,
    tail: Tail,
}

impl Rope {
    pub fn new(x: i32, y: i32) -> Self {
        let head = Head::new(x, y);
        let tail = Tail::new(x, y);
        Self { head, tail }
    }

    pub fn travel(&mut self, x: i8, y: i8) {
        self.head.travel(x, y);
        self.tail.follow(&self.head);
    }

    pub fn tail_visited(&self) -> &HashSet<(i32, i32)> {
        self.tail.visited()
    }
}
