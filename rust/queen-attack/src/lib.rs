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
        if (0..8).contains(&rank) && (0..8).contains(&file) {
            Some(ChessPosition {rank, file})
        } else {
            None
        }
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Queen {
            position
        }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        let (self_rank, self_file) = (&self.position.rank, &self.position.file);
        let (other_rank, other_file) = (&other.position.rank, &other.position.file);

        match (self_rank == other_rank, self_file == other_file) {
            (true, _) => true,
            (_, true) => true,
            (false, false) => {
                let diff_file = (self_file - other_file).abs();
                let diff_rank = (self_rank - other_rank).abs();

                diff_file == diff_rank
            }
        }
    }
}
