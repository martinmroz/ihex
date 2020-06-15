//
// Copyright 2016 ihex Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use std::error::Error;
use std::fmt;
use std::fmt::Write;

use crate::checksum::checksum;
use crate::record::Record;

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub enum WriterError {
    /// A record contains data too large to represent.
    DataExceedsMaximumLength(usize),
    /// Object does not end in an EoF record.
    MissingEndOfFileRecord,
    /// Object contains multiple EoF records.
    MultipleEndOfFileRecords(usize),
    /// Unable to synthesize record string.
    SynthesisFailed,
}

impl Error for WriterError {}

impl fmt::Display for WriterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            WriterError::DataExceedsMaximumLength(bytes) => {
                write!(f, "record has {} bytes (max 255)", bytes)
            }
            WriterError::MissingEndOfFileRecord => {
                write!(f, "object is missing end of file record")
            }
            WriterError::MultipleEndOfFileRecords(eofs) => {
                write!(f, "object contains {} end of file records", eofs)
            }
            WriterError::SynthesisFailed => {
                write!(f, "unable to write string representation of record")
            }
        }
    }
}

impl Record {
    ///
    /// Returns the IHEX record representation of the receiver, or an error on failure.
    ///
    pub fn to_record_string(&self) -> Result<String, WriterError> {
        match self {
            Record::Data { offset, value } => format_record(self.record_type(), *offset, value),

            Record::EndOfFile => format_record(self.record_type(), 0x0000, &[]),

            Record::ExtendedSegmentAddress(segment_address) => format_record(
                self.record_type(),
                0x0000,
                &[
                    ((segment_address & 0xFF00) >> 8) as u8,
                    (segment_address & 0x00FF) as u8,
                ],
            ),

            Record::StartSegmentAddress { cs, ip } => format_record(
                self.record_type(),
                0x0000,
                &[
                    ((cs & 0xFF00) >> 8) as u8,
                    (cs & 0x00FF) as u8,
                    ((ip & 0xFF00) >> 8) as u8,
                    (ip & 0x00FF) as u8,
                ],
            ),

            Record::ExtendedLinearAddress(linear_address) => format_record(
                self.record_type(),
                0x0000,
                &[
                    ((linear_address & 0xFF00) >> 8) as u8,
                    (linear_address & 0x00FF) as u8,
                ],
            ),

            Record::StartLinearAddress(address) => format_record(
                self.record_type(),
                0x0000,
                &[
                    ((address & 0xFF00_0000) >> 24) as u8,
                    ((address & 0x00FF_0000) >> 16) as u8,
                    ((address & 0x0000_FF00) >> 8) as u8,
                    (address & 0x0000_00FF) as u8,
                ],
            ),
        }
    }
}

///
/// IHEX records all contain the following fields:
/// `+-----+------------+--------------+----------+------------+-------------+`
/// `| ':' | Length: u8 | Address: u16 | Type: u8 | Data: [u8] | Checkum: u8 |`
/// `+-----+------------+--------------+----------+------------+-------------+`
/// Any multi-byte values are represented big endian.
/// Note that this method will fail if a data record is more than 255 bytes long.
/// This method returns a formatted IHEX record on success with the specified
/// `record_type`, `address` and `data` values. On failure, an error is returned.
///
fn format_record<T>(record_type: u8, address: u16, input: T) -> Result<String, WriterError>
where
    T: AsRef<[u8]>,
{
    let data = input.as_ref();
    if data.len() > 0xFF {
        return Err(WriterError::DataExceedsMaximumLength(data.len()));
    }

    // Allocate space for the data region (everything but the start code).
    let data_length = 1 + 2 + 1 + data.len() + 1;
    let mut data_region = Vec::<u8>::with_capacity(data_length);

    // Build the record (excluding start code) up to the checksum.
    data_region.push(data.len() as u8);
    data_region.push(((address & 0xFF00) >> 8) as u8);
    data_region.push((address & 0x00FF) as u8);
    data_region.push(record_type);
    data_region.extend_from_slice(data);

    // Compute the checksum of the data region thus far and append it.
    let checksum = checksum(data_region.as_slice());
    data_region.push(checksum);

    // The result string is twice as long as the record plus the start code.
    let result_length = 1 + (2 * data_length);
    let mut result = String::with_capacity(result_length);

    // Construct the record.
    result.push(':');
    data_region.iter().try_fold(result, |mut acc, byte| {
        write!(&mut acc, "{:02X}", byte)
            .map_err(|_| WriterError::SynthesisFailed)
            .map(|_| acc)
    })
}

///
/// Generates an Intel HEX object file representation of the `records` provided. It is the callers
/// responsibility to ensure that no overlapping data ranges are defined within the
/// object file. In addition, `records` must have contain 1 EoF record,
/// and it must be the last element in `records`.
///
/// # Example
///
/// ```rust
/// use ihex::Record;
///
/// let records = &[
///   Record::Data { offset: 0x0010, value: vec![0x48,0x65,0x6C,0x6C,0x6F] },
///   Record::EndOfFile
/// ];
///
/// let result = ihex::create_object_file_representation(records).unwrap();
/// ```
///
pub fn create_object_file_representation(records: &[Record]) -> Result<String, WriterError> {
    if let Some(Record::EndOfFile) = records.last() {
    } else {
        return Err(WriterError::MissingEndOfFileRecord);
    }

    // Validate exactly one EoF record exists.
    let eof_record_count = records
        .iter()
        .filter(|x| {
            if let Record::EndOfFile = x {
                true
            } else {
                false
            }
        })
        .count();
    if eof_record_count > 1 {
        return Err(WriterError::MultipleEndOfFileRecords(eof_record_count));
    }

    records.iter().try_fold(String::new(), |mut acc, record| {
        acc.push_str(&record.to_record_string()?);
        acc.push_str("\n");
        Ok(acc)
    })
}
