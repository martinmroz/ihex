ihex
====

A Rust library for parsing and generating [Intel HEX](https://en.wikipedia.org/wiki/Intel_HEX) 
(or IHEX) objects. This format is commonly used for representing compiled program code
and data to be loaded into a microcontroller, flash memory or ROM.

[![](http://meritbadge.herokuapp.com/ihex)](https://crates.io/crates/ihex)

### Usage

Add the following to your `Cargo.toml`:

```toml
[dependencies]
ihex = "0.1"
```

In addition, and this to your crate:

```rust
extern crate ihex;
```

# License

`ihex` is distributed under the terms of the MIT license.

See LICENSE for details.
