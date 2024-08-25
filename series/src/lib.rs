pub fn series(digits: &str, len: usize) -> Vec<String> {
    let size = digits.len();
    if len > size {
        return vec![];
    }

    // Calculate the total number of series so the
    // the vector can be initialized with capacity
    // avoiding any new reallocations later.
    let total_series = size - len + 1;

    vec!["".to_string(); total_series]
        .iter()
        .enumerate()
        .map(|(i, _)| digits.chars().skip(i).take(len).collect::<String>())
        .collect()
}
