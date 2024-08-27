pub fn brackets_are_balanced(string: &str) -> bool {
    let mut brackets = vec![];

    for c in string.chars() {
        match c {
            '[' | '(' | '{' => brackets.push(c),
            ']' | ')' | '}' => {
                match brackets.pop() {
                    None => return false,
                    Some('[') if c != ']' => return false,
                    Some('(') if c != ')' => return false,
                    Some('{') if c != '}' => return false,
                    _ => {}
                };
            }
            _ => {}
        };
    }

    brackets.is_empty()
}
