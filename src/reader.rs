//
// Copyright 2016 The IHEX Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>.
// All files in the project carrying such notice may not be copied, modified, or 
// distributed except according to those terms.
//

use std::error::Error;
use std::fmt;
use std::str;

use checksum::*;
use record::*;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
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
  ChecksumMismatch(u8,u8),
  /// The record is not the length it claims.
  PayloadLengthMismatch,
  /// The record type is not supported.
  UnsupportedRecordType(u8),
  /// The payload length does not match the record type.
  InvalidLengthForType,
}

impl Error for ReaderError {
  fn description(&self) -> &str {
    match self {
      &ReaderError::MissingStartCode          => "Record does not being with a Start Code (':')",
      &ReaderError::RecordTooShort            => "Record string is shorter than the smallest valid record",
      &ReaderError::RecordTooLong             => "Record string is longer than the longest valid record",
      &ReaderError::RecordNotEvenLength       => "Record does not contain a whole number of bytes",
      &ReaderError::ContainsInvalidCharacters => "Record contains invalid characters",
      &ReaderError::ChecksumMismatch(_,_)     => "The checksum for the record does not match",
      &ReaderError::PayloadLengthMismatch     => "The length of the payload does not match the length field",
      &ReaderError::UnsupportedRecordType(_)  => "The record specifies an unsupported IHEX record type",
      &ReaderError::InvalidLengthForType      => "The payload length is invalid for the IHEX record type",
    }
  }
}

impl fmt::Display for ReaderError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Failed to parse IHEX record: {}.", self.description())
  }
}

/// The smallest record (excluding start code) is Byte Count + Address + Record Type + Checksum.
const SMALLEST_RECORD_CHAR_COUNT: usize = (1 + 2 + 1 + 1) * 2;

/// The smallest record (excluding start code) {Smallest} + a 255 byte payload region.
const LARGEST_RECORD_CHAR_COUNT: usize = SMALLEST_RECORD_CHAR_COUNT + (255 * 2);

pub mod payload_sizes {
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
  /// use ihex::record::Record;
  ///
  /// let record = Record::from_record_string(":");
  /// ```
  ///
  pub fn from_record_string(string: &str) -> Result<Self, ReaderError> {
    if let Some(':') = string.chars().next() {} else {
      return Err(ReaderError::MissingStartCode);
    }

    let data_portion = &string[1 .. ];
    let data_poriton_length = data_portion.chars().count();

    // Basic sanity-checking the input record string.
    if data_poriton_length < SMALLEST_RECORD_CHAR_COUNT {
      return Err(ReaderError::RecordTooShort);
    } else if data_poriton_length > LARGEST_RECORD_CHAR_COUNT {
      return Err(ReaderError::RecordTooLong);
    } else if (data_poriton_length % 2) != 0 {
      return Err(ReaderError::RecordNotEvenLength);
    }

    // Validate all characters are hexadecimal.
    if data_portion.chars().all(|character| character.is_digit(16)) == false {
      return Err(ReaderError::ContainsInvalidCharacters);
    }

    // Convert the character stream to bytes.
    let data_bytes = 
      data_portion
        .as_bytes()
        .chunks(2)
        .map(|chunk| str::from_utf8(chunk).unwrap())
        .map(|byte_str| u8::from_str_radix(byte_str, 16).unwrap())
        .collect::<Vec<u8>>();

    // Compute the checksum.
    let validated_region_bytes = &(data_bytes.as_slice()[0 .. data_bytes.len()-1]);
    let expected_checksum = *data_bytes.last().unwrap();
    let checksum = checksum(validated_region_bytes);

    // The read is failed if the checksum does not match.
    if checksum != expected_checksum {
      return Err(ReaderError::ChecksumMismatch(checksum, expected_checksum));
    }

    // Decode header values.
    let length: u8 = validated_region_bytes[0];
    let address: u16 = 
      ((validated_region_bytes[1] as u16) << 8) |
      ((validated_region_bytes[2] as u16) << 0);
    let record_type: u8 = validated_region_bytes[3];
    let payload_bytes = &validated_region_bytes[4 .. ];

    // Validate the length of the record matches what was specified in the header.
    if payload_bytes.len() != (length as usize) {
      return Err(ReaderError::PayloadLengthMismatch);
    }

    match record_type {
      types::DATA => {
        // A Data record consists of an address and payload bytes.
        Ok(Record::Data { offset: address, value: Vec::from(payload_bytes) })
      }

      types::END_OF_FILE => {
        // An EoF record has no payload.
        match payload_bytes.len() {
          payload_sizes::END_OF_FILE => Ok(Record::EndOfFile),
          _ => Err(ReaderError::InvalidLengthForType)
        }
      }

      types::EXTENDED_SEGMENT_ADDRESS => {
        match payload_bytes.len() {
          payload_sizes::EXTENDED_SEGMENT_ADDRESS => {
            // The 16-bit extended segment address is encoded big-endian.
            let address_hi = (payload_bytes[0] as u16) << 8;
            let address_lo = (payload_bytes[1] as u16) << 0;
            let address = address_hi | address_lo;

            Ok( Record::ExtendedSegmentAddress(address) )
          }

          _ => Err(ReaderError::InvalidLengthForType)
        }
      }

      types::START_SEGMENT_ADDRESS => {
        match payload_bytes.len() {
          payload_sizes::START_SEGMENT_ADDRESS => {
            // The CS:IP pair is encoded as two 16-bit big-endian integers.
            let cs_hi = (payload_bytes[0] as u16) << 8;
            let cs_lo = (payload_bytes[1] as u16) << 0;
            let ip_hi = (payload_bytes[2] as u16) << 8;
            let ip_lo = (payload_bytes[3] as u16) << 0;
            let cs = cs_hi | cs_lo;
            let ip = ip_hi | ip_lo;

            Ok( Record::StartSegmentAddress { cs: cs, ip: ip } )
          }

          _ => Err(ReaderError::InvalidLengthForType)
        }
      }

      types::EXTENDED_LINEAR_ADDRESS => {
        match payload_bytes.len() {
          payload_sizes::EXTENDED_LINEAR_ADDRESS => {
            // The upper 16 bits of the linear address are encoded as a 16-bit big-endian integer.
            let ela_hi = (payload_bytes[0] as u16) << 8;
            let ela_lo = (payload_bytes[1] as u16) << 0;
            let ela = ela_hi | ela_lo;

            Ok( Record::ExtendedLinearAddress(ela) )
          }

          _ => Err(ReaderError::InvalidLengthForType)
        }
      }

      types::START_LINEAR_ADDRESS => {
        match payload_bytes.len() {
          payload_sizes::START_LINEAR_ADDRESS => {
            // The 32-bit value loaded into EIP is encoded as a 32-bit big-endian integer.
            let sla_4 = (payload_bytes[0] as u32) << 24;
            let sla_3 = (payload_bytes[1] as u32) << 16;
            let sla_2 = (payload_bytes[2] as u32) <<  8;
            let sla_1 = (payload_bytes[3] as u32) <<  0;
            let sla = sla_4 | sla_3 | sla_2 | sla_1;

            Ok( Record::StartLinearAddress(sla) )
          }

          _ => Err(ReaderError::InvalidLengthForType)
        }
      }

      _ => {
        Err(ReaderError::UnsupportedRecordType(record_type))
      }
    }
  }
}

pub struct Reader<'a> {
  /// Input string.
  input:  &'a str,
  /// Current offset into the input string, in bytes.
  offset: usize,
  /// Reading may complete before the line iterator.
  finished: bool,
  /// A flag indicating that iteration should stop on first failure.
  stop_after_first_error: bool,
  /// A flag indicating that iteration should stop on first EOF record encountered.
  stop_after_eof: bool
}

impl<'a> Reader<'a> {

  ///
  /// Creates a new IHEX reader over `string` with the specified configuration parameters. If 
  /// `stop_after_first_error` is `true` then the first error will make all subsequent calls
  /// to `next()` return `None`. If `stop_after_eof` is `true` then the first EoF record
  /// will make all subsequent calls to `next()` return `None`.
  ///
  pub fn new_stopping_after_error_and_eof(string: &'a str, stop_after_first_error: bool, stop_after_eof: bool) -> Self {
    Reader {
      input: string,
      offset: 0,
      finished: false,
      stop_after_first_error: stop_after_first_error,
      stop_after_eof: stop_after_eof
    }
  }

  ///
  /// Creates a new IHEX reader over `string` with default configuration parameters.
  ///
  pub fn new(string: &'a str) -> Self {
    Reader::new_stopping_after_error_and_eof(string, true, true)
  }

  ///
  /// Private helper method for obtaining the next record string, skipping empty lines.
  /// Increments the offset by the number of bytes processed. Does not respect the 'finished' flag.
  /// It will return either the next record string to be read, or None if nothing is left to process.
  ///
  fn next_record(&mut self) -> Option<&'a str> {
    if self.offset >= self.input.len() {
      return None;
    }
    
    self.input[self.offset .. ]
      .split('\n')
      .inspect(|&x| self.offset += x.as_bytes().len() + '\n'.len_utf8())
      .skip_while(|x| x.len() == 0)
      .next()
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
        let parse_result = Record::from_record_string(line);

        // Check if iteration should end after a parse failure.
        if let Err(_) = parse_result {
          if self.stop_after_first_error {
            self.finished = true;
          }
        }

        // Check if iteration should end after an EOF.
        if let Ok(Record::EndOfFile) = parse_result {
          if self.stop_after_eof {
            self.finished = true;
          }
        }

        Some(parse_result)
      }
    }
  }

}
