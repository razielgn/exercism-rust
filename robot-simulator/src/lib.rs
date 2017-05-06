// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot {
    pos: (isize, isize),
    dir: Direction,
}

impl Robot {
    pub fn new(x: isize, y: isize, d: Direction) -> Self {
        Robot {
            pos: (x, y),
            dir: d,
        }
    }

    pub fn turn_right(mut self) -> Self {
        use Direction::*;

        self.dir = match self.dir {
            North => East,
            East => South,
            South => West,
            West => North,
        };

        self
    }

    pub fn turn_left(mut self) -> Self {
        use Direction::*;

        self.dir = match self.dir {
            North => West,
            East => North,
            South => East,
            West => South,
        };

        self
    }

    pub fn advance(mut self) -> Self {
        use Direction::*;

        let inc = match self.dir {
            North => (0, 1),
            East => (1, 0),
            South => (0, -1),
            West => (-1, 0),
        };

        self.pos = (self.pos.0 + inc.0, self.pos.1 + inc.1);

        self
    }

    pub fn instructions(self, instructions: &str) -> Self {
        instructions.chars()
            .fold(self, |robot, c| {
                match c {
                    'L' => robot.turn_left(),
                    'A' => robot.advance(),
                    'R' => robot.turn_right(),
                    _ => robot,
                }
            })
    }

    pub fn position(&self) -> (isize, isize) { self.pos }

    pub fn direction(&self) -> &Direction { &self.dir }
}
