/// Sorts the array in-place to be in the ascending order.
pub fn insertion_sort(arr: &mut Vec<impl Ord>) {
    for index_to_sort in 1..arr.len() {
        let v = &arr[index_to_sort];

        (0..index_to_sort)
            .find(|&i| arr[i].ge(v))
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
        let mut arr: Vec<()> = vec![];

        insertion_sort(&mut arr);

        assert_eq!(arr, vec![]);
    }

    #[test]
    fn it_should_sort_a_few_unsorted_elements() {
        let mut arr = vec![1, 8, 3, 2, 5, 10, -1];

        insertion_sort(&mut arr);

        assert_eq!(arr, vec![-1, 1, 2, 3, 5, 8, 10]);
    }

    #[derive(PartialEq, Eq, Debug)]
    struct MyNode {
        /// The key to sort by
        key: i32,
        metadata: i32,
    }

    impl PartialOrd for MyNode {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            self.key.partial_cmp(&other.key)
        }
    }

    impl Ord for MyNode {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.key.cmp(&other.key)
        }
    }

    fn get_simple_nodes_vector() -> Vec<MyNode> {
        vec![
            MyNode {
                key: 1,
                metadata: 1,
            },
            MyNode {
                key: 5,
                metadata: 2,
            },
            MyNode {
                key: 1,
                metadata: 3,
            },
            MyNode {
                key: 8,
                metadata: 4,
            },
            MyNode {
                key: 1,
                metadata: 5,
            },
        ]
    }

    #[test]
    fn it_should_sort_any_vector_which_implements_ord() {
        let mut arr = get_simple_nodes_vector();

        insertion_sort(&mut arr);

        assert_eq!(
            arr.into_iter().map(|node| node.key).collect::<Vec<i32>>(),
            vec![1, 1, 1, 5, 8],
        )
    }

    #[test]
    fn it_should_perform_stable_sorting() {
        let mut arr = get_simple_nodes_vector();

        insertion_sort(&mut arr);

        assert_eq!(
            arr,
            vec![
                MyNode {
                    key: 1,
                    metadata: 1
                },
                MyNode {
                    key: 1,
                    metadata: 3
                },
                MyNode {
                    key: 1,
                    metadata: 5
                },
                MyNode {
                    key: 5,
                    metadata: 2
                },
                MyNode {
                    key: 8,
                    metadata: 4
                }
            ]
        )
    }
}
