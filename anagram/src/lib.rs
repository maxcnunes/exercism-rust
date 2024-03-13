use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let w_ordered = to_ordered_string(word);

    let matches = possible_anagrams
        .iter()
        .filter_map(|pa| {
            let pa_ordered = to_ordered_string(pa);

            if w_ordered == pa_ordered && word.to_lowercase() != pa.to_lowercase() {
                Some(pa.to_owned())
            } else {
                None
            }
        })
        .collect::<Vec<&'a str>>();

    HashSet::from_iter(matches)
}

fn to_ordered_string(w: &str) -> String {
    let mut w_ordered = w.to_lowercase().chars().collect::<Vec<char>>();
    w_ordered.sort();
    w_ordered.iter().collect::<String>()
}
