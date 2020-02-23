use crate::objects::Token;
use crate::traits::Case;
use crate::CaseStyle;

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
    fn parse_str<S: AsRef<str>>(&self, input: S) -> CaseStyle {
        let mut tokens = Vec::with_capacity(input.as_ref().len());
        let mut first = true;
        let mut after_space = false;
        tokens.push(Token::Start);

        for c in input.as_ref().chars() {
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
                tokens.push(Token::AfterSpacing(new_c));
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
                tokens.push(Token::Letter(new_c));
            }
        }

        CaseStyle {
            tokens: tokens,
            original: String::from(input.as_ref()),
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
                Token::Letter(c) | Token::AfterSpacing(c) => buffer.push(*c),
                Token::Literal(s) => buffer.push_str(&s),
            }
        }

        buffer
    }
}
