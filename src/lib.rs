//! A library for parsing strings into chemical symbols permutations.
//! <br>
//!
//! # Example
//! ```
//! use chemstring::ChemString;
//!
//! let chem_string = ChemString::parse("seal").unwrap();
//! assert_eq!(chem_string.results(), vec!["Se Al"]);
//!
//! let chem_string = ChemString::parse("bichon").unwrap();
//! let possible_permutation = "Bi C H O N".to_string();
//! assert!(chem_string.results().contains(&possible_permutation));
//!
//! ````

pub mod errors;
pub mod parsing_utils;
pub mod symbols;

use errors::ChemStringErrors;
use parsing_utils::parse_string;

/// A `ChemString` is a string that can be written using only chemical symbols.
/// The `ChemString` struct has a single field, `0`, which is a vector of all the possible
/// chemical symbols permutations that can be formed from the string.
pub struct ChemString(Vec<String>);

impl ChemString {
    /// Attemtps to parse a string and returns a `Result` containing a `ChemString` if the
    /// string is valid, and an error otherwise. The function is case-insensitive and will
    /// return an error if the input string is empty or contains any non-alphabetic characters.
    pub fn parse(s: impl Into<String>) -> Result<Self, ChemStringErrors> {
        let s: String = s.into();
        if s.is_empty() {
            return Err(ChemStringErrors::EmptyString);
        }
        let s = s.split_whitespace().collect::<String>().to_lowercase();

        if s.chars().any(|c| !c.is_ascii_alphabetic()) {
            return Err(ChemStringErrors::InvalidCharacter);
        }

        let result = parse_string(s);

        Ok(ChemString(result))
    }

    /// Returns a vector of all the possible chemical symbols permutations that can be formed
    /// from the string.
    pub fn results(&self) -> Vec<String> {
        self.0.clone()
    }

    /// Returns the number of possible chemical symbols permutations that can be formed from the string.
    pub fn count(&self) -> usize {
        self.0.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_chem_string_with_one_possible_result_is_parsed() {
        let chem_string = "seal";

        let result = ChemString::parse(chem_string).unwrap();

        assert_eq!(result.0, vec!["Se Al"]);
    }

    #[test]
    fn valid_chem_string_with_multiple_possible_results_is_parsed() {
        let chem_string = "bichon";
        let mut possible_results = ["Bi C H O N", "B I C H O N", "Bi C Ho N", "B I C Ho N"];

        let mut result = ChemString::parse(chem_string).unwrap();
        possible_results.sort();
        result.0.sort();

        assert_eq!(result.0, possible_results);
    }

    #[test]
    fn invalid_chem_strings_are_not_parsed() {
        let invalid_chem_strings = vec![
            "",
            "123",
            "bichon123",
            "bichon 123",
            "?#!$@",
            "bichon??",
            "bichon ??",
        ];

        for invalid_chem_string in invalid_chem_strings {
            let result = ChemString::parse(invalid_chem_string);

            assert!(result.is_err());
        }
    }
}
