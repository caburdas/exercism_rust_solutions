/*
Your task is to convert a number into a string that contains raindrop sounds corresponding to certain potential factors. A factor is a number that evenly divides into another number, leaving no remainder. The simplest way to test if a one number is a factor of another is to use the modulo operation.
The rules of raindrops are that if a given number:
    has 3 as a factor, add 'Pling' to the result.
    has 5 as a factor, add 'Plang' to the result.
    has 7 as a factor, add 'Plong' to the result.
    does not have any of 3, 5, or 7 as a factor, the result should be the digits of the number.
Examples
    28 has 7 as a factor, but not 3 or 5, so the result would be "Plong".
    30 has both 3 and 5 as factors, but not 7, so the result would be "PlingPlang".
    34 is not factored by 3, 5, or 7, so the result would be "34".
*/
pub mod raindrops {
    pub fn raindrops(n: u32) -> String {
        let mut ret = String::new();
        if n % 3_u32 == 0 {
            ret.push_str("Pling");
        }
        if n % 5_u32 == 0 {
            ret.push_str("Plang");
        }
        if n % 7_u32 == 0 {
            ret.push_str("Plong");
        }

        if ret.is_empty() {
            ret = n.to_string();
        }
        ret
    }
}
#[cfg(test)]
mod raindrops_test {

    use super::raindrops;
    #[test]
    fn test_1() {
        assert_eq!("1", raindrops::raindrops(1));
    }

    #[test]
    fn test_3() {
        assert_eq!("Pling", raindrops::raindrops(3));
    }

    #[test]
    fn test_5() {
        assert_eq!("Plang", raindrops::raindrops(5));
    }

    #[test]
    fn test_7() {
        assert_eq!("Plong", raindrops::raindrops(7));
    }

    #[test]
    fn test_6() {
        assert_eq!("Pling", raindrops::raindrops(6));
    }

    #[test]
    fn test_8() {
        assert_eq!("8", raindrops::raindrops(8));
    }

    #[test]
    fn test_9() {
        assert_eq!("Pling", raindrops::raindrops(9));
    }

    #[test]
    fn test_10() {
        assert_eq!("Plang", raindrops::raindrops(10));
    }

    #[test]
    fn test_14() {
        assert_eq!("Plong", raindrops::raindrops(14));
    }

    #[test]
    fn test_15() {
        assert_eq!("PlingPlang", raindrops::raindrops(15));
    }

    #[test]
    fn test_21() {
        assert_eq!("PlingPlong", raindrops::raindrops(21));
    }

    #[test]
    fn test_25() {
        assert_eq!("Plang", raindrops::raindrops(25));
    }

    #[test]
    fn test_27() {
        assert_eq!("Pling", raindrops::raindrops(27));
    }

    #[test]
    fn test_35() {
        assert_eq!("PlangPlong", raindrops::raindrops(35));
    }

    #[test]
    fn test_49() {
        assert_eq!("Plong", raindrops::raindrops(49));
    }

    #[test]
    fn test_52() {
        assert_eq!("52", raindrops::raindrops(52));
    }

    #[test]
    fn test_105() {
        assert_eq!("PlingPlangPlong", raindrops::raindrops(105));
    }

    #[test]
    fn test_3125() {
        assert_eq!("Plang", raindrops::raindrops(3125));
    }

    #[test]
    fn test_12121() {
        assert_eq!("12121", raindrops::raindrops(12_121));
    }
}
