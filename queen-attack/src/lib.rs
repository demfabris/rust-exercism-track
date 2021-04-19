#[derive(Debug)]
pub struct ChessPosition {
    rank: i32,
    file: i32,
}

#[derive(Debug)]
pub struct Queen {
    position: ChessPosition,
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        let valid_range = 0..8;

        if valid_range.contains(&rank) && valid_range.contains(&file) {
            return Some(Self { rank, file });
        } else {
            None
        }
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Self { position }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        let delta_x = i32::abs(self.position.rank - other.position.rank);
        let delta_y = i32::abs(self.position.file - other.position.file);
        let range = i32::abs(delta_x - delta_y);

        if delta_x == 0 {
            true
        } else if delta_y == 0 {
            true
        } else if range == 0 {
            true
        } else {
            false
        }
    }
}
