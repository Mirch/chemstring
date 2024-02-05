# ChemString

[![CI](https://github.com/Mirch/chemstring/actions/workflows/ci.yml/badge.svg)](https://github.com/Mirch/chemstring/actions/workflows/ci.yml)
[![crates](https://img.shields.io/crates/v/chemstring)](https://crates.io/crates/chemstring)

 **ChemString** provides a parser that allows you to convert a string to its representation(s) using chemical symbols.
 ## Examples

 ```rust
    use chemstring::ChemString;

    let chem_string = ChemString::parse("seal").unwrap();
    assert_eq!(chem_string, vec!["Se", "Al"]);

    let chem_string = ChemString::parse("bichon").unwrap();
    let possible_permutation = "Bi C H O N".to_string();
     assert!(chem_string.results().contains(&possible_permutation));

 ```

## License

This project is licensed under the [MIT license](LICENSE).
