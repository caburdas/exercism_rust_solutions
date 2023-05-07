#[warn(dead_code)]

/*
The square of the sum of the first ten natural numbers is (1 + 2 + ... + 10)² = 55² = 3025.
The sum of the squares of the first ten natural numbers is 1² + 2² + ... + 10² = 385.
Hence the difference between the square of the sum of the first ten natural numbers and
the sum of the squares of the first ten natural numbers is 3025 - 385 = 2640.
 */
pub fn square_of_sum(n: u32) -> u32 {
    let x: u32 = (1..=n).sum();
    x * x
}

pub fn sum_of_squares(n: u32) -> u32 {
    let x: u32 = (1..=n).map(|x| x * x).sum();
    x
}

pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}

#[cfg(test)]
mod squarre_sum {
    use crate::square_of_sum::*;

    #[test]
    fn test_square_of_sum_1() {
        assert_eq!(1, square_of_sum(1));
    }

    #[test]
    fn test_square_of_sum_5() {
        assert_eq!(225, square_of_sum(5));
    }

    #[test]
    fn test_square_of_sum_100() {
        assert_eq!(25_502_500, square_of_sum(100));
    }

    #[test]
    fn test_sum_of_squares_1() {
        assert_eq!(1, sum_of_squares(1));
    }

    #[test]
    fn test_sum_of_squares_5() {
        assert_eq!(55, sum_of_squares(5));
    }

    #[test]

    fn test_sum_of_squares_100() {
        assert_eq!(338_350, sum_of_squares(100));
    }

    #[test]
    fn test_difference_1() {
        assert_eq!(0, difference(1));
    }

    #[test]
    fn test_difference_5() {
        assert_eq!(170, difference(5));
    }

    #[test]
    fn test_difference_100() {
        assert_eq!(25_164_150, difference(100));
    }
}
