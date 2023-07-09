/*
Instructions
Convert a phrase to its acronym.
Techies love their TLA (Three Letter Acronyms)!
Help generate some jargon by writing a program that converts a long name like Portable Network Graphics to its acronym (PNG).
 */
pub fn abbreviate(phrase: &str) -> String {
    let my_phrase = phrase.replace("-", " ");
    let my_phrase = my_phrase.replace("_", " ");

    let words_iterator = my_phrase.split_ascii_whitespace();
    let w: Vec<String> = words_iterator
        .map(|f| {
            if f.to_uppercase() == f {
                f.to_lowercase().to_string()
            } else {
                f.to_string()
            }
        })
        .collect();

    let mut ret = "".to_string();
    for data in w {
        for (pos, c) in data.as_str().chars().enumerate() {
            if pos == 0 {
                ret.push(c.to_ascii_uppercase())
            } else if c.is_ascii_uppercase() {
                ret.push(c.to_ascii_uppercase())
            }
        }
    }
    ret
}

#[cfg(test)]
mod acronym_tests {
    use crate::acronym;
    #[test]
    fn empty() {
        assert_eq!(acronym::abbreviate(""), "");
    }
    #[test]

    fn basic() {
        assert_eq!(acronym::abbreviate("Portable Network Graphics"), "PNG");
    }
    #[test]

    fn lowercase_words() {
        assert_eq!(acronym::abbreviate("Ruby on Rails"), "ROR");
    }
    #[test]

    fn camelcase() {
        assert_eq!(acronym::abbreviate("HyperText Markup Language"), "HTML");
    }
    #[test]

    fn punctuation() {
        assert_eq!(acronym::abbreviate("First In, First Out"), "FIFO");
    }
    #[test]

    fn all_caps_word() {
        assert_eq!(
            acronym::abbreviate("GNU Image Manipulation Program"),
            "GIMP"
        );
    }
    #[test]

    fn all_caps_word_with_punctuation() {
        assert_eq!(acronym::abbreviate("PHP: Hypertext Preprocessor"), "PHP");
    }
    #[test]

    fn punctuation_without_whitespace() {
        assert_eq!(
            acronym::abbreviate("Complementary metal-oxide semiconductor"),
            "CMOS"
        );
    }
    #[test]

    fn very_long_abbreviation() {
        assert_eq!(
            acronym::abbreviate(
                "Rolling On The Floor Laughing So Hard That My Dogs Came Over And Licked Me"
            ),
            "ROTFLSHTMDCOALM"
        );
    }
    #[test]

    fn consecutive_delimiters() {
        assert_eq!(
            acronym::abbreviate("Something - I made up from thin air"),
            "SIMUFTA"
        );
    }
    #[test]

    fn apostrophes() {
        assert_eq!(acronym::abbreviate("Halley's Comet"), "HC");
    }
    #[test]

    fn underscore_emphasis() {
        assert_eq!(acronym::abbreviate("The Road _Not_ Taken"), "TRNT");
    }
}
