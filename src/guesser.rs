use crate::formats;
use crate::Case;

pub fn guess_type<S: AsRef<str>>(input: S) -> Result<Box<dyn Case>, ()> {
    match input.as_ref() {
        x if is_snake_case(x) => Ok(Box::new(formats::SnakeCase::default())),
        x if is_kebab_case(x) => Ok(Box::new(formats::KebabCase::default())),
        x if is_lowercase_space(x) => Ok(Box::new(formats::LowercaseSpace::default())),
        x if is_constant_case(x) => Ok(Box::new(formats::ConstantCase::default())),
        x if is_camel_case(x) => Ok(Box::new(formats::CamelCase::default())),
        x if is_sentence_case(x) => Ok(Box::new(formats::SentenceCase::default())),

        _ => Err(()),
    }
}

fn is_snake_case(input: &str) -> bool {
    input
        .chars()
        .all(|x| x == '_' || is_ascii_alphanumeric_lowercase(x))
}

fn is_constant_case(input: &str) -> bool {
    input
        .chars()
        .all(|x| x == '_' || is_ascii_alphanumeric_uppercase(x))
}

fn is_kebab_case(input: &str) -> bool {
    input
        .chars()
        .all(|x| x == '-' || is_ascii_alphanumeric_lowercase(x))
}

fn is_lowercase_space(input: &str) -> bool {
    input
        .chars()
        .all(|x| x == ' ' || is_ascii_alphanumeric_lowercase(x))
}

fn is_camel_case(input: &str) -> bool {
    let mut chars = input.chars();
    let what_first_char = chars.next().unwrap_or(' ').is_ascii_lowercase();
    what_first_char && chars.all(|x| x.is_ascii_alphabetic())
}

fn is_sentence_case(input: &str) -> bool {
    let mut chars = input.chars();
    let what_first_char = chars.next().unwrap_or(' ').is_ascii_uppercase();
    what_first_char && chars.all(|x| x == ' ' || x == '.' || is_ascii_alphanumeric_lowercase(x))
}

fn is_ascii_alphanumeric_lowercase(c: char) -> bool {
    c.is_ascii_lowercase() || c.is_digit(10)
}

fn is_ascii_alphanumeric_uppercase(c: char) -> bool {
    c.is_ascii_uppercase() || c.is_digit(10)
}

#[test]
fn snake() {
    assert!(is_snake_case("snake_case"));
    assert!(is_snake_case("testing"));
    assert!(is_snake_case("__testing_snake_case__"));
    assert!(!is_snake_case("snakeCase"));
    assert!(!is_snake_case("snake case"));
    assert!(!is_snake_case("SNAKE_CASE"));
    assert!(!is_snake_case("testing_:D"));
}

#[test]
fn kebab() {
    assert!(is_kebab_case("kebab-case"));
    assert!(is_kebab_case("testing"));
    assert!(is_kebab_case("testing-snake-case"));
    assert!(!is_kebab_case("kebabCase"));
    assert!(!is_kebab_case("KEBAB-CASE"));
    assert!(!is_kebab_case("kebab case"));
    assert!(!is_kebab_case("testing-:D"));
}

#[test]
fn lowercase_space() {
    assert!(is_lowercase_space("space case"));
    assert!(is_lowercase_space("lowercase"));
    assert!(is_lowercase_space("lowercase space case"));
    assert!(!is_lowercase_space("camelCase"));
    assert!(!is_lowercase_space("KEBAB-CASE"));
    assert!(!is_lowercase_space("snake_case"));
    assert!(!is_lowercase_space("lowercase space :D"));
}

#[test]
fn constant() {
    assert!(is_constant_case("SNAKE_CASE"));
    assert!(is_constant_case("TESTING"));
    assert!(is_constant_case("__TESTING_SNAKE_CASE__"));
    assert!(!is_constant_case("snakeCase"));
    assert!(!is_constant_case("SNAKE CASE"));
    assert!(!is_constant_case("snake_case"));
    assert!(!is_constant_case("TESTING_:D"));
}

#[test]
fn camel() {
    assert!(is_camel_case("camelCase"));
    assert!(is_camel_case("testing"));
    assert!(is_camel_case("testingCamelCase"));
    assert!(is_camel_case("sNAKECASE"));
    assert!(!is_camel_case("snake_case"));
    assert!(!is_camel_case("PascelCase"));
    assert!(!is_camel_case("camelCase:D"));
}

#[test]
fn sentence() {
    assert!(is_sentence_case("Sentence case"));
    assert!(is_sentence_case("Testing."));
    assert!(is_sentence_case("Sentence case testing."));
    assert!(!is_sentence_case("camelCase"));
    assert!(!is_sentence_case("PascalCase"));
    assert!(!is_sentence_case("KEBAB-CASE"));
    assert!(!is_sentence_case("snake_case"));
    assert!(!is_sentence_case("Lowercase space :D"));
}
