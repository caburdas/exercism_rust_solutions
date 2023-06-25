/*
Given two lists determine if the first list is contained within the second list, if the second list is contained within the first list, if both lists are contained within each other or if none of these are true.
Specifically, a list A is a sublist of list B if by dropping 0 or more elements from the front of B and 0 or more elements from the back of B you get a list that's completely equal to A.
Examples:
    A = [1, 2, 3], B = [1, 2, 3, 4, 5], A is a sublist of B
    A = [3, 4, 5], B = [1, 2, 3, 4, 5], A is a sublist of B
    A = [3, 4], B = [1, 2, 3, 4, 5], A is a sublist of B
    A = [1, 2, 3], B = [1, 2, 3], A is equal to B
    A = [1, 2, 3, 4, 5], B = [2, 3, 4], A is a superlist of B
    A = [1, 2, 4], B = [1, 2, 3, 4, 5], A is not a superlist of, sublist of or equal to B
 */

use std::f32::consts::E;

#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}
pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    match (_first_list.len(), _second_list.len()) {
        (0, 0) => Comparison::Equal,
        (0, _) => Comparison::Sublist,
        (_, 0) => Comparison::Superlist,
        (n, m) if n == m => {
            if _first_list == _second_list {
                return Comparison::Equal;
            } else {
                return Comparison::Unequal;
            }
        }
        (n, m) if n > m => {
            for sl in _first_list.windows(_second_list.len()) {
                if sl == _second_list {
                    return Comparison::Superlist;
                }
            }
            return Comparison::Unequal;
        }
        _ => {
            //_second_list > _first_list
            for sl in _second_list.windows(_first_list.len()) {
                if sl == _first_list {
                    return Comparison::Sublist;
                }
            }
            return Comparison::Unequal;
        }
    }
}

#[cfg(test)]
mod sublist_test {
    use super::{sublist, Comparison};
    #[test]
    fn empty_equals_empty() {
        let v: &[u32] = &[];
        assert_eq!(Comparison::Equal, sublist(v, v));
    }
    #[test]
    fn test_empty_is_a_sublist_of_anything() {
        assert_eq!(Comparison::Sublist, sublist(&[], &['a', 's', 'd', 'f']));
    }
    #[test]
    fn test_anything_is_a_superlist_of_empty() {
        assert_eq!(Comparison::Superlist, sublist(&['a', 's', 'd', 'f'], &[]));
    }
    #[test]
    fn test_1_is_not_2() {
        assert_eq!(Comparison::Unequal, sublist(&[1], &[2]));
    }
    #[test]
    fn test_compare_larger_equal_lists() {
        use std::iter::repeat;
        let v: Vec<char> = repeat('x').take(1000).collect();
        assert_eq!(Comparison::Equal, sublist(&v, &v));
    }
    #[test]
    fn test_sublist_at_start() {
        assert_eq!(Comparison::Sublist, sublist(&[1, 2, 3], &[1, 2, 3, 4, 5]));
    }
    #[test]
    fn sublist_in_middle() {
        assert_eq!(Comparison::Sublist, sublist(&[4, 3, 2], &[5, 4, 3, 2, 1]));
    }
    #[test]
    fn sublist_at_end() {
        assert_eq!(Comparison::Sublist, sublist(&[3, 4, 5], &[1, 2, 3, 4, 5]));
    }
    #[test]
    fn partially_matching_sublist_at_start() {
        assert_eq!(Comparison::Sublist, sublist(&[1, 1, 2], &[1, 1, 1, 2]));
    }
    #[test]
    fn sublist_early_in_huge_list() {
        let huge: Vec<u32> = (1..1_000_000).collect();
        assert_eq!(Comparison::Sublist, sublist(&[3, 4, 5], &huge));
    }
    #[test]
    fn huge_sublist_not_in_huge_list() {
        let v1: Vec<u64> = (10..1_000_001).collect();
        let v2: Vec<u64> = (1..1_000_000).collect();
        assert_eq!(Comparison::Unequal, sublist(&v1, &v2));
    }
    #[test]
    fn superlist_at_start() {
        assert_eq!(Comparison::Superlist, sublist(&[1, 2, 3, 4, 5], &[1, 2, 3]));
    }
    #[test]
    fn superlist_in_middle() {
        assert_eq!(Comparison::Superlist, sublist(&[5, 4, 3, 2, 1], &[4, 3, 2]));
    }
    #[test]
    fn superlist_at_end() {
        assert_eq!(Comparison::Superlist, sublist(&[1, 2, 3, 4, 5], &[3, 4, 5]));
    }
    #[test]
    fn second_list_missing_element_from_first_list() {
        assert_eq!(Comparison::Unequal, sublist(&[1, 2, 3], &[1, 3]));
    }
    #[test]

    fn superlist_early_in_huge_list() {
        let huge: Vec<u32> = (1..1_000_000).collect();
        assert_eq!(Comparison::Superlist, sublist(&huge, &[3, 4, 5]));
    }
    #[test]
    fn recurring_values_sublist() {
        assert_eq!(
            Comparison::Sublist,
            sublist(&[1, 2, 1, 2, 3], &[1, 2, 3, 1, 2, 1, 2, 3, 2, 1])
        );
    }
    #[test]
    fn recurring_values_unequal() {
        assert_eq!(
            Comparison::Unequal,
            sublist(&[1, 2, 1, 2, 3], &[1, 2, 3, 1, 2, 3, 2, 3, 2, 1])
        );
    }
}
