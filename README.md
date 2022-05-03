# Symbols

## What is this?

![What the Fuck Did You Just Bring Upon This Cursed Land](./img/meme.jpg)

This is an utility to build a proc-macro that connects to a database, retrieves data from given table and populates an enum variants with primary keys values.<br />
It also generates a method for every non-primary-key field, and, when there are multiple primary keys, a costructor for every possible subset of primary keys.

## Replacements

Replacements are done using annotated parameters.<br/>
Basic replacements are written in the form `#[macro(field = "enum")]` or `#[macro(field(type = "enum"))]`, where we are telling to replace string values from `field` with variants from enum `enum`, variant names will be the CamelCase version of field value.<br />
Advanced replacements are done in the form `#[macro(field(type = "bar", fn = "foo"))]`, where we are telling to replace string values from `field` with a call to method `foo` from struct/enum `bar`, method output is expected to be of type `bar`.<br />
*WARNING:* Since all produced methods are `const`, also methods you pass this way must be `const`.

### Cache

To avoid flooding the database with requests, light up macro run times and be able to work offline, data cache files are stored in crate folder under the name of `<table>.cache`.<br />
What does crate folder means? Well, if you're working directly on this crate, you'll find cache files in crate folder. If you're using this crate as a dependency of your project, you'll find cache files in cargo home folder, under git/checkouts folders (e.g. `~/.cargo/git/checkouts/symbols-3627ca5bd9a20120/4e682cf/<table>.cache`).<br />
Quick way to delete it is to run
```bash
find ~/.cargo/git -name *.cache -delete
```

### Examples

You can find a basic example in `example` folder, it uses a mariadb container to load a database, you can run it with:
```bash
docker-compose run rust
```
Example code would expand to:
```rust
pub enum Example {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
}

impl Example {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Example::A => "A",
            Example::B => "B",
            Example::C => "C",
            Example::D => "D",
            Example::E => "E",
            Example::F => "F",
            Example::G => "G",
            Example::H => "H",
            Example::I => "I",
            Example::J => "J",
            Example::K => "K",
            Example::L => "L",
            Example::M => "M",
            Example::N => "N",
            Example::O => "O",
            Example::P => "P",
            Example::Q => "Q",
            Example::R => "R",
            Example::S => "S",
            Example::T => "T",
            Example::U => "U",
            Example::V => "V",
            Example::W => "W",
            Example::X => "X",
            Example::Y => "Y",
            Example::Z => "Z",
        }
    }
    pub const fn kind(&self) -> Kind {
        match self {
            Example::A => Kind::from_str("D"),
            Example::B => Kind::from_str("D"),
            Example::C => Kind::from_str("D"),
            Example::D => Kind::from_str("D"),
            Example::E => Kind::from_str("D"),
            Example::F => Kind::from_str("D"),
            Example::G => Kind::from_str("D"),
            Example::H => Kind::from_str("D"),
            Example::I => Kind::from_str("D"),
            Example::J => Kind::from_str("C"),
            Example::K => Kind::from_str("C"),
            Example::L => Kind::from_str("D"),
            Example::M => Kind::from_str("D"),
            Example::N => Kind::from_str("D"),
            Example::O => Kind::from_str("D"),
            Example::P => Kind::from_str("D"),
            Example::Q => Kind::from_str("D"),
            Example::R => Kind::from_str("D"),
            Example::S => Kind::from_str("D"),
            Example::T => Kind::from_str("D"),
            Example::U => Kind::from_str("D"),
            Example::V => Kind::from_str("D"),
            Example::W => Kind::from_str("C"),
            Example::X => Kind::from_str("C"),
            Example::Y => Kind::from_str("C"),
            Example::Z => Kind::from_str("D"),
        }
    }
}

impl<'a> TryFrom<&'a str> for Example {
    type Error = String;
    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        match s {
            "A" => Ok(Example::A),
            "B" => Ok(Example::B),
            "C" => Ok(Example::C),
            "D" => Ok(Example::D),
            "E" => Ok(Example::E),
            "F" => Ok(Example::F),
            "G" => Ok(Example::G),
            "H" => Ok(Example::H),
            "I" => Ok(Example::I),
            "J" => Ok(Example::J),
            "K" => Ok(Example::K),
            "L" => Ok(Example::L),
            "M" => Ok(Example::M),
            "N" => Ok(Example::N),
            "O" => Ok(Example::O),
            "P" => Ok(Example::P),
            "Q" => Ok(Example::Q),
            "R" => Ok(Example::R),
            "S" => Ok(Example::S),
            "T" => Ok(Example::T),
            "U" => Ok(Example::U),
            "V" => Ok(Example::V),
            "W" => Ok(Example::W),
            "X" => Ok(Example::X),
            "Y" => Ok(Example::Y),
            "Z" => Ok(Example::Z),
            _ => Err({
                let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                    &["Unknown ", " "],
                    &[
                        ::core::fmt::ArgumentV1::new_display(&"Example"),
                        ::core::fmt::ArgumentV1::new_display(&s),
                    ],
                ));
                res
            }),
        }
    }
}
```
