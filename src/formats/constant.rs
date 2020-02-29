use crate::traits::DelimiterSpaced;

#[derive(Debug)]
pub struct ConstantCase {
    spacing_char: char,
}

impl DelimiterSpaced for ConstantCase {
    fn spacing_char(&self) -> char {
        self.spacing_char
    }

    fn do_uppercase(&self) -> bool {
        true
    }
}

impl Default for ConstantCase {
    fn default() -> ConstantCase {
        ConstantCase { spacing_char: '_' }
    }
}
