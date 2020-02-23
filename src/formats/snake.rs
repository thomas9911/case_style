use crate::traits::DelimiterSpaced;

pub struct SnakeCase {
    spacing_char: char,
}

impl DelimiterSpaced for SnakeCase {
    fn spacing_char(&self) -> char {
        self.spacing_char
    }
}

impl Default for SnakeCase {
    fn default() -> SnakeCase {
        SnakeCase { spacing_char: '_' }
    }
}
