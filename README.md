# case_style

[![Crates.io](https://img.shields.io/crates/v/case_style)](https://crates.io/crates/case_style)
[![Documentation](https://docs.rs/case_style/badge.svg)](https://docs.rs/case_style)
![Crates.io](https://img.shields.io/crates/l/case_style)

Converts from and to different case styles

## Examples
There are two ways to use this libary. One is when you know the type upfront:
```rust
use case_style::CaseStyle;
let kebab = CaseStyle::from_camelcase("camelCase").to_kebabcase();
println!("{}", kebab);
assert_eq!("camel-case", kebab);
```

Or from string
```rust
use case_style::CaseStyle;
let pascal = CaseStyle::from_case("kebab", "kebab-case")
    .expect("kebab is an existing format")
    .to_case("Pascal")
    .expect("pascal is an existing format");
println!("{}", pascal);
assert_eq!("KebabCase", pascal);
```

And one where you don't know exactly:
```rust
use case_style::CaseStyle;
let kebab = CaseStyle::guess("snake_case").unwrap().to_kebabcase();
println!("{}", kebab);
assert_eq!("snake-case", kebab);
```

The guess method is restrictive by design, so you should decide a backup type yourself like:
```rust
use case_style::CaseStyle;
let input = "user just typed this in :P";
let snake = CaseStyle::guess(input)
    .unwrap_or(CaseStyle::from_sentencecase(input))
    .to_snakecase();
println!("{}", snake);
assert_eq!("user_just_typed_this_in_:p", snake);
```

In the previous example you can see that the convertion of styles does not remove odd characters. This you should also do youself.
```rust
use case_style::CaseStyle;
let input = "this is just some random input 🦖 ";
let filtered_input: String = input
    .chars()
    .filter(|x| x == &' ' || x.is_ascii_alphanumeric())
    .collect();
let filtered_input = filtered_input.trim();

let snake = CaseStyle::guess(filtered_input)
    .unwrap_or(CaseStyle::from_lowercase_spacecase(filtered_input))
    .to_camelcase();
println!("{}", snake);
assert_eq!("thisIsJustSomeRandomInput", snake);
```

Current supported formats:
  - SnakeCase
  - CamelCase
  - ConstantCase
  - KebabCase
  - LowercaseSpace
  - PascalCase
  - SentenceCase

for a up to date list look at the docs

License: Unlicense
