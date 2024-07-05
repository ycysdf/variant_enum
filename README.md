# variant_enum

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/ycysdf/variant_enum#LICENSE)
[![Crates.io](https://img.shields.io/crates/v/variant_enum.svg)](https://crates.io/crates/variant_enum)
[![Docs](https://docs.rs/variant_enum/badge.svg)](https://docs.rs/variant_enum)

## Example

```rust
use variant_enum::typed_variant;

#[typed_variant(derive(Debug))]
#[derive(derive_more::From)]
pub enum Msg {
    #[inner(derive(Clone))]
    A {
        pub a: u32,
        b: f32,
        c: usize,
        d: String,
    },
    B {
        foo: String,
    },
    C {
        bar: bool,
    },
}
```

generated:

```rust
#[derive(Debug)]
#[derive(derive_more::From)]
pub enum Msg {
    A(A),
    B(B),
    C(C),
}
#[derive(Clone)]
#[derive(Debug)]
pub struct A {
    pub a: u32,
    b: f32,
    c: usize,
    d: String,
}
#[derive(Debug)]
pub struct B {
    foo: String,
}
#[derive(Debug)]
pub struct C {
    bar: bool,
}
impl TryFrom<Msg> for A {
    type Error = Msg;
    fn try_from(value: Msg) -> Result<Self, Self::Error> { if let Msg::A(m) = value { Ok(m) } else { Err(value) } }
}
impl TryFrom<Msg> for B {
    type Error = Msg;
    fn try_from(value: Msg) -> Result<Self, Self::Error> { if let Msg::B(m) = value { Ok(m) } else { Err(value) } }
}
impl TryFrom<Msg> for C {
    type Error = Msg;
    fn try_from(value: Msg) -> Result<Self, Self::Error> { if let Msg::C(m) = value { Ok(m) } else { Err(value) } }
}
```