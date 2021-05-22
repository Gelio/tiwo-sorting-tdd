/// Sorts the array in-place to be in the ascending order.
pub fn insertion_sort(arr: &mut Vec<i32>) {
    for index_to_sort in 1..arr.len() {
        let v = arr[index_to_sort];

        (0..index_to_sort)
            .find(|&i| arr[i] >= v)
            .and_then(|index_to_insert_at| {
                arr[index_to_insert_at..=index_to_sort].rotate_right(1);

                Some(())
            });
    }
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

    #[test]
    fn it_should_sort_a_few_unsorted_elements() {
        let mut arr = vec![1, 8, 3, 2, 5, 10, -1];

        insertion_sort(&mut arr);

        assert_eq!(arr, vec![-1, 1, 2, 3, 5, 8, 10]);
    }
}
