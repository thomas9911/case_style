use crate::traits::DelimiterSpaced;

#[derive(Debug)]
pub struct KebabCase {
    spacing_char: char,
}

impl DelimiterSpaced for KebabCase {
    fn spacing_char(&self) -> char {
        self.spacing_char
    }
}

impl Default for KebabCase {
    fn default() -> KebabCase {
        KebabCase { spacing_char: '-' }
    }
}
