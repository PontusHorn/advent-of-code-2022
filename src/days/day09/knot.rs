pub struct Knot {
    x: i32,
    y: i32,
}

impl Knot {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn travel(&mut self, x: i8, y: i8) {
        self.x += x as i32;
        self.y += y as i32;
    }

    pub fn follow(&mut self, knot: &Knot) {
        if self.is_touching(knot) {
            return;
        }

        let (knot_x, knot_y) = knot.position();
        let (x, y) = (knot_x - self.x, knot_y - self.y);
        self.x += x.signum();
        self.y += y.signum();
    }

    fn is_touching(&self, knot: &Knot) -> bool {
        let (x, y) = knot.position();
        (x - 1..=x + 1).contains(&self.x) && (y - 1..=y + 1).contains(&self.y)
    }
}
