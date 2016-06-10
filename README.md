ihex
====

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
ihex = "0.1"
```

In addition, and this to your crate root:

```rust
extern crate ihex;
```

Here is an example which builds an IHEX object file with test data and prints it:


```rust
extern crate ihex;

use ihex::record::Record;
use ihex::writer;

fn main() {
  let records = &[
    Record::Data { offset: 0x0010, value: vec![0x48,0x65,0x6C,0x6C,0x6F] },
    Record::EndOfFile
  ];

  let result = writer::create_object_file_representation(records);
  if result.is_ok() {
    println!("{}", result.unwrap());
  }
}
```

# License

`ihex` is distributed under the terms of the MIT license.

See LICENSE for details.
