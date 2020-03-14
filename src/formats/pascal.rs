use crate::naming_conventions::case_spaced::{build_string, parse_str};
use crate::traits::Case;
use crate::CaseStyle;

#[derive(Debug)]
pub struct PascalCase {
    pub extra_spacing_char: Option<char>,
}

impl Default for PascalCase {
    fn default() -> PascalCase {
        PascalCase {
            extra_spacing_char: None,
        }
    }
}

impl Case for PascalCase {
    fn parse_str(&self, input: &str) -> CaseStyle {
        parse_str(input, String::from("pascal"), self.extra_spacing_char)
    }

    fn build_string(&self, case_style: CaseStyle) -> String {
        build_string(case_style, self.extra_spacing_char, true)
    }
}
