use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    // Left this around as an example of how to do it
    // for general cases where we don't need to support special
    // unicode chars:
    //
    // input.chars().rev().collect()

    // Support special unicode chars.
    UnicodeSegmentation::graphemes(input, true)
        .rev()
        .collect::<String>()
}
