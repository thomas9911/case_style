use case_style::CaseStyle;

fn main() {
    println!(
        "{:?}",
        CaseStyle::from_camelcase("camelCase").to_kebabcase()
    );
    println!(
        "{:?}",
        CaseStyle::from_snakecase("snake_case").to_sentencecase()
    );
    println!(
        "{:?}",
        CaseStyle::from_lowercase_spacecase("lowercase space").to_constantcase()
    );
    println!(
        "{:?}",
        CaseStyle::from_constantcase("CONSTANT_CASE").to_snakecase()
    );
    println!(
        "{:?}",
        CaseStyle::from_kebabcase("kebab-case").to_lowercase_spacecase()
    );
    println!(
        "{:?}",
        CaseStyle::from_sentencecase("This is a sentence.").to_camelcase()
    );
}
