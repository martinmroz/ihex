//
// Copyright 2016 ihex Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

#[derive(PartialEq, Eq, Debug, Clone, Hash)]
pub enum Record {
    /// Specifies a 16-bit offset address and up to 255 bytes of data.
    /// Availability: I8HEX, I16HEX and I32HEX.
    Data {
        /// The offset of the data record in memory.
        offset: u16,
        /// Up to 255 bytes of data to be written to memory.
        value: Vec<u8>,
    },

    /// Indicates the end of the object file. Must occur exactly once per file, at the end.
    /// Availability: I8HEX, I16HEX and I32HEX.
    EndOfFile,

    /// Specifies bits 4-19 of the Segment Base Address (SBA) to address up to 1MiB.
    /// Availability: I16HEX.
    ExtendedSegmentAddress(u16),

    /// Specifies the 20-bit segment address via the CS and IP registers.
    /// Availability: I16HEX.
    StartSegmentAddress {
        /// Value of the CS register.
        cs: u16,
        /// Value of the IP register.
        ip: u16,
    },

    /// Specifies the upper 16 bits of a 32-bit linear address.
    /// The lower 16 bits are derived from the Data record load offset.
    /// Availability: I32HEX.
    ExtendedLinearAddress(u16),

    /// Specifies the execution start address for the object file.
    /// This is the 32-bit linear address for register EIP.
    /// Availability: I32HEX.
    StartLinearAddress(u32),
}

impl Record {
    ///
    /// The record type specifier corresponding to the receiver.
    ///
    pub fn record_type(&self) -> u8 {
        match self {
            Record::Data { .. } => types::DATA,
            Record::EndOfFile => types::END_OF_FILE,
            Record::ExtendedSegmentAddress(..) => types::EXTENDED_SEGMENT_ADDRESS,
            Record::StartSegmentAddress { .. } => types::START_SEGMENT_ADDRESS,
            Record::ExtendedLinearAddress(..) => types::EXTENDED_LINEAR_ADDRESS,
            Record::StartLinearAddress(..) => types::START_LINEAR_ADDRESS,
        }
    }
}

pub mod types {
    /// Type specifier for a Data record.
    pub const DATA: u8 = 0x00;
    /// Type specifier for an End-Of-File record.
    pub const END_OF_FILE: u8 = 0x01;
    /// Type specifier for an Extended Segment Address record.
    pub const EXTENDED_SEGMENT_ADDRESS: u8 = 0x02;
    /// Type specifier for a Start Segment Address record.
    pub const START_SEGMENT_ADDRESS: u8 = 0x03;
    /// Type specifier for an Extended Linear Address record.
    pub const EXTENDED_LINEAR_ADDRESS: u8 = 0x04;
    /// Type specifier for a Start Linear Address record.
    pub const START_LINEAR_ADDRESS: u8 = 0x05;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_record_type() {
        let data_record = Record::Data {
            offset: 0u16,
            value: Vec::new(),
        };
        assert_eq!(data_record.record_type(), 0x00);

        let eof_record = Record::EndOfFile;
        assert_eq!(eof_record.record_type(), 0x01);

        let extended_segment_address_record = Record::ExtendedSegmentAddress(0);
        assert_eq!(extended_segment_address_record.record_type(), 0x02);

        let start_segment_address_record = Record::StartSegmentAddress { cs: 0, ip: 0 };
        assert_eq!(start_segment_address_record.record_type(), 0x03);

        let extended_linear_address_record = Record::ExtendedLinearAddress(0);
        assert_eq!(extended_linear_address_record.record_type(), 0x04);

        let start_linear_address_record = Record::StartLinearAddress(0);
        assert_eq!(start_linear_address_record.record_type(), 0x05);
    }
}
