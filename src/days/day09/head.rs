pub struct Head {
    pub x: i32,
    pub y: i32,
}

impl Head {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn travel(&mut self, x: i8, y: i8) {
        self.x += x as i32;
        self.y += y as i32;
    }
}
