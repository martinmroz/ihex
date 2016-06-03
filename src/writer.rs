
use record::*;
use checksum::*;

impl ToString for Record {
  /**
   @return The IHEX record representation of the receiver as a String.
   */
  fn to_string(&self) -> String {
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

/**
 Generates an Intel HEX object file representation of the record set. It is the callers 
 responsibility to ensure that no overlapping data ranges are defined within the
 object file. The set of records MUST exactly 1 EOF record as the last record in the list.
 As with Record::to_string() this function may panic if a data record specifies
 more than 255 bytes of data.
 @param records Set of records to include in the object file representation.
 @return Some(_) if the an object file representation was built successfully, or None.
 */
pub fn create_object_file_representation(records: &[Record]) -> Option<String> {
  if let Some(&Record::EndOfFile) = records.last() {} else {
    return None;
  }

  let object_file_representation = 
    records
      .iter()
      .map(|ref record| record.to_string())
      .collect::<Vec<_>>()
      .join("\n");

  Some(object_file_representation)
}

#[cfg(test)]
mod tests {

  use record::Record;

  use super::*;

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

  #[test]
  fn test_create_object_file_representation_incorrect_termination() {
    assert_eq!(create_object_file_representation(&[]), None);
    assert_eq!(create_object_file_representation(&[Record::ExtendedLinearAddress(0)]), None);
    assert_eq!(create_object_file_representation(&[Record::EndOfFile, Record::ExtendedLinearAddress(0)]), None);
  }

  #[test]
  fn test_create_object_file_representation_eof_only() {
    let records = &[Record::EndOfFile];
    let expected_result = String::from(":00000001FF");
    assert_eq!(create_object_file_representation(records).unwrap(), expected_result);
  }

  #[test]
  fn test_create_object_file_representation_all_types() {
    let records = &[
      Record::Data { offset: 0x0010, value: vec![0x61,0x64,0x64,0x72,0x65,0x73,0x73,0x20,0x67,0x61,0x70] },
      Record::ExtendedSegmentAddress(0x1200),
      Record::StartSegmentAddress { cs: 0x0000, ip: 0x3800 },
      Record::ExtendedLinearAddress(0xFFFF),
      Record::StartLinearAddress(0x000000CD),
      Record::EndOfFile
    ];

    let expected_result = String::new() +
      &":0B0010006164647265737320676170A7\n" +
      &":020000021200EA\n" +
      &":0400000300003800C1\n" +
      &":02000004FFFFFC\n" +
      &":04000005000000CD2A\n" +
      &":00000001FF";

    assert_eq!(create_object_file_representation(records).unwrap(), expected_result);
  }

}
