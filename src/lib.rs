//
// Copyright 2016 ihex Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

//! # The IHEX Library
//!
//! A Rust library for parsing and generating Intel HEX (or IHEX) objects.
//! This format is commonly used for representing compiled program code and
//! data to be loaded into a microcontroller, flash memory or ROM.

mod checksum;
mod reader;
mod record;
mod writer;

pub use checksum::*;
pub use reader::*;
pub use record::*;
pub use writer::*;
