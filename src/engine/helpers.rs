

pub struct Position {
    x: i64,
    y: i64
} impl Position {
    pub fn getX(&self) {
        self.x
    }

    pub fn getY(&self) {
        self.y
    }
}

pub struct ColorChar {
    text: char,
    color: Vec<i8; 3>
} impl ColorChar {

    fn default() -> ColorChar {
        self {
            text: " ",
            color: [0, 0, 0]
        }
    }
}