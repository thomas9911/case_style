use crate::objects::Token;
use crate::CaseStyle;

pub trait Case: Default {
    fn parse_str<S: AsRef<str>>(&self, input: S) -> CaseStyle;
    fn build_string(&self, obj: CaseStyle) -> String;
}

pub trait DelimiterSpaced: Default {
    fn spacing_char(&self) -> char;
    fn do_uppercase(&self) -> bool {
        false
    }
}

impl<T> Case for T
where
    T: DelimiterSpaced,
{
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
            if first {
                tokens.push(Token::FirstLetter(new_c));
                first = false;
            } else {
                tokens.push(Token::Letter(new_c));
            }
        }
        tokens.push(Token::End);

        CaseStyle {
            tokens: tokens,
            original: String::from(input.as_ref()),
            case_info: None,
        }
    }

    fn build_string(&self, case_style: CaseStyle) -> String {
        let mut buffer = String::with_capacity(case_style.tokens.len());

        for token in case_style.tokens.iter() {
            match token {
                Token::Start | Token::End => (),
                Token::FirstLetter(c) | Token::Letter(c) | Token::AfterSpacing(c) => {
                    if self.do_uppercase() {
                        buffer.push(c.to_ascii_uppercase())
                    } else {
                        buffer.push(*c)
                    }
                }
                Token::Literal(s) => buffer.push_str(&s),
                Token::Spacing => buffer.push(self.spacing_char()),
            }
        }

        buffer
    }
}
