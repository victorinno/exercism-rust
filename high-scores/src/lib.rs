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

    pub fn reverse_ordered_list(&self) -> Vec<u32> {
        let mut v: Vec<u32> = self.scores_sorted.clone();
        v.reverse();
        v
    }

    pub fn reverse_ordered_list_from_vec(original: &Vec<u32>) -> Vec<u32> {
        let mut v: Vec<u32> = original.clone();
        v.reverse();
        v
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        match self.scores().len() {
            0 => Vec::new(),
            _x if _x >= 1 && _x <= 3 => HighScores::reverse_ordered_list(self),
            x => HighScores::reverse_ordered_list_from_vec(self.scores_sorted[(x - 3)..].to_vec().as_ref()),
        }
    }
}
