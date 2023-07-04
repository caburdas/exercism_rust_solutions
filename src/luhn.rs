/*
Check a Luhn checksum.
Instructions
Given a number determine whether or not it is valid per the Luhn formula.
The Luhn algorithm is a simple checksum formula used to validate a variety of identification numbers, such as credit card numbers and Canadian Social Insurance Numbers.
The task is to check if a given string is valid.
Validating a Number
Strings of length 1 or less are not valid. Spaces are allowed in the input, but they should be stripped before checking. All other non-digit characters are disallowed.
Example 1: valid credit card number
4539 3195 0343 6467
The first step of the Luhn algorithm is to double every second digit, starting from the right. We will be doubling
4_3_ 3_9_ 0_4_ 6_6_
If doubling the number results in a number greater than 9 then subtract 9 from the product. The results of our doubling:
8569 6195 0383 3437
Then sum all of the digits:
8+5+6+9+6+1+9+5+0+3+8+3+3+4+3+7 = 80
If the sum is evenly divisible by 10, then the number is valid. This number is valid!
Example 2: invalid credit card number
8273 1232 7352 0569
Double the second digits, starting from the right
7253 2262 5312 0539
Sum the digits
7+2+5+3+2+2+6+2+5+3+1+2+0+5+3+9 = 57
57 is not evenly divisible by 10, so this number is not valid.
 */

pub fn is_valid(code: &str) -> bool {
    let aux = code.replace(' ', "");
    if aux.len() <= 1 {
        return false;
    }

    let mut count = 0;
    for (i, c) in aux.chars().rev().enumerate() {
        if c.is_ascii_digit() {
            let mut n = c.to_digit(10).unwrap();
            if (i + 1) % 2 == 0 {
                n = n * 2;
                n = if n > 9 { n - 9 } else { n };
            }
            count += n;
        } else {
            return false;
        }
    }
    count % 10 == 0
}

#[cfg(test)]
mod luhn_tests {
    use crate::luhn::*;
    fn process_valid_case(number: &str, is_luhn_expected: bool) {
        assert_eq!(is_valid(number), is_luhn_expected);
    }
    #[test]
    fn test_single_digit_strings_can_not_be_valid() {
        process_valid_case("1", false);
    }
    #[test]
    fn test_a_single_zero_is_invalid() {
        process_valid_case("0", false);
    }
    #[test]
    fn test_a_simple_valid_sin_that_remains_valid_if_reversed() {
        process_valid_case("059", true);
    }
    #[test]
    fn test_a_simple_valid_sin_that_becomes_invalid_if_reversed() {
        process_valid_case("59", true);
    }
    #[test]
    fn test_a_valid_canadian_sin() {
        process_valid_case("055 444 285", true);
    }
    #[test]
    fn test_invalid_canadian_sin() {
        process_valid_case("055 444 286", false);
    }
    #[test]
    fn test_invalid_credit_card() {
        process_valid_case("8273 1232 7352 0569", false);
    }
    #[test]

    fn test_valid_number_with_an_even_number_of_digits() {
        process_valid_case("095 245 88", true);
    }
    #[test]

    fn strings_that_contain_non_digits_are_invalid() {
        process_valid_case("055a 444 285", false);
    }
    #[test]

    fn test_valid_strings_with_punctuation_included_become_invalid() {
        process_valid_case("055-444-285", false);
    }
    #[test]

    fn symbols_are_invalid() {
        process_valid_case("055£ 444$ 285", false);
    }
    #[test]

    fn test_single_zero_with_space_is_invalid() {
        process_valid_case(" 0", false);
    }
    #[test]

    fn test_more_than_a_single_zero_is_valid() {
        process_valid_case("0000 0", true);
    }
    #[test]

    fn test_input_digit_9_is_correctly_converted_to_output_digit_9() {
        process_valid_case("091", true);
    }
    #[test]

    /// using ASCII value for doubled non-digit isn't allowed
    /// Convert non-digits to their ASCII values and then offset them by 48 sometimes accidentally declare an invalid string to be valid.
    /// This test is designed to avoid that solution.
    fn test_using_ascii_value_for_doubled_nondigit_isnt_allowed() {
        process_valid_case(":9", false);
    }
    #[test]

    /// valid strings with a non-digit added at the end become invalid
    fn test_valid_strings_with_a_nondigit_added_at_the_end_become_invalid() {
        process_valid_case("059a", false);
    }
    #[test]

    /// valid strings with symbols included become invalid
    fn test_valid_strings_with_symbols_included_become_invalid() {
        process_valid_case("055# 444$ 285", false);
    }
    #[test]

    /// using ASCII value for non-doubled non-digit isn't allowed
    /// Convert non-digits to their ASCII values and then offset them by 48 sometimes accidentally declare an invalid string to be valid.
    /// This test is designed to avoid that solution.
    fn test_using_ascii_value_for_nondoubled_nondigit_isnt_allowed() {
        process_valid_case("055b 444 285", false);
    }
    #[test]

    /// valid number with an odd number of spaces
    fn test_valid_number_with_an_odd_number_of_spaces() {
        process_valid_case("234 567 891 234", true);
    }
    #[test]

    /// non-numeric, non-space char in the middle with a sum that's divisible by 10 isn't allowed
    fn test_invalid_char_in_middle_with_sum_divisible_by_10_isnt_allowed() {
        process_valid_case("59%59", false);
    }
    #[test]

    /// unicode numeric characters are not allowed in a otherwise valid number
    fn test_valid_strings_with_numeric_unicode_characters_become_invalid() {
        process_valid_case("1249①", false);
    }
}
