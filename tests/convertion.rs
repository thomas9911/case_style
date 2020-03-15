use case_style::CaseStyle;

#[test]
fn explicitly() {
    assert_eq!(
        "camel-case",
        CaseStyle::from_camelcase("camelCase").to_kebabcase()
    );
    assert_eq!(
        "Snake case.",
        CaseStyle::from_snakecase("snake_case").to_sentencecase()
    );
    assert_eq!(
        "LOWERCASE_SPACE",
        CaseStyle::from_lowercase_spacecase("lowercase space").to_constantcase()
    );
    assert_eq!(
        "ConstantCase",
        CaseStyle::from_constantcase("CONSTANT_CASE").to_pascalcase()
    );
    assert_eq!(
        "kebab case",
        CaseStyle::from_kebabcase("kebab-case").to_lowercase_spacecase()
    );
    assert_eq!(
        "thisIsASentence",
        CaseStyle::from_sentencecase("This is a sentence.").to_camelcase()
    );
    assert_eq!(
        "pascal_case",
        CaseStyle::from_pascalcase("PascalCase").to_snakecase()
    );
}

#[test]
fn implicitly() {
    assert_eq!(
        "camel-case",
        CaseStyle::guess("camelCase").unwrap().to_kebabcase()
    );
    assert_eq!(
        "Snake case.",
        CaseStyle::guess("snake_case").unwrap().to_sentencecase()
    );
    assert_eq!(
        "LOWERCASE_SPACE",
        CaseStyle::guess("lowercase space")
            .unwrap()
            .to_constantcase()
    );
    assert_eq!(
        "ConstantCase",
        CaseStyle::guess("CONSTANT_CASE").unwrap().to_pascalcase()
    );
    assert_eq!(
        "kebab case",
        CaseStyle::guess("kebab-case")
            .unwrap()
            .to_lowercase_spacecase()
    );
    assert_eq!(
        "thisIsASentence",
        CaseStyle::guess("This is a sentence.")
            .unwrap()
            .to_camelcase()
    );
    assert_eq!(
        "pascal_case",
        CaseStyle::guess("PascalCase").unwrap().to_snakecase()
    );
}
