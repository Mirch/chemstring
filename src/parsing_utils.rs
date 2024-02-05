use crate::symbols::SYMBOLS;

/// A parsing utility function that takes in a string and returns all the chemical symbols permutations
/// that can be formed from the string. The function is case-insensitive and will return an empty vector
/// if the input string is empty.
pub(crate) fn parse_string(value: String) -> Vec<String> {
    if value.is_empty() {
        return vec![String::new()];
    }

    let tokens: Vec<char> = value.chars().collect();
    let mut result: Vec<String> = vec![];

    let start = tokens[0].to_uppercase().to_string();
    if SYMBOLS.contains(&start.as_str()) {
        let remaining_possibilities = parse_string(tokens[1..].iter().collect());
        let possibilities = process_possibilities(start.clone(), remaining_possibilities);
        result.extend(possibilities);
    }

    if value.len() >= 2 {
        let start = format!("{}{}", tokens[0].to_uppercase(), tokens[1]);
        if SYMBOLS.contains(&start.as_str()) {
            let remaining_possibilities = parse_string(tokens[2..].iter().collect());
            let possibilities = process_possibilities(start, remaining_possibilities);
            result.extend(possibilities);
        }
    }

    result
}

/// A helper function that takes in a start string and a vector of remaining strings and returns a vector
/// of strings that are formed by combining the start string with each of the remaining strings.
fn process_possibilities(start: String, remaining: Vec<String>) -> Vec<String> {
    let mut result: Vec<String> = vec![];

    for remaining_possibility in remaining.iter() {
        result.push(
            format!("{} {}", start, remaining_possibility)
                .trim()
                .to_string(),
        );
    }

    result
}
