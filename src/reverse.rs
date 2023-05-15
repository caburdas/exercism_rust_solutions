/*
Reverse a string
For example: input: "cool" output: "looc"
Test your function on this string: uüu and see what happens.
 */
#[warn(dead_code)]
//mod reverse_string {
pub fn reverse(input: &str) -> String {
    input.chars().rev().collect()
}
//}

#[cfg(test)]
mod reverse_string_tests {

    // Process a single test case for the property `reverse`

    use crate::reverse::reverse;

    fn process_reverse_case(input: &str, expected: &str) {
        assert_eq!(&reverse(input), expected)
    }
    #[test]
    /// empty string
    fn test_an_empty_string() {
        process_reverse_case("", "");
    }

    #[test]
    /// a word
    fn test_a_word() {
        process_reverse_case("robot", "tobor");
    }

    #[test]
    /// a capitalized word
    fn test_a_capitalized_word() {
        process_reverse_case("Ramen", "nemaR");
    }

    #[test]
    /// a sentence with punctuation
    fn test_a_sentence_with_punctuation() {
        process_reverse_case("I'm hungry!", "!yrgnuh m'I");
    }
    #[test]
    /// a palindrome

    fn test_a_palindrome() {
        process_reverse_case("racecar", "racecar");
    }
    #[test]
    /// an even-sized word
    fn test_an_even_sized_word() {
        process_reverse_case("drawer", "reward");
    }

    #[test]
    /// wide characters
    fn test_wide_characters() {
        process_reverse_case("子猫", "猫子");
    }
}
