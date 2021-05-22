/// Sorts the array in-place to be in the ascending order.
pub fn insertion_sort(_arr: &mut Vec<i32>) {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_swap_two_unsorted_elements() {
        let mut arr = vec![2, 1];

        insertion_sort(&mut arr);

        assert_eq!(arr, vec![1, 2]);
    }
}
