use case_style::CaseStyle;

fn main() {
    println!("Explicitly");
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
        CaseStyle::from_constantcase("CONSTANT_CASE").to_pascalcase()
    );
    println!(
        "{:?}",
        CaseStyle::from_kebabcase("kebab-case").to_lowercase_spacecase()
    );
    println!(
        "{:?}",
        CaseStyle::from_sentencecase("This is a sentence.").to_camelcase()
    );
    println!(
        "{:?}",
        CaseStyle::from_pascalcase("PascalCase").to_snakecase()
    );

    println!("\nImplicitly");
    println!(
        "{:?}",
        CaseStyle::guess("camelCase").unwrap().to_kebabcase()
    );
    println!(
        "{:?}",
        CaseStyle::guess("snake_case").unwrap().to_sentencecase()
    );
    println!(
        "{:?}",
        CaseStyle::guess("lowercase space")
            .unwrap()
            .to_constantcase()
    );
    println!(
        "{:?}",
        CaseStyle::guess("CONSTANT_CASE").unwrap().to_pascalcase()
    );
    println!(
        "{:?}",
        CaseStyle::guess("kebab-case")
            .unwrap()
            .to_lowercase_spacecase()
    );
    println!(
        "{:?}",
        CaseStyle::guess("This is a sentence.")
            .unwrap()
            .to_camelcase()
    );
    println!(
        "{:?}",
        CaseStyle::guess("PascalCase").unwrap().to_snakecase()
    );
}
