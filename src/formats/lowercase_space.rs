use crate::traits::DelimiterSpaced;

pub struct LowercaseSpace {
    spacing_char: char,
}

impl DelimiterSpaced for LowercaseSpace {
    fn spacing_char(&self) -> char {
        self.spacing_char
    }
}

impl Default for LowercaseSpace {
    fn default() -> LowercaseSpace {
        LowercaseSpace { spacing_char: ' ' }
    }
}
