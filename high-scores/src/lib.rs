#[derive(Debug)]
pub struct HighScores {
    scores: Vec<u32>,
    scores_sorted: Vec<u32>,
}

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        let mut sorted: Vec<u32> = Vec::from(scores);
        sorted.sort();
        HighScores {
            scores: Vec::from(scores),
            scores_sorted: Vec::from(sorted),
        }
    }

    pub fn scores(&self) -> &[u32] {
        &self.scores.as_slice()
    }

    pub fn latest(&self) -> Option<u32> {
        self.scores().last().cloned()
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.scores_sorted.last().cloned()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        self.scores_sorted.clone().into_iter().rev().take(3).collect::<Vec<u32>>()
    }
}
