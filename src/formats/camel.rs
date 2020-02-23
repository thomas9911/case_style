use crate::objects::Token;
use crate::traits::Case;
use crate::CaseStyle;

pub struct CamelCase;

impl Default for CamelCase {
    fn default() -> CamelCase {
        CamelCase {}
    }
}

impl Case for CamelCase {
    fn parse_str<S: AsRef<str>>(&self, input: S) -> CaseStyle {
        let mut tokens = Vec::with_capacity(input.as_ref().len());
        let mut first = true;
        tokens.push(Token::Start);

        for c in input.as_ref().chars() {
            if first {
                tokens.push(Token::FirstLetter(c.to_ascii_lowercase()));
                first = false;
            } else if c.is_uppercase() {
                tokens.push(Token::Spacing);
                tokens.push(Token::AfterSpacing(c.to_ascii_lowercase()));
            } else if c.is_ascii_alphanumeric() {
                tokens.push(Token::Letter(c.to_ascii_lowercase()));
            } else {
                tokens.push(Token::Literal(c.to_string()))
            }
        }
        tokens.push(Token::End);

        CaseStyle {
            tokens: tokens,
            original: String::from(input.as_ref()),
            case_info: Some(String::from("camel")),
        }
    }

    fn build_string(&self, case_style: CaseStyle) -> String {
        let mut buffer = String::with_capacity(case_style.tokens.len());

        for token in case_style.tokens.iter() {
            match token {
                Token::Start | Token::End | Token::Spacing => (),
                Token::FirstLetter(c) | Token::Letter(c) => buffer.push(*c),
                Token::AfterSpacing(c) => buffer.push(c.to_ascii_uppercase()),
                Token::Literal(s) => buffer.push_str(&s),
            }
        }

        buffer
    }
}
