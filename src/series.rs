/*
Instructions

Given a string of digits, output all the contiguous substrings of length n in that string in the order that they appear.
For example, the string "49142" has the following 3-digit series:
    "491"
    "914"
    "142"
And the following 4-digit series:
    "4914"
    "9142"
And if you ask for a 6-digit series from a 5-digit string, you deserve whatever you get.
Note that these series are only required to occupy adjacent positions in the input; the digits need not be numerically consecutive.
*/

pub fn series(digits: &str, len: usize) -> Vec<String> {
    if len == 0 {
        let mut res: Vec<String> = Vec::new();
        for _ in 0..digits.len() + 1 {
            res.push(String::from(""));
        }
        return res;
    } else if len > digits.len() {
        return vec![];
    }

    let chars: Vec<char> = digits.chars().collect();
    chars
        .windows(len)
        .map(|x| x.into_iter().collect())
        .collect()
}

#[cfg(test)]
mod series_tests {
    use crate::series::*;
    #[test]
    fn test_with_zero_length() {
        let expected = vec!["".to_string(); 6];
        assert_eq!(series("92017", 0), expected);
    }
    #[test]
    fn test_with_length_2() {
        let expected = vec![
            "92".to_string(),
            "20".to_string(),
            "01".to_string(),
            "17".to_string(),
        ];
        assert_eq!(series("92017", 2), expected);
    }
    #[test]
    fn test_with_numbers_length() {
        let expected = vec!["92017".to_string()];
        assert_eq!(series("92017", 5), expected);
    }
    #[test]
    fn test_too_long() {
        let expected: Vec<String> = vec![];
        assert_eq!(series("92017", 6), expected);
    }
    #[test]
    fn test_way_too_long() {
        let expected: Vec<String> = vec![];
        assert_eq!(series("92017", 42), expected);
    }
}
