pub type Tokens = Vec<Token>;

#[derive(Debug, PartialEq)]
pub enum Token {
    /// how words are split
    Spacing,
    /// first char in the stream
    FirstLetter(char),
    /// first char after split
    AfterSpacing(char),
    /// just a char
    Letter(char),
    /// text that should not be treated
    Literal(String),
    /// start of the stream
    Start,
    /// end of the stream
    End,
}
