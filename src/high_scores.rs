/*
Manage a game player's High Score list.
Your task is to build a high-score component of the classic Frogger game, one of the highest selling and addictive games of all time, and a classic of the arcade era. Your task is to write methods that return the highest score from the list, the last added score and the three highest scores.

Consider retaining a reference to scores in the struct - copying is not necessary. You will require some lifetime annotations, though.
 */
#[derive(Debug)]
pub struct HighScores {
    sc: Vec<u32>,
}

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        //unimplemented!("Construct a HighScores struct, given the scores: {scores:?}")
        HighScores {
            sc: scores.to_vec(),
        }
    }

    pub fn scores(&self) -> &[u32] {
        &self.sc
    }

    pub fn latest(&self) -> Option<u32> {
        let ret = self.sc.last()?;
        Some(*ret)
    }

    pub fn personal_best(&self) -> Option<u32> {
        let ret = self.sc.iter().max()?;
        Some(*ret)
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut aux = self.sc.clone();
        aux.sort();
        aux.reverse();
        let ret: Vec<u32> = aux.iter().map(|f| *f).take(3).collect();
        ret
    }
}

#[cfg(test)]
mod high_scores_tests {

    use crate::high_scores::HighScores;
    #[test]
    fn test_list_of_scores() {
        let expected = [30, 50, 20, 70];
        let high_scores = HighScores::new(&expected);
        assert_eq!(high_scores.scores(), &expected);
    }
    #[test]
    fn test_latest_score() {
        let high_scores = HighScores::new(&[100, 0, 90, 30]);
        assert_eq!(high_scores.latest(), Some(30));
    }
    #[test]
    fn test_latest_score_empty() {
        let high_scores = HighScores::new(&[]);
        assert_eq!(high_scores.latest(), None);
    }
    #[test]
    fn test_personal_best() {
        let high_scores = HighScores::new(&[40, 100, 70]);
        assert_eq!(high_scores.personal_best(), Some(100));
    }
    #[test]
    fn test_personal_best_empty() {
        let high_scores = HighScores::new(&[]);
        assert_eq!(high_scores.personal_best(), None);
    }
    #[test]
    fn test_personal_top_three() {
        let high_scores = HighScores::new(&[10, 30, 90, 30, 100, 20, 10, 0, 30, 40, 40, 70, 70]);
        assert_eq!(high_scores.personal_top_three(), vec![100, 90, 70]);
    }
    #[test]
    fn test_personal_top_three_highest_to_lowest() {
        let high_scores = HighScores::new(&[20, 10, 30]);
        assert_eq!(high_scores.personal_top_three(), vec![30, 20, 10]);
    }
    #[test]
    fn test_personal_top_three_with_tie() {
        let high_scores = HighScores::new(&[40, 20, 40, 30]);
        assert_eq!(high_scores.personal_top_three(), vec![40, 40, 30]);
    }
    #[test]
    fn test_personal_top_three_with_less_than_three_scores() {
        let high_scores = HighScores::new(&[30, 70]);
        assert_eq!(high_scores.personal_top_three(), vec![70, 30]);
    }
    #[test]
    fn test_personal_top_three_only_one_score() {
        let high_scores = HighScores::new(&[40]);
        assert_eq!(high_scores.personal_top_three(), vec![40]);
    }
    #[test]
    fn test_personal_top_three_empty() {
        let high_scores = HighScores::new(&[]);
        assert!(high_scores.personal_top_three().is_empty());
    }
}
