use crate::objects::Token;
use crate::CaseStyle;

pub fn parse_str(input: &str, name: String, extra_spacing_char: Option<char>) -> CaseStyle {
    let mut tokens = Vec::with_capacity(input.len());
    let mut first = true;
    let mut after_spacing = false;
    tokens.push(Token::Start);

    for c in input.chars() {
        if let Some(spacing) = extra_spacing_char {
            if c == spacing {
                after_spacing = true;
                continue;
            }
        }

        if first {
            tokens.push(Token::FirstLetter(c.to_ascii_lowercase()));
            first = false;
        } else if c.is_uppercase() {
            tokens.push(Token::Spacing);
            tokens.push(Token::AfterSpacingChar(c.to_ascii_lowercase()));
        } else if c.is_ascii_alphabetic() {
            tokens.push(Token::Char(c.to_ascii_lowercase()));
        } else if c.is_digit(10) {
            if after_spacing {
                tokens.push(Token::Spacing);
                tokens.push(Token::AfterSpacingDigit(c));
            } else {
                tokens.push(Token::Digit(c));
            }
        } else {
            tokens.push(Token::Literal(c.to_string()))
        }
        after_spacing = false;
    }
    tokens.push(Token::End);

    CaseStyle {
        tokens: tokens,
        original: String::from(input),
        case_info: Some(name),
    }
}

pub fn build_string(
    case_style: CaseStyle,
    extra_spacing_char: Option<char>,
    start_with_capital_letter: bool,
) -> String {
    let mut buffer = String::with_capacity(case_style.tokens.len());

    for token in case_style.tokens.iter() {
        match token {
            Token::Start | Token::End | Token::Spacing => (),
            Token::FirstLetter(c) => {
                if start_with_capital_letter {
                    buffer.push(c.to_ascii_uppercase())
                } else {
                    buffer.push(*c)
                }
            }
            Token::Char(c) | Token::Digit(c) => buffer.push(*c),
            Token::AfterSpacingChar(c) => buffer.push(c.to_ascii_uppercase()),
            Token::AfterSpacingDigit(c) => {
                if let Some(a) = extra_spacing_char {
                    buffer.push(a);
                }
                buffer.push(*c);
            }
            Token::Literal(s) => buffer.push_str(&s),
        }
    }

    buffer
}
