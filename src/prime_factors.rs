/*
Compute the prime factors of a given natural number.
A prime number is only evenly divisible by itself and 1.
Note that 1 is not a prime number.
Example

What are the prime factors of 60?
The list of prime factors of 60: 2, 2, 3, and 5.
 */

pub fn factors(n: u64) -> Vec<u64> {
    let mut v = Vec::new();
    let mut num = n;
    let mut i =2; 
    while i <= num   {
        if num % i == 0 {
            v.push(i);
            num /=i;
        }else{
            i+=1;
        }
    }
    v
}

#[cfg(test)]
mod prime_factors_tests {

    use super::factors;

    #[test]

    fn test_no_factors() {
        assert_eq!(factors(1), vec![]);
    }

    #[test]
    fn test_prime_number() {
        assert_eq!(factors(2), vec![2]);
    }

    #[test]
    fn test_square_of_a_prime() {
        assert_eq!(factors(9), vec![3, 3]);
    }

    #[test]
    fn test_cube_of_a_prime() {
        assert_eq!(factors(8), vec![2, 2, 2]);
    }

    #[test]
    fn test_product_of_primes_and_non_primes() {
        assert_eq!(factors(12), vec![2, 2, 3]);
    }

    #[test]
    fn test_product_of_primes() {
        assert_eq!(factors(901_255), vec![5, 17, 23, 461]);
    }

    #[test]
    fn test_factors_include_large_prime() {
        assert_eq!(factors(93_819_012_551), vec![11, 9539, 894_119]);
    }
}
