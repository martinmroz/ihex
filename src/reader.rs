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
use std::iter::FusedIterator;
use std::str;

use crate::checksum::checksum;
use crate::record::{types, Record};

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub enum ReaderError {
    /// The record provided does not begin with a ':'.
    MissingStartCode,
    /// The record provided is shorter than the smallest valid.
    RecordTooShort,
    /// The record provided exceeds the maximum size (255b payload).
    RecordTooLong,
    /// The record is not an even number of bytes.
    RecordNotEvenLength,
    /// The record is not all hexadecimal characters.
    ContainsInvalidCharacters,
    /// The checksum did not match.
    ChecksumMismatch(u8, u8),
    /// The record is not the length it claims.
    PayloadLengthMismatch,
    /// The record type is not supported.
    UnsupportedRecordType(u8),
    /// The payload length does not match the record type.
    InvalidLengthForType,
}

impl Error for ReaderError {}

impl fmt::Display for ReaderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ReaderError::MissingStartCode => write!(f, "missing start code ':'"),
            ReaderError::RecordTooShort => write!(f, "too short"),
            ReaderError::RecordTooLong => write!(f, "too long"),
            ReaderError::RecordNotEvenLength => {
                write!(f, "record does not contain a whole number of bytes")
            }
            ReaderError::ContainsInvalidCharacters => {
                write!(f, "invalid characters encountered in record")
            }
            ReaderError::ChecksumMismatch(found, expecting) => write!(
                f,
                "invalid checksum '{:02X}', expecting '{:02X}'",
                found, expecting,
            ),
            ReaderError::PayloadLengthMismatch => {
                write!(f, "payload length does not match record header")
            }
            ReaderError::UnsupportedRecordType(record_type) => {
                write!(f, "unsupported IHEX record type '{:02X}'", record_type)
            }
            ReaderError::InvalidLengthForType => {
                write!(f, "payload length invalid for record type")
            }
        }
    }
}

mod char_counts {
    /// The smallest record (excluding start code) is Byte Count + Address + Record Type + Checksum.
    pub const SMALLEST_RECORD_EXCLUDING_START_CODE: usize = (1 + 2 + 1 + 1) * 2;
    /// The smallest record (excluding start code) {Smallest} + a 255 byte payload region.
    pub const LARGEST_RECORD_EXCLUDING_START_CODE: usize = (1 + 2 + 1 + 255 + 1) * 2;
}

mod payload_sizes {
    /// An EoF record has no payload.
    pub const END_OF_FILE: usize = 0;
    /// An Extended Segment Address has a 16-bit payload.
    pub const EXTENDED_SEGMENT_ADDRESS: usize = 2;
    /// An Start Segment Address has two 16-bit payloads.
    pub const START_SEGMENT_ADDRESS: usize = 4;
    /// An Extended Linear Address has a 16-bit payload.
    pub const EXTENDED_LINEAR_ADDRESS: usize = 2;
    /// An Start Linear Address has a 32-bit payload.
    pub const START_LINEAR_ADDRESS: usize = 4;
}

impl Record {
    ///
    /// Constructs a new `Record` by parsing `string`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ihex::Record;
    ///
    /// let record = Record::from_record_string(":00000001FF").unwrap();
    /// ```
    ///
    pub fn from_record_string(string: &str) -> Result<Self, ReaderError> {
        if let Some(':') = string.chars().next() {
        } else {
            return Err(ReaderError::MissingStartCode);
        }

        let data_portion = &string[1..];
        let data_poriton_length = data_portion.chars().count();

        // Validate all characters are hexadecimal before checking the digit counts for more accurate errors.
        if !data_portion
            .chars()
            .all(|character| character.is_ascii_hexdigit())
        {
            return Err(ReaderError::ContainsInvalidCharacters);
        }

        // Basic sanity-checking the input record string.
        if data_poriton_length < char_counts::SMALLEST_RECORD_EXCLUDING_START_CODE {
            return Err(ReaderError::RecordTooShort);
        } else if data_poriton_length > char_counts::LARGEST_RECORD_EXCLUDING_START_CODE {
            return Err(ReaderError::RecordTooLong);
        } else if (data_poriton_length % 2) != 0 {
            return Err(ReaderError::RecordNotEvenLength);
        }

        // Convert the character stream to bytes.
        let mut data_bytes = data_portion
            .as_bytes()
            .chunks(2)
            .map(|chunk| str::from_utf8(chunk).unwrap())
            .map(|byte_str| u8::from_str_radix(byte_str, 16).unwrap())
            .collect::<Vec<u8>>();

        // Compute the checksum.
        let expected_checksum = data_bytes.pop().unwrap();
        let validated_region_bytes = data_bytes.as_slice();
        let checksum = checksum(validated_region_bytes);

        // The read is failed if the checksum does not match.
        if checksum != expected_checksum {
            return Err(ReaderError::ChecksumMismatch(checksum, expected_checksum));
        }

        // Decode header values.
        let length = validated_region_bytes[0];
        let address_hi = (validated_region_bytes[1] as u16) << 8;
        let address_lo = validated_region_bytes[2] as u16;
        let address = address_hi | address_lo;
        let record_type = validated_region_bytes[3];
        let payload_bytes = &validated_region_bytes[4..];

        // Validate the length of the record matches what was specified in the header.
        if payload_bytes.len() != (length as usize) {
            return Err(ReaderError::PayloadLengthMismatch);
        }

        match record_type {
            types::DATA => {
                // A Data record consists of an address and payload bytes.
                Ok(Record::Data {
                    offset: address,
                    value: Vec::from(payload_bytes),
                })
            }

            types::END_OF_FILE => {
                // An EoF record has no payload.
                match payload_bytes.len() {
                    payload_sizes::END_OF_FILE => Ok(Record::EndOfFile),

                    _ => Err(ReaderError::InvalidLengthForType),
                }
            }

            types::EXTENDED_SEGMENT_ADDRESS => {
                match payload_bytes.len() {
                    payload_sizes::EXTENDED_SEGMENT_ADDRESS => {
                        // The 16-bit extended segment address is encoded big-endian.
                        let address_hi = (payload_bytes[0] as u16) << 8;
                        let address_lo = payload_bytes[1] as u16;
                        let address = address_hi | address_lo;

                        Ok(Record::ExtendedSegmentAddress(address))
                    }

                    _ => Err(ReaderError::InvalidLengthForType),
                }
            }

            types::START_SEGMENT_ADDRESS => {
                match payload_bytes.len() {
                    payload_sizes::START_SEGMENT_ADDRESS => {
                        // The CS:IP pair is encoded as two 16-bit big-endian integers.
                        let cs_hi = (payload_bytes[0] as u16) << 8;
                        let cs_lo = payload_bytes[1] as u16;
                        let ip_hi = (payload_bytes[2] as u16) << 8;
                        let ip_lo = payload_bytes[3] as u16;
                        let cs = cs_hi | cs_lo;
                        let ip = ip_hi | ip_lo;

                        Ok(Record::StartSegmentAddress { cs, ip })
                    }

                    _ => Err(ReaderError::InvalidLengthForType),
                }
            }

            types::EXTENDED_LINEAR_ADDRESS => {
                match payload_bytes.len() {
                    payload_sizes::EXTENDED_LINEAR_ADDRESS => {
                        // The upper 16 bits of the linear address are encoded as a 16-bit big-endian integer.
                        let ela_hi = (payload_bytes[0] as u16) << 8;
                        let ela_lo = payload_bytes[1] as u16;
                        let ela = ela_hi | ela_lo;

                        Ok(Record::ExtendedLinearAddress(ela))
                    }

                    _ => Err(ReaderError::InvalidLengthForType),
                }
            }

            types::START_LINEAR_ADDRESS => {
                match payload_bytes.len() {
                    payload_sizes::START_LINEAR_ADDRESS => {
                        // The 32-bit value loaded into EIP is encoded as a 32-bit big-endian integer.
                        let sla_4 = (payload_bytes[0] as u32) << 24;
                        let sla_3 = (payload_bytes[1] as u32) << 16;
                        let sla_2 = (payload_bytes[2] as u32) << 8;
                        let sla_1 = payload_bytes[3] as u32;
                        let sla = sla_4 | sla_3 | sla_2 | sla_1;

                        Ok(Record::StartLinearAddress(sla))
                    }

                    _ => Err(ReaderError::InvalidLengthForType),
                }
            }

            _ => Err(ReaderError::UnsupportedRecordType(record_type)),
        }
    }
}

impl str::FromStr for Record {
    type Err = ReaderError;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Record::from_record_string(input)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct ReaderOptions {
    /// A flag indicating that iteration should stop on first failure.
    pub stop_after_first_error: bool,
    /// A flag indicating that iteration should stop on first EOF record encountered.
    pub stop_after_eof: bool,
}

impl Default for ReaderOptions {
    fn default() -> Self {
        ReaderOptions {
            stop_after_first_error: true,
            stop_after_eof: true,
        }
    }
}

pub struct Reader<'a> {
    /// Iterator over distinct lines of the input regardless of line ending.
    line_iterator: str::Lines<'a>,
    /// Reading may complete before the line iterator.
    finished: bool,
    /// Configuration options.
    options: ReaderOptions,
}

impl<'a> Reader<'a> {
    ///
    /// Creates a new IHEX reader over `string` with the specified configuration parameters. If
    /// `stop_after_first_error` is `true` then the first error will make all subsequent calls
    /// to `next()` return `None`. If `stop_after_eof` is `true` then the first EoF record
    /// will make all subsequent calls to `next()` return `None`.
    ///
    pub fn new_with_options(string: &'a str, options: ReaderOptions) -> Self {
        Reader {
            line_iterator: string.lines(),
            finished: false,
            options,
        }
    }

    ///
    /// Creates a new IHEX reader over `string` with default configuration parameters.
    ///
    pub fn new(string: &'a str) -> Self {
        Reader::new_with_options(string, Default::default())
    }

    ///
    /// Private helper method for obtaining the next record string, skipping empty lines.
    /// Increments the offset by the number of bytes processed. Does not respect the 'finished' flag.
    /// It will return either the next record string to be read, or None if nothing is left to process.
    ///
    fn next_record(&mut self) -> Option<&'a str> {
        let mut result = None;

        // Locate the first non-empty line.
        while let Some(line) = self.line_iterator.next() {
            if !line.is_empty() {
                result = Some(line);
                break;
            }
        }

        result
    }
}

impl<'a> Iterator for Reader<'a> {
    type Item = Result<Record, ReaderError>;

    ///
    /// Iterates over the lines of the IHEX object, skipping any empty ones,
    /// and returns the result of parsing that line.
    ///
    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }

        match self.next_record() {
            None => {
                self.finished = true;
                None
            }

            Some(line) => {
                let parse_result = str::parse::<Record>(line);

                // Check if iteration should end after a parse failure.
                if parse_result.is_err() && self.options.stop_after_first_error {
                    self.finished = true;
                }

                // Check if iteration should end after an EOF.
                if let Ok(Record::EndOfFile) = parse_result {
                    if self.options.stop_after_eof {
                        self.finished = true;
                    }
                }

                Some(parse_result)
            }
        }
    }
}

impl<'a> FusedIterator for Reader<'a> {}
