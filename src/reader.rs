
use std::error::Error;
use std::fmt;
use std::str;

use record::*;
use checksum::*;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum ReaderError {
  /// A record string does not being with a ':'.
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
  ChecksumMismatch,
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
      &ReaderError::ChecksumMismatch          => "The checksum for the record does not match",
      &ReaderError::PayloadLengthMismatch     => "The length of the payload does not match the length field",
      &ReaderError::UnsupportedRecordType(_)  => "The record specifies an unsupported IHEX record type",
      &ReaderError::InvalidLengthForType      => "The payload length is invalid for the IHEX record type"
    }
  }
}

impl fmt::Display for ReaderError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Failed to parse IHEX record: {}.", self.description())
  }
}

/// The smallest record (excluding start code) is Byte Count + Address + Record Type + Checksum.
const SMALLEST_RECORD_CHAR_COUNT: usize = (5 * 2);

/// The smallest record (excluding start code) {Smallest} + a 255 byte payload region.
const LARGEST_RECORD_CHAR_COUNT: usize = SMALLEST_RECORD_CHAR_COUNT + (255 * 2);

impl Record {
  /**
   Parses a given IHEX string representation of a Record and 
   @param string The IHEX string representation of the record.
   @return The Record corresponding to the IHEX string representation.
   */
  pub fn from_record_string(string: &str) -> Result<Self, ReaderError> {
    if let Some(':') = string.chars().next() {} else {
      return Err(ReaderError::MissingStartCode);
    }

    let data_portion = &string[1..];
    let data_poriton_length = data_portion.chars().count();

    // Basic sanity-checking the input record string.
    if (data_poriton_length % 2) != 0 {
      return Err(ReaderError::RecordNotEvenLength);
    } else if data_poriton_length < SMALLEST_RECORD_CHAR_COUNT {
      return Err(ReaderError::RecordTooShort);
    } else if data_poriton_length > LARGEST_RECORD_CHAR_COUNT {
      return Err(ReaderError::RecordTooLong);
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
    let validated_region_bytes = &(data_bytes.as_slice()[1 .. data_bytes.len()-1]);
    let expected_checksum = *data_bytes.last().unwrap();
    let checksum = checksum(validated_region_bytes);

    // The read is failed if the checksum does not match.
    if checksum != expected_checksum {
      return Err(ReaderError::ChecksumMismatch);
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
          0 => Ok(Record::EndOfFile),
          _ => Err(ReaderError::InvalidLengthForType)
        }
      }

      types::EXTENDED_SEGMENT_ADDRESS => {
        match payload_bytes.len() {
          2 => {
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
          4 => {
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
          2 => {
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
          4 => {
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
