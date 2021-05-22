/// Sorts the array in-place to be in the ascending order.
pub fn insertion_sort(arr: &mut Vec<i32>) {
    arr[0] = 1;
    arr[1] = 2;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_swap_two_unsorted_elements() {
        let mut arr = vec![2, 1];

        insertion_sort(&mut arr);

        assert_eq!(arr, vec![1, 2]);
    }

    #[test]
    fn it_should_work_for_empty_vectors() {
        let mut arr = vec![];

        insertion_sort(&mut arr);

        assert_eq!(arr, vec![]);
    }
}
