use case_style::formats::CamelCase;
use case_style::CaseStyle;

fn main() {
    let custom_case = CamelCase {
        extra_spacing_char: Some('_'),
    };
    let input = "camelCase_123";
    let tokens = CaseStyle::decode(&custom_case, input);
    let output = CaseStyle::encode(&custom_case, tokens);
    println!("{:?}", output);
    assert_eq!(input, output);
}
