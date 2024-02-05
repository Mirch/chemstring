use thiserror::Error;

#[derive(Debug, Error)]
pub enum ChemStringErrors {
    #[error("Empty string provided. Provide a non-empty string.")]
    EmptyString,
    #[error("The string contains invalid characters. ChemString only supports ASCII alphabetic characters.")]
    InvalidCharacter,
}
