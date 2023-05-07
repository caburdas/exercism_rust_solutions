/*
An Armstrong number is a number that is the sum of its own digits each raised to the 
power of the number of digits.
For example:
    9 is an Armstrong number, because 9 = 9^1 = 9
    10 is not an Armstrong number, because 10 != 1^2 + 0^2 = 1
    153 is an Armstrong number, because: 153 = 1^3 + 5^3 + 3^3 = 1 + 125 + 27 = 153
    154 is not an Armstrong number, because: 154 != 1^3 + 5^3 + 4^3 = 1 + 125 + 64 = 190
 */
#[warn(dead_code)]
#[warn(unused_assignments)]
pub fn is_armstrong_number(num: u32) -> bool {
    let s = num.to_string();
    let len = s.len();

    let mut sum: u32 = 0;
    for i in s.chars() {
        let aux: u32 = i.to_digit(10).unwrap();

        let mut j: u32 = 0;
        let aux2 = aux.checked_pow(len as u32);
        match aux2 {
            None => return false,
            Some(n) => j = n,
        }

        let aux3 = sum.checked_add(j);
        match aux3 {
            None => return false,
            Some(n) => sum = n,
        }
    }
    if sum == num {
        return true;
    }
    false
}

#[cfg(test)]
mod test {
    use crate::armstrong_numbers::*;

    #[test]

    fn test_zero_is_an_armstrong_number() {
        assert!(is_armstrong_number(0))
    }

    #[test]
    fn test_single_digit_numbers_are_armstrong_numbers() {
        assert!(is_armstrong_number(5))
    }

    #[test]
    fn test_there_are_no_2_digit_armstrong_numbers() {
        assert!(!is_armstrong_number(10))
    }

    #[test]
    fn test_three_digit_armstrong_number() {
        assert!(is_armstrong_number(153))
    }

    #[test]
    fn test_three_digit_non_armstrong_number() {
        assert!(!is_armstrong_number(100))
    }

    #[test]
    fn test_four_digit_armstrong_number() {
        assert!(is_armstrong_number(9474))
    }

    #[test]
    fn test_four_digit_non_armstrong_number() {
        assert!(!is_armstrong_number(9475))
    }

    #[test]
    fn test_seven_digit_armstrong_number() {
        assert!(is_armstrong_number(9_926_315))
    }

    #[test]
    fn test_seven_digit_non_armstrong_number() {
        assert!(!is_armstrong_number(9_926_316))
    }

    #[test]
    fn test_nine_digit_armstrong_number() {
        assert!(is_armstrong_number(912_985_153));
    }

    #[test]
    fn test_nine_digit_non_armstrong_number() {
        assert!(!is_armstrong_number(999_999_999));
    }

    #[test]
    fn test_ten_digit_non_armstrong_number() {
        assert!(!is_armstrong_number(3_999_999_999));
    }

    // The following number has an Armstrong sum equal to 2^32 plus itself,

    // and therefore will be detected as an Armstrong number if you are

    // incorrectly using wrapping arithmetic.

    #[test]
    fn test_properly_handles_overflow() {
        assert!(!is_armstrong_number(4_106_098_957));
    }
}
