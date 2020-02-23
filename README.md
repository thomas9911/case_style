# case_style

Converts from and to different case style

## example
```rust
use case_style::CaseStyle;
let kebab = CaseStyle::from_camelcase("camelCase").to_kebabcase();
println!("{}", kebab);
```

Current supported formats:
  - SnakeCase
  - CamelCase
  - ConstantCase
  - KebabCase
  - LowercaseSpace
  - SentenceCase

for a up to date list look at the docs
