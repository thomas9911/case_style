use crate::naming_conventions::case_spaced::{build_string, parse_str};
use crate::traits::Case;
use crate::CaseStyle;

#[derive(Debug)]
pub struct CamelCase {
    pub extra_spacing_char: Option<char>,
}

impl Default for CamelCase {
    fn default() -> CamelCase {
        CamelCase {
            extra_spacing_char: None,
        }
    }
}

impl Case for CamelCase {
    fn parse_str(&self, input: &str) -> CaseStyle {
        parse_str(input, String::from("camel"), self.extra_spacing_char)
    }

    fn build_string(&self, case_style: CaseStyle) -> String {
        build_string(case_style, self.extra_spacing_char, false)
    }
}
