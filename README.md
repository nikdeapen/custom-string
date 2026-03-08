# custom-string

[![Crates.io](https://img.shields.io/crates/v/custom-string.svg)](https://crates.io/whcrates/custom-string)
[![Docs.rs](https://docs.rs/custom-string/badge.svg)](https://docs.rs/custom-string)
[![License: MIT](https://img.shields.io/crates/l/custom-string.svg)](https://opensource.org/licenses/MIT)

This library aids in generating string types with custom validation.

    custom-string = "0.10.0-rc.1"

## Features

    serde

For more features see the [Crate Docs](https://docs.rs/custom-string/latest/custom_string/).

## Example

```rust
use custom_string::custom_string;

custom_string!(
    #[doc = "A lowercase string."],
    Lower,
    |s: &str| if !s.as_bytes().iter().all(|c| c.is_ascii_lowercase()) {
        Err("not lowercase")
    } else {
        Ok(())
    }
);

let owned: Lower = Lower::new("hello").unwrap();
assert_eq!(owned.value(), "hello");

let reference: LowerRef = owned.to_ref();
assert_eq!(reference.value(), "hello");

assert!(Lower::new("HELLO").is_err());
```
