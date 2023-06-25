/*
Bob only ever answers one of five things:
    "Sure." This is his response if you ask him a question, such as "How are you?" The convention used for questions is that it ends with a question mark.
    "Whoa, chill out!" This is his answer if you YELL AT HIM. The convention used for yelling is ALL CAPITAL LETTERS.
    "Calm down, I know what I'm doing!" This is what he says if you yell a question at him.
    "Fine. Be that way!" This is how he responds to silence. The convention used for silence is nothing, or various combinations of whitespace characters.
    "Whatever." This is what he answers to anything else.
 */
pub fn reply(message: &str) -> &str {
    let aux: Vec<char> = message.trim().chars().collect();

    let alphabetic = aux.iter().any(|f| f.is_alphabetic());
    let question = message.trim().ends_with('?');
    let yell = aux.iter().all(|f| {
        if f.is_alphabetic() {
            f.is_ascii_uppercase()
        } else {
            true
        }
    });
    let empty = message.trim().is_empty();

    match (empty, alphabetic, yell, question) {
        (true, _, _, _) => "Fine. Be that way!",
        (_, true, true, true) => "Calm down, I know what I'm doing!",
        (_, _, _, true) => "Sure.",
        (_, true, true, _) => "Whoa, chill out!",
        (_, _, _, _) => "Whatever.",
    }
}

#[cfg(test)]
mod bob_test {
    use crate::bob;
    fn process_response_case(phrase: &str, expected_response: &str) {
        assert_eq!(bob::reply(phrase), expected_response);
    }
    #[test]
    /// stating something
    fn test_stating_something() {
        process_response_case("Tom-ay-to, tom-aaaah-to.", "Whatever.");
    }
    #[test]
    /// ending with whitespace
    fn test_ending_with_whitespace() {
        process_response_case("Okay if like my  spacebar  quite a bit?   ", "Sure.");
    }
    #[test]
    /// shouting numbers
    fn test_shouting_numbers() {
        process_response_case("1, 2, 3 GO!", "Whoa, chill out!");
    }
    #[test]

    /// other whitespace
    fn test_other_whitespace() {
        process_response_case("\r\r 	", "Fine. Be that way!");
    }
    #[test]

    /// shouting with special characters
    fn test_shouting_with_special_characters() {
        process_response_case(
            "ZOMG THE %^*@#$(*^ ZOMBIES ARE COMING!!11!!1!",
            "Whoa, chill out!",
        );
    }
    #[test]

    /// talking forcefully
    fn test_talking_forcefully() {
        process_response_case("Hi there!", "Whatever.");
    }
    #[test]

    /// prattling on
    fn test_prattling_on() {
        process_response_case("Wait! Hang on. Are you going to be OK?", "Sure.");
    }
    #[test]

    /// forceful question
    fn test_forceful_question() {
        process_response_case("WHAT'S GOING ON?", "Calm down, I know what I'm doing!");
    }
    #[test]

    /// shouting with no exclamation mark
    fn test_shouting_with_no_exclamation_mark() {
        process_response_case("I HATE THE DENTIST", "Whoa, chill out!");
    }
    #[test]

    /// asking gibberish
    fn test_asking_gibberish() {
        process_response_case("fffbbcbeab?", "Sure.");
    }
    #[test]

    /// question with no letters
    fn test_question_with_no_letters() {
        process_response_case("4?", "Sure.");
    }
    #[test]

    /// no letters
    fn test_no_letters() {
        process_response_case("1, 2, 3", "Whatever.");
    }
    #[test]

    /// statement containing question mark
    fn test_statement_containing_question_mark() {
        process_response_case("Ending with ? means a question.", "Whatever.");
    }
    //NEW
    #[test]

    /// multiple line question
    fn test_multiple_line_question() {
        process_response_case(
            "\rDoes this cryogenic chamber make me look fat?\rNo.",
            "Whatever.",
        );
    }
    #[test]

    /// non-question ending with whitespace
    fn test_nonquestion_ending_with_whitespace() {
        process_response_case(
            "This is a statement ending with whitespace      ",
            "Whatever.",
        );
    }
    #[test]

    /// shouting
    fn test_shouting() {
        process_response_case("WATCH OUT!", "Whoa, chill out!");
    }
    #[test]

    /// non-letters with question
    fn test_nonletters_with_question() {
        process_response_case(":) ?", "Sure.");
    }
    #[test]

    /// shouting gibberish
    fn test_shouting_gibberish() {
        process_response_case("FCECDFCAAB", "Whoa, chill out!");
    }
    #[test]

    /// asking a question
    fn test_asking_a_question() {
        process_response_case("Does this cryogenic chamber make me look fat?", "Sure.");
    }
    #[test]

    /// asking a numeric question
    fn test_asking_a_numeric_question() {
        process_response_case("You are, what, like 15?", "Sure.");
    }
    #[test]

    /// silence
    fn test_silence() {
        process_response_case("", "Fine. Be that way!");
    }
    #[test]

    /// starting with whitespace
    fn test_starting_with_whitespace() {
        process_response_case("         hmmmmmmm...", "Whatever.");
    }
    #[test]

    /// using acronyms in regular speech
    fn test_using_acronyms_in_regular_speech() {
        process_response_case(
            "It's OK if you don't want to go work for NASA.",
            "Whatever.",
        );
    }
    #[test]

    /// alternate silence
    fn test_alternate_silence() {
        process_response_case("										", "Fine. Be that way!");
    }
    #[test]

    /// prolonged silence
    fn test_prolonged_silence() {
        process_response_case("          ", "Fine. Be that way!");
    }
}
