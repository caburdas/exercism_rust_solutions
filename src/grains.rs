/*
Calculate the number of grains of wheat on a chessboard given that the number on each square doubles.
There once was a wise servant who saved the life of a prince. The king promised to pay whatever the servant could dream up. Knowing that the king loved chess, the servant told the king he would like to have grains of wheat. One grain on the first square of a chess board, with the number of grains doubling on each successive square.
There are 64 squares on a chessboard (where square 1 has one grain, square 2 has two grains, and so on).
Write code that shows:

    how many grains were on a given square,
    the total number of grains on the chessboard, and
    panics with a message of "Square must be between 1 and 64" if the value is not valid

 */
pub fn square(s: u32) -> u64 {
    if s == 0 || s > 64 {
        panic!("Square must be between 1 and 64")
    }
    2_u64.pow(s - 1)
}

pub fn total() -> u64 {
    (0..64_u32).map(|x| 2_u64.pow(x)).sum()
}

#[cfg(test)]
mod grains_test {
    use crate::grains;

    fn process_square_case(input: u32, expected: u64) {
        assert_eq!(grains::square(input), expected);
    }

    #[test]
    /// 1
    fn test_1() {
        process_square_case(1, 1);
    }

    #[test]
    /// 2
    fn test_2() {
        process_square_case(2, 2);
    }

    #[test]
    /// 3
    fn test_3() {
        process_square_case(3, 4);
    }

    #[test]
    /// 4
    fn test_4() {
        process_square_case(4, 8);
    }

    //NEW
    #[test]
    /// 16
    fn test_16() {
        process_square_case(16, 32_768);
    }

    #[test]
    /// 32
    fn test_32() {
        process_square_case(32, 2_147_483_648);
    }

    #[test]
    /// 64
    fn test_64() {
        process_square_case(64, 9_223_372_036_854_775_808);
    }

    #[test]
    #[should_panic(expected = "Square must be between 1 and 64")]

    fn test_square_0_raises_an_exception() {
        grains::square(0);
    }

    #[test]
    #[should_panic(expected = "Square must be between 1 and 64")]
    fn test_square_greater_than_64_raises_an_exception() {
        grains::square(65);
    }

    #[test]
    fn test_returns_the_total_number_of_grains_on_the_board() {
        assert_eq!(grains::total(), 18_446_744_073_709_551_615);
    }
}
