# light-morse

A library for Morse code implementation in Rust.

[![Travis](https://img.shields.io/travis/luki/light-morse.svg)](https://travis-ci.org/luki/light-morse)
[![Crates.io](https://img.shields.io/crates/v/light-morse.svg)](https://crates.io/crates/light-morse)
[![license](https://img.shields.io/github/license/luki/light-morse.svg)](https://github.com/luki/light-morse/blob/master/LICENSE)

[Documentation](https://docs.rs/light-morse/0.2.0/light_morse/)

## Getting Started
The following instructions will explain how to get any machine up-and-running for Rust
and how to subsequently implement the library into a project.

### Prerequisties

In order to run Rust on a local machine, the language has to be installed.
A detailed description of how that is done, can be found on the [official Rust page](https://www.rust-lang.org/en-US/install.html).


### Implementation/Embedment

Add this to your `Cargo.toml`:

```toml
[dependencies]
light-morse = "0.2.0"
```

and this to your crate root:

```rust
extern crate light_morse;

use light_morse::*;
```

## Tests

Enter the light-morse repository and run: `cargo test --verbose`

## Telegram-CLI

This is a full-functioning Telegram, playing sounds for the word(s) one writes.

### How To Use

1. Go into the /examples/telegram directory.
2. Run `telegram --message "[MESSAGE]" --morse-type "ITU, Gerke or Morse"`.

*Note*: `telegram` is `cargo run --`

### Dependencies

* [clap](https://github.com/kbknapp/clap-rs) - Command Line Argument Parser
* [ears](https://github.com/jhasse/ears) - Play sounds and music
* [itertools](https://github.com/bluss/rust-itertools) - Extra iterator adaptors, functions and macros

### Sound Effects

Cut as needed, originals by [soundsnap](soundsnap.com).

## Building On

Everything regarding how to build upon light-morse.

### Adding Substitution / Morse Method

A substitution method can be added, as long as it looks like the following:

```Rust
fn name(chr: PlainChunk) -> MorseChunk { /* Substitution for chars */ }
```

* PlainChunk: *char*
* MorseChunk: *String*

## Versioning

[SemVer](http://semver.org/) applies for versioning. For the versions available, see the [crate](https://crates.io/crates/light-morse).

## Authors

* [Lukas Mueller](https://github.com/luki) - Initial Work

## Acknowledgements

The topic for this library was inspired by the morse iOS application of [Ilias Ennmouri](https://github.com/iiias).
