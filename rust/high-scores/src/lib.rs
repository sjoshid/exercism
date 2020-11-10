use std::collections::BinaryHeap;

#[derive(Debug)]
pub struct HighScores<'a> {
    scores: &'a [u32],
}

impl <'a> HighScores<'a> {
    pub fn new(scores: &'a [u32]) -> Self {
        HighScores {
            scores
        }
    }

    pub fn scores(&self) -> &[u32] {
        self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        self.scores.last().and_then(|e| Some(*e))
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.scores.iter().max().and_then(|e| Some(*e))
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut max_heap: BinaryHeap<u32> = self.scores.iter().map(|e| *e).collect();

        let mut top_three = Vec::new();
        for _ in 0..3 {
            if let Some(v) = max_heap.pop() {
                top_three.push(v);
            }
        }
        top_three
    }
}
