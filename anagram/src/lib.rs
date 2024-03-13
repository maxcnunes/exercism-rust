use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let (w_lower, w_sorted) = transform(word);

    let matches = possible_anagrams
        .iter()
        .filter_map(|pa| {
            let (pa_lower, pa_sorted) = transform(pa);

            // Check the words are equal after the lower case and sorting transformations
            if w_lower != pa_lower && w_sorted == pa_sorted {
                Some(pa.to_owned())
            } else {
                None
            }
        })
        .collect::<Vec<&'a str>>();

    HashSet::from_iter(matches)
}

fn transform(w: &str) -> (Vec<char>, String) {
    // Make a copy of the word in lower case, and keep it as a slice of chars
    let w_lower = w.to_lowercase().chars().collect::<Vec<char>>();

    // Make a new copy of that slice, so we can sort it, but without modifying the previous slice.
    // This is required so we can compare later the lower case version, without being sorted.
    let mut w_chars_sort = w_lower.clone();

    // Sort the first slice.
    w_chars_sort.sort();

    // Transform the slice of chars sorted version into a String.
    let w_sorted = w_chars_sort.iter().collect::<String>();

    (w_lower, w_sorted)
}
