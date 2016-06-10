//
// Copyright 2016 The IHEX Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>.
// All files in the project carrying such notice may not be copied, modified, or 
// distributed except according to those terms.
//

//! # The IHEX Library.
//!
//! A Rust library for parsing and generating Intel HEX (or IHEX) objects. 
//! This format is commonly used for representing compiled program code and 
//! data to be loaded into a microcontroller, flash memory or ROM.

pub mod checksum;
pub mod reader;
pub mod record;
pub mod writer;
