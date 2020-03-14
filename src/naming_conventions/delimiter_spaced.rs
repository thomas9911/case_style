use crate::objects::Token;
use crate::CaseStyle;

pub fn parse_str(input: &str, spacing_char: char) -> CaseStyle {
    let mut tokens = Vec::with_capacity(input.len());
    let mut first = true;
    let mut after_space = false;

    tokens.push(Token::Start);

    for c in input.chars() {
        if c == spacing_char {
            if first {
                tokens.push(Token::Literal(c.to_string()));
            } else {
                tokens.push(Token::Spacing);
                after_space = true;
            }
            continue;
        };
        let new_c = c.to_ascii_lowercase();
        if after_space {
            if new_c.is_digit(10) {
                tokens.push(Token::AfterSpacingDigit(new_c));
            } else {
                tokens.push(Token::AfterSpacingChar(new_c));
            }
            after_space = false;
            first = false;
            continue;
        };
        if first {
            tokens.push(Token::FirstLetter(new_c));
            first = false;
        } else {
            tokens.push(Token::Char(new_c));
        }
    }
    tokens.push(Token::End);

    CaseStyle {
        tokens: tokens,
        original: String::from(input),
        case_info: None,
    }
}

pub fn build_string(case_style: CaseStyle, spacing_char: char, do_uppercase: bool) -> String {
    let mut buffer = String::with_capacity(case_style.tokens.len());

    for token in case_style.tokens.iter() {
        match token {
            Token::Start | Token::End => (),
            Token::FirstLetter(c) | Token::Char(c) | Token::AfterSpacingChar(c) => {
                if do_uppercase {
                    buffer.push(c.to_ascii_uppercase())
                } else {
                    buffer.push(*c)
                }
            }
            Token::Digit(c) | Token::AfterSpacingDigit(c) => buffer.push(*c),
            Token::Literal(s) => buffer.push_str(&s),
            Token::Spacing => buffer.push(spacing_char),
        }
    }

    buffer
}
