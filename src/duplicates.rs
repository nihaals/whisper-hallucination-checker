#[derive(Debug, PartialEq, Eq)]
pub struct DuplicateResult<T> {
    pub sequence: Vec<T>,
    pub count: usize,
}

/// Finds the longest sequence of consecutive duplicates in a list of items.
///
/// # Examples
///
/// ```
/// assert_eq!(find_duplicates(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]), DuplicateResult { sequence: vec![], count: 0 });
/// assert_eq!(find_duplicates(vec![1, 2, 3, 1, 3, 2, 1, 3, 1, 3]), DuplicateResult { sequence: vec![1, 3], count: 2 });
/// ```
pub fn find_duplicates<T: Eq + Clone>(items: Vec<T>) -> DuplicateResult<T> {
    let mut longest_repeated = Vec::new();
    let mut longest_length = 0;
    let mut count = 0;

    // Identify the longest sequence
    for i in 0..items.len() {
        for length in 1..=3 {
            if i + 2 * length <= items.len() {
                let sequence = &items[i..i + length];
                let next_sequence = &items[i + length..i + 2 * length];
                if sequence == next_sequence && length > longest_length {
                    longest_repeated = sequence.to_vec();
                    longest_length = length;
                }
            }
        }
    }

    if !longest_repeated.is_empty() {
        let mut i = 0;
        while i <= items.len() - longest_length {
            if &items[i..i + longest_length] == longest_repeated.as_slice() {
                count += 1;
                // Skip over this sequence for the next iteration
                i += longest_length;
                continue;
            }
            i += 1;
        }
    }

    DuplicateResult {
        sequence: longest_repeated,
        count,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! find_duplicates_test {
        ($test_name:ident, $input:expr, $sequence:expr, $count:expr) => {
            #[test]
            fn $test_name() {
                assert_eq!(
                    find_duplicates($input),
                    DuplicateResult {
                        sequence: $sequence,
                        count: $count
                    }
                )
            }
        };
    }

    find_duplicates_test!(test_find_duplicates_empty, Vec::<usize>::new(), vec![], 0);
    find_duplicates_test!(
        test_find_duplicates_ascending,
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
        vec![],
        0
    );
    find_duplicates_test!(
        test_find_duplicates_descending,
        vec![9, 8, 7, 6, 5, 4, 3, 2, 1],
        vec![],
        0
    );
    find_duplicates_test!(
        test_find_duplicates_single,
        vec![1, 1, 1, 1, 1, 1, 1, 1],
        vec![1],
        8
    );
    find_duplicates_test!(
        test_find_duplicates_two_start,
        vec![1, 2, 1, 2, 1, 2],
        vec![1, 2],
        3
    );
    find_duplicates_test!(
        test_find_duplicates_two,
        vec![1, 2, 3, 2, 3, 2, 3],
        vec![2, 3],
        3
    );
    find_duplicates_test!(
        test_find_duplicates_three_start,
        vec![1, 2, 3, 1, 2, 3, 1, 2, 3],
        vec![1, 2, 3],
        3
    );
    find_duplicates_test!(
        test_find_duplicates_three,
        vec![1, 2, 3, 4, 2, 3, 4, 2, 3, 4],
        vec![2, 3, 4],
        3
    );
    find_duplicates_test!(
        test_find_duplicates_two_start_twice,
        vec![1, 2, 1, 2],
        vec![1, 2],
        2
    );
    find_duplicates_test!(
        test_find_duplicates_multiple,
        vec![1, 2, 1, 2, 3, 4, 3, 4, 3, 4],
        vec![1, 2],
        2
    );
    find_duplicates_test!(
        test_find_duplicates_multiple_shorter,
        vec![1, 2, 3, 1, 2, 3, 4, 5, 4, 5, 4, 5, 4, 5],
        vec![1, 2, 3],
        2
    );
    find_duplicates_test!(
        test_find_duplicates_multiple_same_length,
        vec![1, 2, 1, 2, 3, 4, 3, 4],
        vec![1, 2],
        2
    );
    find_duplicates_test!(
        test_find_duplicates_repeat,
        vec![1, 2, 3, 1, 3, 2, 1, 3, 1, 3],
        vec![1, 3],
        2
    );
    find_duplicates_test!(
        test_find_duplicates_repeat_no_duplicates,
        vec![1, 2, 3, 4, 1, 2, 3],
        vec![],
        0
    );
}
