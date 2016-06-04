

extern crate ihex;

use ihex::record::Record;
use ihex::reader::*;

#[test]
fn test_record_from_record_string_parses_valid_data_records() {
  assert_eq!(Record::from_record_string(":0B0010006164647265737320676170A7"),
    Ok(Record::Data {
      offset: 0x0010,
       value: vec![0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x20, 0x67, 0x61, 0x70]
    })
  );

  assert_eq!(Record::from_record_string(":00FFFE0003"),
    Ok(Record::Data {
      offset: 0xFFFE,
       value: vec![]
    })
  );
}

#[test]
fn test_record_from_record_string_parses_valid_eof_record() {
  assert_eq!(Record::from_record_string(":00000001FF"), Ok(Record::EndOfFile));
}

#[test]
fn test_record_from_record_string_parses_valid_extended_segment_address() {
  assert_eq!(Record::from_record_string(":0200000212FEEC"), Ok(Record::ExtendedSegmentAddress(0x12FE)));
}

#[test]
fn test_record_from_record_string_parses_valid_start_segment_address() {
  assert_eq!(Record::from_record_string(":04000003123438007B"),
    Ok(Record::StartSegmentAddress {
      cs: 0x1234,
      ip: 0x3800
    })
  );
}

#[test]
fn test_record_from_record_string_parses_valid_extended_linear_address() {
  assert_eq!(Record::from_record_string(":02000004ABCD82"), Ok(Record::ExtendedLinearAddress(0xABCD)));
}

#[test]
fn test_record_from_record_string_parses_valid_start_linear_address() {
  assert_eq!(Record::from_record_string(":0400000512345678E3"), Ok(Record::StartLinearAddress(0x12345678)));
}
