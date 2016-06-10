
use std::error::Error;
use std::fmt;

use checksum::*;
use record::*;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum WriterError {
  /// A record contains data too large to represent.
  DataExceedsMaximumLength(usize),
  /// Object does not end in an EoF record.
  MissingEndOfFileRecord,
}

impl Error for WriterError {
  fn description(&self) -> &str {
    match self {
      &WriterError::DataExceedsMaximumLength(_) => "Record contains data exceeding 255 bytes.",
      &WriterError::MissingEndOfFileRecord      => "Object files must end with an End of File Record.",
    }
  }
}

impl fmt::Display for WriterError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Failed to generate IHEX record: {}.", self.description())
  }
}

impl Record {
  /**
   @return The IHEX record representation of the receiver, or an error on failure.
   */
  pub fn to_string(&self) -> Result<String,WriterError> {
    match self {
      &Record::Data { offset, ref value } =>
        format_record(self.record_type(), offset, value.as_slice()),

      &Record::EndOfFile =>
        format_record(self.record_type(), 0x0000, &[]),

      &Record::ExtendedSegmentAddress(address) =>
        format_record(self.record_type(), 0x0000, &[
          ((address & 0xFF00) >> 8) as u8,
          ((address & 0x00FF) >> 0) as u8
        ]),

      &Record::StartSegmentAddress { cs, ip } =>
        format_record(self.record_type(), 0x0000, &[
          ((cs & 0xFF00) >> 8) as u8,
          ((cs & 0x00FF) >> 0) as u8,
          ((ip & 0xFF00) >> 8) as u8,
          ((ip & 0x00FF) >> 0) as u8
        ]),

      &Record::ExtendedLinearAddress(address) =>
        format_record(self.record_type(), 0x0000, &[
          ((address & 0xFF00) >> 8) as u8,
          ((address & 0x00FF) >> 0) as u8
        ]),

      &Record::StartLinearAddress(address) => 
        format_record(self.record_type(), 0x0000, &[
          ((address & 0xFF000000) >> 24) as u8,
          ((address & 0x00FF0000) >> 16) as u8,
          ((address & 0x0000FF00) >>  8) as u8,
          ((address & 0x000000FF) >>  0) as u8
        ])

    }
  }
}

/**
 IHEX records all contain the following fields:
 +-----+------------+--------------+----------+------------+-------------+
 | ':' | Length: u8 | Address: u16 | Type: u8 | Data: [u8] | Checkum: u8 |
 +-----+------------+--------------+----------+------------+-------------+
 Any multi-byte values are represented big endian.
 @note This method will panic if data is more than 255 bytes long.
 @return Formatted IHEX record.
 */
fn format_record(record_type: u8, address: u16, data: &[u8]) -> Result<String,WriterError> {
  if data.len() > 0xFF {
    return Err(WriterError::DataExceedsMaximumLength(data.len()));
  }

  // Allocate space for the data region (everything but the start code).
  let data_length = 1 + 2 + 1 + data.len() + 1;
  let mut data_region = Vec::<u8>::with_capacity(data_length);

  // Build the record (excluding start code) up to the checksum.
  data_region.push(data.len() as u8);
  data_region.push(((address & 0xFF00) >> 8) as u8);
  data_region.push(((address & 0x00FF) >> 0) as u8);
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
  let record_string = data_region
    .iter()
    .map(|&byte| format!("{:02X}", byte))
    .fold(result, |mut acc, ref byte_string| { 
      acc.push_str(byte_string); 
      acc 
    });

  Ok(record_string)
}

/**
 Generates an Intel HEX object file representation of the record set. It is the callers 
 responsibility to ensure that no overlapping data ranges are defined within the
 object file. The set of records MUST exactly 1 EOF record as the last record in the list.
 As with Record::to_string() this function may panic if a data record specifies
 more than 255 bytes of data.
 @param records Set of records to include in the object file representation.
 @return Some(_) if the an object file representation was built successfully, or None.
 */
pub fn create_object_file_representation(records: &[Record]) -> Result<String,WriterError> {
  if let Some(&Record::EndOfFile) = records.last() {} else {
    return Err(WriterError::MissingEndOfFileRecord);
  }

  records
    .iter()
    .map(|ref record| record.to_string())
    .collect::<Result<Vec<String>, WriterError>>()
    .map(|list| list.join("\n"))
}
