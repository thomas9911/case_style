//! [![Crates.io](https://img.shields.io/crates/v/case_style)](https://crates.io/crates/case_style)
//! [![Documentation](https://docs.rs/case_style/badge.svg)](https://docs.rs/case_style)
//! ![Crates.io](https://img.shields.io/crates/l/case_style)
//!
//! Converts from and to different case styles
//!
//! # Examples
//! There are two ways to use this libary. One is when you know the type upfront:
//! ```
//! use case_style::CaseStyle;
//! let kebab = CaseStyle::from_camelcase("camelCase").to_kebabcase();
//! println!("{}", kebab);
//! assert_eq!("camel-case", kebab);
//! ```
//!
//! And one where you don't know exactly:
//! ```
//! use case_style::CaseStyle;
//! let kebab = CaseStyle::guess("snake_case").unwrap().to_kebabcase();
//! println!("{}", kebab);
//! assert_eq!("snake-case", kebab);
//! ```
//!
//! The guess method is restrictive by design, so you should decide a backup type yourself like:
//! ```
//! use case_style::CaseStyle;
//! let input = "user just typed this in :P";
//! let snake = CaseStyle::guess(input)
//!     .unwrap_or(CaseStyle::from_sentencecase(input))
//!     .to_snakecase();
//! println!("{}", snake);
//! assert_eq!("user_just_typed_this_in_:p", snake);
//! ```
//!
//! In the previous example you can see that the convertion of styles does not remove odd characters. This you should also do youself.
//! ```
//! use case_style::CaseStyle;
//! let input = "this is just some random input 🦖 ";
//! let filtered_input: String = input
//!     .chars()
//!     .filter(|x| x == &' ' || x.is_ascii_alphanumeric())  
//!     .collect();
//! let filtered_input = filtered_input.trim();
//!
//! let snake = CaseStyle::guess(filtered_input)
//!     .unwrap_or(CaseStyle::from_lowercase_spacecase(filtered_input))
//!     .to_camelcase();
//! println!("{}", snake);
//! assert_eq!("thisIsJustSomeRandomInput", snake);
//! ```
//!
//! Current supported formats:
//!   - SnakeCase
//!   - CamelCase
//!   - ConstantCase
//!   - KebabCase
//!   - LowercaseSpace
//!   - PascalCase
//!   - SentenceCase
//!
//! for a up to date list look at the docs

pub mod formats;
pub mod guesser;
pub mod naming_conventions;
pub mod objects;
pub mod traits;
pub use objects::{Token, Tokens};
pub use traits::Case;

#[derive(Debug)]
pub struct CaseStyle {
    pub tokens: Tokens,
    pub original: String,
    pub case_info: Option<String>,
}

///
///
/// impl CaseStyle {
///     pub fn from_snakecase<S: AsRef<str>>(input: S) -> CaseStyle {
///         Self::decode(formats::SnakeCase::default(), input)
///     }

///     pub fn to_snakecase(self) -> String {
///         Self::encode(formats::SnakeCase::default(), self)
///     }
/// }
///
macro_rules! impl_case {
    ($from_name:ident, $to_name:ident, $input_type:ty, $type_name:expr) => {
        impl CaseStyle {
            #[doc = "uses"]
            #[doc = $type_name]
            #[doc = "Default trait to convert to CaseStyle"]
            pub fn $from_name<S: AsRef<str>>(input: S) -> CaseStyle {
                Self::decode(&<$input_type>::default(), input)
            }

            #[doc = "uses"]
            #[doc = $type_name]
            #[doc = "Default trait to convert to String"]
            pub fn $to_name(self) -> String {
                Self::encode(&<$input_type>::default(), self)
            }
        }
    };

    ($from_name:ident, $to_name:ident, $input_type:ty) => {
        impl_case!($from_name, $to_name, $input_type, stringify!($input_type));
    };
}

impl_case!(from_snakecase, to_snakecase, formats::SnakeCase);
impl_case!(from_camelcase, to_camelcase, formats::CamelCase);
impl_case!(from_kebabcase, to_kebabcase, formats::KebabCase);
impl_case!(
    from_lowercase_spacecase,
    to_lowercase_spacecase,
    formats::LowercaseSpace
);
impl_case!(from_pascalcase, to_pascalcase, formats::PascalCase);
impl_case!(from_constantcase, to_constantcase, formats::ConstantCase);
impl_case!(from_sentencecase, to_sentencecase, formats::SentenceCase);

impl CaseStyle {
    /// Use Case to convert string to CaseStyle object
    pub fn decode<T: Case, S: AsRef<str>>(case: &T, input: S) -> CaseStyle {
        case.parse_str(input.as_ref())
    }

    /// Use Case to convert CaseStyle object to String
    pub fn encode<T: Case>(case: &T, input: CaseStyle) -> String {
        case.build_string(input)
    }

    pub fn guess<S: AsRef<str>>(input: S) -> Result<CaseStyle, ()> {
        let case_type = guesser::guess_type(input.as_ref())?;
        Ok(case_type.parse_str(input.as_ref()))
    }
}

#[cfg(test)]
mod invertable {
    use crate::CaseStyle;

    #[test]
    fn snake() {
        assert_eq!(
            CaseStyle::from_snakecase("snake_case").to_snakecase(),
            "snake_case"
        );
    }

    #[test]
    fn kebab() {
        assert_eq!(
            CaseStyle::from_kebabcase("kebab-case").to_kebabcase(),
            "kebab-case"
        );
    }

    #[test]
    fn camel() {
        assert_eq!(
            CaseStyle::from_camelcase("camelCase").to_camelcase(),
            "camelCase"
        );
    }

    #[test]
    fn lowercase_space() {
        assert_eq!(
            CaseStyle::from_lowercase_spacecase("lowercase space").to_lowercase_spacecase(),
            "lowercase space"
        );
    }

    #[test]
    fn pascalcasee() {
        assert_eq!(
            CaseStyle::from_pascalcase("PascalCase").to_pascalcase(),
            "PascalCase"
        );
    }

    #[test]
    fn constant() {
        assert_eq!(
            CaseStyle::from_constantcase("CONSTANT_CASE").to_constantcase(),
            "CONSTANT_CASE"
        );
    }

    #[test]
    fn sentence() {
        assert_eq!(
            CaseStyle::from_sentencecase("This is a test sentence.").to_sentencecase(),
            "This is a test sentence."
        );
    }
}

#[test]
fn guess_test() {
    let s = "snake-case";
    let t = CaseStyle::guess(s).unwrap();
    let x = t.to_camelcase();

    assert_eq!(x, "snakeCase")
}
