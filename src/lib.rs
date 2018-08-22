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

/// Function for computing IHEX checksum.
pub mod checksum;

/// Operations for parsing IHEX records and object files.
pub mod reader;

/// An Intel HEX record type.
pub mod record;

/// Operations for generating IHEX records and object files.
pub mod writer;
