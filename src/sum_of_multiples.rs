/*
Your task is to write the code that calculates the energy points that get awarded to players when they complete a level.
The points awarded depend on two things:
    The level (a number) that the player completed.
    The base value of each magical item collected by the player during that level.
The energy points are awarded according to the following rules:
    For each magical item, take the base value and find all the multiples of that value that are less than the level number.
    Combine the sets of numbers.
    Remove any duplicates.
    Calculate the sum of all the numbers that are left.
Let's look at an example:
The player completed level 20 and found two magical items with base values of 3 and 5.
To calculate the energy points earned by the player, we need to find all the unique multiples of these base values that are less than level 20.
    Multiples of 3 less than 20: {3, 6, 9, 12, 15, 18}
    Multiples of 5 less than 20: {5, 10, 15}
    Combine the sets and remove duplicates: {3, 5, 6, 9, 10, 12, 15, 18}
    Sum the unique multiples: 3 + 5 + 6 + 9 + 10 + 12 + 15 + 18 = 78
    Therefore, the player earns 78 energy points for completing level 20 and finding the two magical items with base values of 3 and 5.
 */
use std::collections::{ HashSet, btree_map::Values};
    
pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    
    let mut hs = HashSet::new();   
    for i in factors{
        let mut cont = 1;
        let mut value = cont * i;
        while value < limit && value != 0{
            hs.insert(value);
            cont +=1;
            value = cont * i;
        }
    }
    
    hs.iter()
            .fold(0, |acc, x| acc + x)
}

#[cfg(test)]
mod  sum_of_multiples_test {

    use crate::sum_of_multiples::sum_of_multiples;

    #[test]
    fn no_multiples_within_limit() {
        assert_eq!(0, sum_of_multiples(1, &[3, 5]))
    }
    #[test]
    
    fn one_factor_has_multiples_within_limit() {
        assert_eq!(3, sum_of_multiples(4, &[3, 5]))
    }
    #[test]
    
    fn more_than_one_multiple_within_limit() {
        assert_eq!(9, sum_of_multiples(7, &[3]))
    }
    #[test]
    
    fn more_than_one_factor_with_multiples_within_limit() {
        assert_eq!(23, sum_of_multiples(10, &[3, 5]))
    }
    #[test]
    
    fn each_multiple_is_only_counted_once() {
        assert_eq!(2318, sum_of_multiples(100, &[3, 5]))
    }
    #[test]
    
    fn a_much_larger_limit() {
        assert_eq!(233_168, sum_of_multiples(1000, &[3, 5]))
    }
    #[test]
    
    fn three_factors() {
        assert_eq!(51, sum_of_multiples(20, &[7, 13, 17]))
    }
    #[test]
    
    fn factors_not_relatively_prime() {
        assert_eq!(30, sum_of_multiples(15, &[4, 6]))
    }
    #[test]
    
    fn some_pairs_of_factors_relatively_prime_and_some_not() {
        assert_eq!(4419, sum_of_multiples(150, &[5, 6, 8]))
    }
    #[test]
    
    fn one_factor_is_a_multiple_of_another() {
        assert_eq!(275, sum_of_multiples(51, &[5, 25]))
    }
    #[test]
    
    fn much_larger_factors() {
        assert_eq!(2_203_160, sum_of_multiples(10_000, &[43, 47]))
    }
    #[test]
    
    fn all_numbers_are_multiples_of_1() {
        assert_eq!(4950, sum_of_multiples(100, &[1]))
    }
    #[test]
    
    fn no_factors_means_an_empty_sum() {
        assert_eq!(0, sum_of_multiples(10_000, &[]))
    }
    #[test]
    
    fn the_only_multiple_of_0_is_0() {
        assert_eq!(0, sum_of_multiples(1, &[0]))
    }
    #[test]
    
    fn the_factor_0_does_not_affect_the_sum_of_multiples_of_other_factors() {
        assert_eq!(3, sum_of_multiples(4, &[3, 0]))
    }
    #[test]
    
    fn solutions_using_include_exclude_must_extend_to_cardinality_greater_than_3() {
        assert_eq!(39_614_537, sum_of_multiples(10_000, &[2, 3, 5, 7, 11]))
    }
}