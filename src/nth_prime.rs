/*
Given a number n, determine what the nth prime is.
By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.
If your language provides methods in the standard library to deal with prime numbers, pretend they don't exist and implement them yourself.
Remember that while people commonly count with 1-based indexing (i.e. "the 6th prime is 13"), many programming languages, including Rust, use 0-based indexing (i.e. primes[5] == 13). Use 0-based indexing for your implementation.
 */
#[warn(dead_code)]
mod nth_prime {
    use std::{thread::sleep, time::Duration};

    pub fn nth(n: u32) -> u32 {
        let mut i = 2;
        let mut j = 0;
        let mut z = 0;
        loop {
            let mut prime = true;
            j = (i as f64).sqrt() as u32;
            while j > 1 {
                if i % j == 0 {
                    prime = false;
                    break;
                }
                j -= 1;
            }
            if prime {
                if z == n{
                    return i;
                }
                z +=1;
            }
            i += 1;
        }
    }
    // pub fn nth(n: u32) -> u32 {
    //      let mut primes: Vec<u32> = vec![];
    //
    //     (2..)
    //         .filter(|candidate: &u32| {
    //             if !primes.iter().any(|i| candidate % i == 0) {
    //                 primes.push(*candidate);
    //                 true
    //             } else {
    //                 false
    //             }
    //         })
    //         .nth(n as usize)
    //         .unwrap()
    // }

     // fn is_prime(number: u32) -> bool {
     //     !(2..(number as f32).sqrt() as u32 + 1).any(|x| number % x == 0)
     // }
     //
     // pub fn nth(n: u32) -> u32 {
     //     (2..).filter(|x| is_prime(*x)).nth(n as usize).unwrap()
     // }
}

#[cfg(test)]
mod nth_prime_test {
    use crate::nth_prime::nth_prime as np;
    #[test]
    fn test_first_prime() {
        assert_eq!(np::nth(0), 2);
    }

    #[test]
    fn test_second_prime() {
        assert_eq!(np::nth(1), 3);
    }

    #[test]
    fn test_sixth_prime() {
        assert_eq!(np::nth(5), 13);
    }

    #[test]
    fn test_big_prime() {
        assert_eq!(np::nth(10_000), 104_743);
    }
}
