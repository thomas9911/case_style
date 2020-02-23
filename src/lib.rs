pub mod formats;
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
                Self::decode(<$input_type>::default(), input)
            }

            #[doc = "uses"]
            #[doc = $type_name]
            #[doc = "Default trait to convert to String"]
            pub fn $to_name(self) -> String {
                Self::encode(<$input_type>::default(), self)
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
impl_case!(from_constantcase, to_constantcase, formats::ConstantCase);
impl_case!(from_sentencecase, to_sentencecase, formats::SentenceCase);

impl CaseStyle {
    /// Use Case to convert string to CaseStyle object
    pub fn decode<T: Case, S: AsRef<str>>(case: T, input: S) -> CaseStyle {
        case.parse_str(input)
    }

    /// Use Case to convert CaseStyle object to String
    pub fn encode<T: Case>(case: T, input: CaseStyle) -> String {
        case.build_string(input)
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
