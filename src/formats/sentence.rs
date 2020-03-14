use crate::objects::Token;
use crate::traits::Case;
use crate::CaseStyle;

#[derive(Debug)]
pub struct SentenceCase {
    spacing_char: char,
}

impl Default for SentenceCase {
    fn default() -> SentenceCase {
        SentenceCase { spacing_char: ' ' }
    }
}

impl SentenceCase {
    fn spacing_char(&self) -> char {
        self.spacing_char
    }
}

impl Case for SentenceCase {
    fn parse_str(&self, input: &str) -> CaseStyle {
        let mut tokens = Vec::with_capacity(input.len());
        let mut first = true;
        let mut after_space = false;
        tokens.push(Token::Start);

        for c in input.chars() {
            if c == self.spacing_char() {
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

            if c == '.' {
                tokens.push(Token::End);
                tokens.push(Token::Start);
                first = true;
                continue;
            }

            if first {
                tokens.push(Token::FirstLetter(new_c));
                first = false;
            } else {
                if new_c.is_digit(10) {
                    tokens.push(Token::Digit(new_c));
                } else {
                    tokens.push(Token::Char(new_c));
                }
            }
        }

        CaseStyle {
            tokens: tokens,
            original: String::from(input),
            case_info: Some(String::from("sentence")),
        }
    }

    fn build_string(&self, case_style: CaseStyle) -> String {
        let mut buffer = String::with_capacity(case_style.tokens.len());

        for token in case_style.tokens.iter() {
            match token {
                Token::Start => (),
                Token::End => buffer.push('.'),
                Token::Spacing => buffer.push(' '),
                Token::FirstLetter(c) => buffer.push(c.to_ascii_uppercase()),
                Token::Char(c)
                | Token::AfterSpacingChar(c)
                | Token::Digit(c)
                | Token::AfterSpacingDigit(c) => buffer.push(*c),
                Token::Literal(s) => buffer.push_str(&s),
            }
        }

        buffer
    }
}
