pub struct ChessPosition {
    x: i8,
    y: i8,
}

impl ChessPosition {
    pub fn new(x: i8, y: i8) -> Result<Self, &'static str> {
        let cp = ChessPosition { x: x, y: y };

        if cp.is_valid() {
            Ok(cp)
        } else {
            Err("Invalid Position")
        }
    }

    fn is_valid(&self) -> bool {
        self.x >= 0 && self.x <= 7 && self.y >= 0 && self.y <= 7
    }
}

pub struct Queen(ChessPosition);

impl Queen {
    pub fn new(c: ChessPosition) -> Self { Queen(c) }

    pub fn can_attack(&self, other: &Queen) -> bool {
        self.same_column(other) ||
            self.same_row(other) ||
            self.same_diagonal(other)
    }

    fn same_column(&self, other: &Queen) -> bool {
        self.0.x == other.0.x
    }

    fn same_row(&self, other: &Queen) -> bool {
        self.0.y == other.0.y
    }

    fn same_diagonal(&self, other: &Queen) -> bool {
        self.0.x + self.0.y == other.0.x + other.0.y ||
            self.0.x - self.0.y == other.0.x - other.0.y
    }
}
