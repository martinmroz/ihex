Intel HEX (IHEX) Library
========================

* [Documentation](https://martinmroz.github.io/ihex/master/ihex/)

A Rust library for parsing and generating [Intel HEX](https://en.wikipedia.org/wiki/Intel_HEX) 
(or IHEX) objects. This format is commonly used for representing compiled program code
and data to be loaded into a microcontroller, flash memory or ROM.

[![](http://meritbadge.herokuapp.com/ihex)](https://crates.io/crates/ihex)
[![Build Status](https://travis-ci.org/martinmroz/ihex.svg?branch=master)](https://travis-ci.org/martinmroz/ihex)
[![Coverage Status](https://coveralls.io/repos/github/martinmroz/ihex/badge.svg?branch=master)](https://coveralls.io/github/martinmroz/ihex?branch=master)

### Usage

Add the following to your `Cargo.toml`:

```toml
[dependencies]
ihex = "3.0"
```

In addition, for Rust 2015 edition projects, and this to your crate root:

```rust
extern crate ihex;
```

Here is an example which builds an IHEX object file with test data and prints it:


```rust
use ihex::Record;

fn main() {
    let records = &[
        Record::Data { offset: 0x0010, value: vec![0x48,0x65,0x6C,0x6C,0x6F] },
        Record::EndOfFile
    ];

    if let Ok(object) = ihex::create_object_file_representation(records) {
        println!("{}", object);
    }
}
```

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
