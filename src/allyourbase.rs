/*
Instructions
Convert a number, represented as a sequence of digits in one base, to any other base.
Implement general base conversion. Given a number in base a, represented as a sequence of digits, convert it to base b.
Note
    Try to implement the conversion yourself. Do not use something else to perform the conversion for you.
About Positional Notation
In positional notation, a number in base b can be understood as a linear combination of powers of b.
The number 42, in base 10, means:
(4 * 10^1) + (2 * 10^0)
The number 101010, in base 2, means:
(1 * 2^5) + (0 * 2^4) + (1 * 2^3) + (0 * 2^2) + (1 * 2^1) + (0 * 2^0)
The number 1120, in base 3, means:
(1 * 3^3) + (1 * 3^2) + (2 * 3^1) + (0 * 3^0)
I think you got the idea!
Yes. Those three numbers above are exactly the same. Congratulations!
 */
#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

///
/// Convert a number between two bases.
///
/// A number is any slice of digits.
/// A digit is any unsigned integer (e.g. u8, u16, u32, u64, or usize).
/// Bases are specified as unsigned integers.
///
/// Return an `Err(.)` if the conversion is impossible.
/// The tests do not test for specific values inside the `Err(.)`.
///
///
/// You are allowed to change the function signature as long as all test still pass.
///
///
/// Example:
/// Input
///   number: &[4, 2]
///   from_base: 10
///   to_base: 2
/// Result
///   Ok(vec![1, 0, 1, 0, 1, 0])
///
/// The example corresponds to converting the number 42 from decimal
/// which is equivalent to 101010 in binary.
///
///
/// Notes:
///  * The empty slice ( "[]" ) is equal to the number 0.
///  * Never output leading 0 digits, unless the input number is 0, in which the output must be `[0]`.
///    However, your function must be able to process input with leading 0 digits.
///
pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    if to_base <= 1 {
        return Err(Error::InvalidOutputBase);
    }
    if from_base <= 1 {
        return Err(Error::InvalidInputBase);
    }
    if number.len() == 0 {
        return Ok(vec![0]);
    }

    let mut res: Vec<u32> = Vec::new();
    let mut value = 0;

    let numbers = number.into_iter().rev();
    for (pos, n) in numbers.enumerate() {
        if *n >= from_base {
            return Err(Error::InvalidDigit(*n));
        }
        value += *n * from_base.pow(pos as u32);
    }

    loop {
        let digit = value % to_base;
        res.push(digit);
        let r = value / to_base;
        if r < to_base {
            if r != 0 {
                res.push(r);
            }
            break;
        }
        value = r;
    }
    res.reverse();
    Ok(res)
}

#[cfg(test)]
mod allyourbase_tests {
    use crate::allyourbase as ayb;
    #[test]
    fn single_bit_one_to_decimal() {
        let input_base = 2;
        let input_digits = &[1];
        let output_base = 10;
        let output_digits = vec![1];
        assert_eq!(
            ayb::convert(input_digits, input_base, output_base),
            Ok(output_digits)
        );
    }
    #[test]
    fn binary_to_single_decimal() {
        let input_base = 2;
        let input_digits = &[1, 0, 1];
        let output_base = 10;
        let output_digits = vec![5];
        assert_eq!(
            ayb::convert(input_digits, input_base, output_base),
            Ok(output_digits)
        );
    }
    #[test]
    fn single_decimal_to_binary() {
        let input_base = 10;
        let input_digits = &[5];
        let output_base = 2;
        let output_digits = vec![1, 0, 1];
        assert_eq!(
            ayb::convert(input_digits, input_base, output_base),
            Ok(output_digits)
        );
    }
    #[test]
    fn binary_to_multiple_decimal() {
        let input_base = 2;
        let input_digits = &[1, 0, 1, 0, 1, 0];
        let output_base = 10;
        let output_digits = vec![4, 2];
        assert_eq!(
            ayb::convert(input_digits, input_base, output_base),
            Ok(output_digits)
        );
    }
    #[test]
    fn decimal_to_binary() {
        let input_base = 10;
        let input_digits = &[4, 2];
        let output_base = 2;
        let output_digits = vec![1, 0, 1, 0, 1, 0];
        assert_eq!(
            ayb::convert(input_digits, input_base, output_base),
            Ok(output_digits)
        );
    }
    #[test]
    fn trinary_to_hexadecimal() {
        let input_base = 3;
        let input_digits = &[1, 1, 2, 0];
        let output_base = 16;
        let output_digits = vec![2, 10];
        assert_eq!(
            ayb::convert(input_digits, input_base, output_base),
            Ok(output_digits)
        );
    }
    #[test]
    fn hexadecimal_to_trinary() {
        let input_base = 16;
        let input_digits = &[2, 10];
        let output_base = 3;
        let output_digits = vec![1, 1, 2, 0];
        assert_eq!(
            ayb::convert(input_digits, input_base, output_base),
            Ok(output_digits)
        );
    }
    #[test]
    fn fifteen_bit_integer() {
        let input_base = 97;
        let input_digits = &[3, 46, 60];
        let output_base = 73;
        let output_digits = vec![6, 10, 45];
        assert_eq!(
            ayb::convert(input_digits, input_base, output_base),
            Ok(output_digits)
        );
    }
    #[test]
    fn empty_list() {
        let input_base = 2;
        let input_digits = &[];
        let output_base = 10;
        let output_digits = vec![0];
        assert_eq!(
            ayb::convert(input_digits, input_base, output_base),
            Ok(output_digits)
        );
    }
    #[test]
    fn single_zero() {
        let input_base = 10;
        let input_digits = &[0];
        let output_base = 2;
        let output_digits = vec![0];
        assert_eq!(
            ayb::convert(input_digits, input_base, output_base),
            Ok(output_digits)
        );
    }
    #[test]
    fn multiple_zeros() {
        let input_base = 10;
        let input_digits = &[0, 0, 0];
        let output_base = 2;
        let output_digits = vec![0];
        assert_eq!(
            ayb::convert(input_digits, input_base, output_base),
            Ok(output_digits)
        );
    }
    #[test]
    fn leading_zeros() {
        let input_base = 7;
        let input_digits = &[0, 6, 0];
        let output_base = 10;
        let output_digits = vec![4, 2];
        assert_eq!(
            ayb::convert(input_digits, input_base, output_base),
            Ok(output_digits)
        );
    }
    #[test]
    fn invalid_positive_digit() {
        let input_base = 2;
        let input_digits = &[1, 2, 1, 0, 1, 0];
        let output_base = 10;
        assert_eq!(
            ayb::convert(input_digits, input_base, output_base),
            Err(ayb::Error::InvalidDigit(2))
        );
    }
    #[test]
    fn input_base_is_one() {
        let input_base = 1;
        let input_digits = &[];
        let output_base = 10;
        assert_eq!(
            ayb::convert(input_digits, input_base, output_base),
            Err(ayb::Error::InvalidInputBase)
        );
    }
    #[test]
    fn output_base_is_one() {
        let input_base = 2;
        let input_digits = &[1, 0, 1, 0, 1, 0];
        let output_base = 1;
        assert_eq!(
            ayb::convert(input_digits, input_base, output_base),
            Err(ayb::Error::InvalidOutputBase)
        );
    }
    #[test]
    fn input_base_is_zero() {
        let input_base = 0;
        let input_digits = &[];
        let output_base = 10;
        assert_eq!(
            ayb::convert(input_digits, input_base, output_base),
            Err(ayb::Error::InvalidInputBase)
        );
    }
    #[test]
    fn output_base_is_zero() {
        let input_base = 10;
        let input_digits = &[7];
        let output_base = 0;
        assert_eq!(
            ayb::convert(input_digits, input_base, output_base),
            Err(ayb::Error::InvalidOutputBase)
        );
    }
}
