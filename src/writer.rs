
use record::*;
use checksum::*;

impl ToString for Record {
  /**
   @return The IHEX record representation of the receiver as a String.
   */
  fn to_string(&self) -> String {
    match self {
      &Record::Data { offset, ref value } =>
        format_record(types::DATA, offset, value.as_slice()),

      &Record::EndOfFile =>
        format_record(types::END_OF_FILE, 0x0000, &[]),

      &Record::ExtendedSegmentAddress(address) =>
        format_record(types::EXTENDED_SEGMENT_ADDRESS, 0x0000, &[
          ((address & 0xFF00) >> 8) as u8,
          ((address & 0x00FF) >> 0) as u8
        ]),

      &Record::StartSegmentAddress { cs, ip } =>
        format_record(types::START_SEGMENT_ADDRESS, 0x0000, &[
          ((cs & 0xFF00) >> 8) as u8,
          ((cs & 0x00FF) >> 0) as u8,
          ((ip & 0xFF00) >> 8) as u8,
          ((ip & 0x00FF) >> 0) as u8
        ]),

      &Record::ExtendedLinearAddress(address) =>
        format_record(types::EXTENDED_LINEAR_ADDRESS, 0x0000, &[
          ((address & 0xFF00) >> 8) as u8,
          ((address & 0x00FF) >> 0) as u8
        ]),

      &Record::StartLinearAddress(address) => 
        format_record(types::START_LINEAR_ADDRESS, 0x0000, &[
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
fn format_record(record_type: u8, address: u16, data: &[u8]) -> String {
  assert!(data.len() <= 0xFF);

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
  data_region
    .iter()
    .map(|&byte| format!("{:02X}", byte))
    .fold(result, |mut acc, ref byte_string| { 
      acc.push_str(byte_string); 
      acc 
    })
}

#[cfg(test)]
mod tests {

  use record::Record;

  #[test]
  fn test_record_to_string_for_data_record() {
    let empty_data_record = Record::Data { offset: 0x0000, value: vec![] };
    assert_eq!(empty_data_record.to_string(), String::from(":0000000000"));

    let data = vec![0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x20, 0x67, 0x61, 0x70];
    let populated_data_record = Record::Data { offset: 0x0010, value: data };
    assert_eq!(populated_data_record.to_string(), String::from(":0B0010006164647265737320676170A7"));

    // Validating that the maximum length data record will not panic on serialization.
    let max_length_data = (0..255).map(|_| 0u8).collect::<Vec<u8>>();
    let max_length_data_record = Record::Data { offset: 0x0000, value: max_length_data };
    let _ = max_length_data_record.to_string();
  }

  #[test]
  #[should_panic]
  fn test_record_to_string_for_data_record_with_invalid_data() {
    let invalid_data = (0..256).map(|_| 0u8).collect::<Vec<u8>>();
    let invalid_data_record = Record::Data { offset: 0x0010, value: invalid_data };
    let _ = invalid_data_record.to_string();
  }

  #[test]
  fn test_record_to_string_for_eof_record() {
    let eof_record = Record::EndOfFile;
    assert_eq!(eof_record.to_string(), String::from(":00000001FF"));
  }

  #[test]
  fn test_record_to_string_for_esa_record() {
    let esa_record_1 = Record::ExtendedSegmentAddress(0x1200);
    assert_eq!(esa_record_1.to_string(), String::from(":020000021200EA"));

    let esa_record_2 = Record::ExtendedSegmentAddress(0x55AA);
    assert_eq!(esa_record_2.to_string(), String::from(":0200000255AAFD"));
  }

  #[test]
  fn test_record_to_string_for_ssa_record() {
    let ssa_record_1 = Record::StartSegmentAddress { cs: 0x0110, ip: 0x3801 };
    assert_eq!(ssa_record_1.to_string(), String::from(":0400000301103801AF"));

    let ssa_record_2 = Record::StartSegmentAddress { cs: 0x0000, ip: 0x3800 };
    assert_eq!(ssa_record_2.to_string(), String::from(":0400000300003800C1"));
  }

  #[test]
  fn test_record_to_string_for_ela_record() {
    let ela_record_1 = Record::ExtendedLinearAddress(0xFFFF);
    assert_eq!(ela_record_1.to_string(), String::from(":02000004FFFFFC"));

    let ela_record_2 = Record::ExtendedLinearAddress(0x0F55);
    assert_eq!(ela_record_2.to_string(), String::from(":020000040F5596"));
  }

  #[test]
  fn test_record_to_string_for_sla_record() {
    let sla_record_1 = Record::StartLinearAddress(0x000000CD);
    assert_eq!(sla_record_1.to_string(), String::from(":04000005000000CD2A"));

    let sla_record_2 = Record::StartLinearAddress(0x11223344);
    assert_eq!(sla_record_2.to_string(), String::from(":04000005112233444D"));
  }

}
