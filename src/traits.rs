use crate::naming_conventions::delimiter_spaced::{build_string, parse_str};
use crate::CaseStyle;
use std::fmt::Debug;

/// Trait for parsing strings to tokens and building string from tokens
pub trait Case: Debug {
    fn parse_str(&self, input: &str) -> CaseStyle;
    fn build_string(&self, obj: CaseStyle) -> String;
}

/// Trait for 'delimiter spaced' style strings, such as snake_case and kebab-case
pub trait DelimiterSpaced: Debug {
    fn spacing_char(&self) -> char;
    fn do_uppercase(&self) -> bool {
        false
    }
}

impl<T> Case for T
where
    T: DelimiterSpaced,
{
    fn parse_str(&self, input: &str) -> CaseStyle {
        parse_str(input, self.spacing_char())
    }

    fn build_string(&self, case_style: CaseStyle) -> String {
        build_string(case_style, self.spacing_char(), self.do_uppercase())
    }
}
