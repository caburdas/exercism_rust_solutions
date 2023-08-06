/*
Introduction
You have stumbled upon a group of mathematicians who are also singer-songwriters. They have written a song for each of their favorite numbers, and, as you can imagine, they have a lot of favorite numbers (like 0 or 73 or 6174).
You are curious to hear the song for your favorite number, but with so many songs to wade through, finding the right song could take a while. Fortunately, they have organized their songs in a playlist sorted by the title — which is simply the number that the song is about.
You realize that you can use a binary search algorithm to quickly find a song given the title.
Instructions
Your task is to implement a binary search algorithm.
A binary search algorithm finds an item in a list by repeatedly splitting it in half, only keeping the half which contains the item we're looking for. It allows us to quickly narrow down the possible locations of our item until we find it, or until we've eliminated all possible locations.
Caution
Binary search only works when a list has been sorted.
The algorithm looks like this:
    Find the middle element of a sorted list and compare it with the item we're looking for.
    If the middle element is our item, then we're done!
    If the middle element is greater than our item, we can eliminate that element and all the elements after it.
    If the middle element is less than our item, we can eliminate that element and all the elements before it.
    If every element of the list has been eliminated then the item is not in the list.
    Otherwise, repeat the process on the part of the list that has not been eliminated.

Here's an example:
Let's say we're looking for the number 23 in the following sorted list: [4, 8, 12, 16, 23, 28, 32].
    We start by comparing 23 with the middle element, 16.
    Since 23 is greater than 16, we can eliminate the left half of the list, leaving us with [23, 28, 32].
    We then compare 23 with the new middle element, 28.
    Since 23 is less than 28, we can eliminate the right half of the list: [23].
    We've found our item.

Restrictions
Rust provides in its standard library already a binary search function. For this exercise you should not use this function but just other basic tools instead.
Hints
Slices have additionally to the normal element access via indexing (slice[index]) many useful functions like split_at or getting subslices (slice[start..end]).
You can solve this exercise by just using boring old element access via indexing, but maybe the other provided functions can make your code cleaner and safer.
For bonus points
Did you get the tests passing and the code clean? If you want to, there are some additional things you could try.
    Currently your find function will probably only work for slices of numbers, but the Rust type system is flexible enough to create a find function which works on all slices which contains elements which can be ordered.
    Additionally this find function can work not only on slices, but at the same time also on a Vec or an Array.
To run the bonus tests, remove the #[ignore] flag and execute the tests with the generic feature, like this:

$ cargo test --features generic

Then please share your thoughts in a comment on the submission. Did this experiment make the code better? Worse? Did you learn anything from it?
Hints for Bonus Points
    To get your function working with all kind of elements which can be ordered, have a look at the Ord Trait.
    To get your function working directly on Vec and Array, you can use the AsRef Trait
*/
pub fn find(array: &[i32], key: i32) -> Option<usize> {
    if array.len() == 0 {
        return None;
    }
    if key < array[0] {
        return None;
    }
    if key > array[array.len() - 1] {
        return None;
    }

    let mut lower = 0 as usize;
    let mut upper = array.len() - 1;

    while upper >= lower {
        let mid = (upper + lower) / 2;
        if array[mid] == key {
            return Some(mid);
        } else if key < array[mid] {
            upper = mid - 1;
        } else {
            lower = mid + 1;
        }
    }

    None
}

#[cfg(test)]
mod binary_search_tests {
    // The &[] borrows are required for the base exercise,
    // where `find` is not generic. Once `find` is made generic,
    // the borrows become needless. Since we want the tests to work
    // without clippy warnings for both people who take on the
    // additional challenge and people who don't, we disable this lint.
    #![allow(clippy::needless_borrow)]
    use crate::binary_search::find;
    #[test]
    fn finds_a_value_in_an_array_with_one_element() {
        assert_eq!(find(&[6], 6), Some(0));
    }
    #[test]
    fn finds_first_value_in_an_array_with_two_element() {
        assert_eq!(find(&[1, 2], 1), Some(0));
    }
    #[test]
    fn finds_second_value_in_an_array_with_two_element() {
        assert_eq!(find(&[1, 2], 2), Some(1));
    }
    #[test]
    fn finds_a_value_in_the_middle_of_an_array() {
        assert_eq!(find(&[1, 3, 4, 6, 8, 9, 11], 6), Some(3));
    }
    #[test]
    fn finds_a_value_at_the_beginning_of_an_array() {
        assert_eq!(find(&[1, 3, 4, 6, 8, 9, 11], 1), Some(0));
    }
    #[test]
    fn finds_a_value_at_the_end_of_an_array() {
        assert_eq!(find(&[1, 3, 4, 6, 8, 9, 11], 11), Some(6));
    }
    #[test]
    fn finds_a_value_in_an_array_of_odd_length() {
        assert_eq!(
            find(&[1, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 634], 144),
            Some(9)
        );
    }
    #[test]
    fn finds_a_value_in_an_array_of_even_length() {
        assert_eq!(
            find(&[1, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377], 21),
            Some(5)
        );
    }
    #[test]
    fn identifies_that_a_value_is_not_included_in_the_array() {
        assert_eq!(find(&[1, 3, 4, 6, 8, 9, 11], 7), None);
    }
    #[test]
    #[ignore]
    fn a_value_smaller_than_the_arrays_smallest_value_is_not_included() {
        assert_eq!(find(&[1, 3, 4, 6, 8, 9, 11], 0), None);
    }
    #[test]
    fn a_value_larger_than_the_arrays_largest_value_is_not_included() {
        assert_eq!(find(&[1, 3, 4, 6, 8, 9, 11], 13), None);
    }
    #[test]
    fn nothing_is_included_in_an_empty_array() {
        assert_eq!(find(&[], 1), None);
    }
    #[test]
    fn nothing_is_found_when_the_left_and_right_bounds_cross() {
        assert_eq!(find(&[1, 2], 0), None);
    }
    #[test]
    #[ignore]
    #[cfg(feature = "generic")]
    fn works_for_arrays() {
        assert_eq!(find([6], 6), Some(0));
    }
    #[test]
    #[ignore]
    #[cfg(feature = "generic")]
    fn works_for_vec() {
        let vector = vec![6];
        assert_eq!(find(&vector, 6), Some(0));
        assert_eq!(find(vector, 6), Some(0));
    }
    #[test]
    #[ignore]
    #[cfg(feature = "generic")]
    fn works_for_str_elements() {
        assert_eq!(find(["a"], "a"), Some(0));
        assert_eq!(find(["a", "b"], "b"), Some(1));
    }
}
