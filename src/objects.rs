pub type Tokens = Vec<Token>;

#[derive(Debug, PartialEq)]
pub enum Token {
    /// how words are split
    Spacing,
    /// first char in the stream
    FirstLetter(char),
    /// just a char
    Char(char),
    /// first char after split
    AfterSpacingChar(char),
    /// just a digit
    Digit(char),
    /// first digit after split
    AfterSpacingDigit(char),
    /// text that should not be treated
    Literal(String),
    /// start of the stream
    Start,
    /// end of the stream
    End,
}
