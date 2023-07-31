/*
Instructions
The Collatz Conjecture or 3x+1 problem can be summarized as follows:
Take any positive integer n. If n is even, divide n by 2 to get n / 2. If n is odd, multiply n by 3 and add 1 to get 3n + 1. Repeat the process indefinitely. The conjecture states that no matter which number you start with, you will always reach 1 eventually.
But sometimes the number grow significantly before it reaches 1. This can lead to an integer overflow and makes the algorithm unsolvable within the range of a number in u64.
Given a number n, return the number of steps required to reach 1.
Examples
Starting with n = 12, the steps would be as follows:
    12
    6
    3
    10
    5
    16
    8
    4
    2
    1
Resulting in 9 steps. So for input n = 12, the return value would be 9.
*/

pub fn collatz(n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }
    let mut num = n;
    let mut counter = 0;

    loop {
        if num == 1 {
            return Some(counter);
        } else if num % 2 != 0 {
            num = num.checked_mul(3)?.checked_add(1)?;
            counter = counter.checked_add(1)?;
        } else {
            num /= 2;
            counter = counter.checked_add(1)?;
        }
    }
}

#[cfg(test)]
mod collatz_conjecture_tes {
    use crate::collatz_conjecture::*;
    #[test]
    fn test_1() {
        assert_eq!(Some(0), collatz(1));
    }
    #[test]
    fn test_16() {
        assert_eq!(Some(4), collatz(16));
    }
    #[test]
    fn test_12() {
        assert_eq!(Some(9), collatz(12));
    }
    #[test]
    fn test_1000000() {
        assert_eq!(Some(152), collatz(1_000_000));
    }
    #[test]
    fn test_0() {
        assert_eq!(None, collatz(0));
    }
    #[test]
    fn test_110243094271() {
        let val = 110243094271;
        assert_eq!(None, collatz(val));
    }
    #[test]
    fn test_max_div_3() {
        let max = u64::MAX / 3;
        assert_eq!(None, collatz(max));
    }
    #[test]
    fn test_max_minus_1() {
        let max = u64::MAX - 1;
        assert_eq!(None, collatz(max));
    }
}
