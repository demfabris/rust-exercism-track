// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Debug, Clone)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot {
    x: i32,
    y: i32,
    d: Direction,
}

impl Clone for Robot {
    fn clone(&self) -> Robot {
        Robot::new(self.x.clone(), self.y.clone(), self.d.clone())
    }
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Self { x, y, d }
    }

    pub fn turn_right(self) -> Self {
        Self {
            x: self.x,
            y: self.y,
            d: match self.d {
                Direction::East => Direction::South,
                Direction::South => Direction::West,
                Direction::West => Direction::North,
                Direction::North => Direction::East,
            },
        }
    }

    pub fn turn_left(self) -> Self {
        Self {
            x: self.x,
            y: self.y,
            d: match self.d {
                Direction::East => Direction::North,
                Direction::North => Direction::West,
                Direction::West => Direction::South,
                Direction::South => Direction::East,
            },
        }
    }

    pub fn advance(self) -> Self {
        let mut pos = (self.x, self.y);

        match self.d {
            Direction::East => pos.0 += 1,
            Direction::West => pos.0 -= 1,
            Direction::North => pos.1 += 1,
            Direction::South => pos.1 -= 1,
        }

        Self {
            x: pos.0,
            y: pos.1,
            d: self.d,
        }
    }

    pub fn instructions(self, instructions: &str) -> Self {
        let mut instance = self.clone();
        for i in instructions.chars() {
            instance = match i {
                'L' => instance.turn_left(),
                'R' => instance.turn_right(),
                'A' => instance.advance(),
                _ => instance,
            }
        }

        instance
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.d
    }
}
