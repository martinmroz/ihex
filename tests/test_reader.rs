//
// Copyright 2016 The IHEX Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>.
// All files in the project carrying such notice may not be copied, modified, or
// distributed except according to those terms.
//

use ihex::*;

#[test]
fn test_record_from_record_string_rejects_missing_start_code() {
    assert_eq!(
        Record::from_record_string("00000001FF"),
        Err(ReaderError::MissingStartCode)
    );
}

#[test]
fn test_record_from_record_string_rejects_short_records() {
    assert_eq!(
        Record::from_record_string(":"),
        Err(ReaderError::RecordTooShort)
    );
    assert_eq!(
        Record::from_record_string(":00"),
        Err(ReaderError::RecordTooShort)
    );
    assert_eq!(
        Record::from_record_string(":00000001F"),
        Err(ReaderError::RecordTooShort)
    );
}

#[test]
fn test_record_from_record_string_rejects_long_records() {
    let longest_valid_data = (0..255).map(|_| 0u8).collect::<Vec<u8>>();
    let longest_valid_data_record = Record::Data {
        offset: 0x0010,
        value: longest_valid_data,
    };
    let longest_valid_string = longest_valid_data_record.to_record_string().unwrap();
    let shortest_invalid_string = longest_valid_string.clone() + &"0";

    assert_eq!(longest_valid_string.len(), 521);
    assert_eq!(
        Record::from_record_string(&longest_valid_string).is_ok(),
        true
    );

    assert_eq!(shortest_invalid_string.len(), 522);
    assert_eq!(
        Record::from_record_string(&shortest_invalid_string),
        Err(ReaderError::RecordTooLong)
    );
}

#[test]
fn test_record_from_record_string_rejects_odd_length_records() {
    assert_eq!(
        Record::from_record_string(":0B0010006164647265737320676170A7D"),
        Err(ReaderError::RecordNotEvenLength)
    );
    assert_eq!(
        Record::from_record_string(":00000001FFF"),
        Err(ReaderError::RecordNotEvenLength)
    );
    assert_eq!(
        Record::from_record_string(":0200000212FEECD"),
        Err(ReaderError::RecordNotEvenLength)
    );
    assert_eq!(
        Record::from_record_string(":04000003123438007BD"),
        Err(ReaderError::RecordNotEvenLength)
    );
    assert_eq!(
        Record::from_record_string(":02000004ABCD823"),
        Err(ReaderError::RecordNotEvenLength)
    );
    assert_eq!(
        Record::from_record_string(":0400000512345678E34"),
        Err(ReaderError::RecordNotEvenLength)
    );
}

#[test]
fn test_record_from_record_string_rejects_non_hex_characters() {
    assert_eq!(
        Record::from_record_string(":000000q1ff"),
        Err(ReaderError::ContainsInvalidCharacters)
    );
    assert_eq!(
        Record::from_record_string(":00000021f*"),
        Err(ReaderError::ContainsInvalidCharacters)
    );
    assert_eq!(
        Record::from_record_string(":^0000001FF"),
        Err(ReaderError::ContainsInvalidCharacters)
    );
    assert_eq!(
        Record::from_record_string(":™0000001FF"),
        Err(ReaderError::ContainsInvalidCharacters)
    );
}

#[test]
fn test_record_from_record_string_rejects_invalid_checksums() {
    assert_eq!(
        Record::from_record_string(":0B0010006164647265737320676170FF"),
        Err(ReaderError::ChecksumMismatch(0xA7, 0xFF))
    );
    assert_eq!(
        Record::from_record_string(":0000000100"),
        Err(ReaderError::ChecksumMismatch(0xFF, 0x00))
    );
    assert_eq!(
        Record::from_record_string(":020000021200EB"),
        Err(ReaderError::ChecksumMismatch(0xEA, 0xEB))
    );
    assert_eq!(
        Record::from_record_string(":04000003000038001C"),
        Err(ReaderError::ChecksumMismatch(0xC1, 0x1C))
    );
    assert_eq!(
        Record::from_record_string(":02000004FFFFFD"),
        Err(ReaderError::ChecksumMismatch(0xFC, 0xFD))
    );
    assert_eq!(
        Record::from_record_string(":04000005000001CD2A"),
        Err(ReaderError::ChecksumMismatch(0x29, 0x2A))
    );
}

#[test]
fn test_record_from_record_string_rejects_payload_length_mismatches() {
    assert_eq!(
        Record::from_record_string(":0C0010006164647265737320676170A6"),
        Err(ReaderError::PayloadLengthMismatch)
    );
    assert_eq!(
        Record::from_record_string(":000010006164647265737320676170B2"),
        Err(ReaderError::PayloadLengthMismatch)
    );
    assert_eq!(
        Record::from_record_string(":01000001FE"),
        Err(ReaderError::PayloadLengthMismatch)
    );
    assert_eq!(
        Record::from_record_string(":0F0000021200DD"),
        Err(ReaderError::PayloadLengthMismatch)
    );
    assert_eq!(
        Record::from_record_string(":0200000300003800C3"),
        Err(ReaderError::PayloadLengthMismatch)
    );
    assert_eq!(
        Record::from_record_string(":01000004FFFFFD"),
        Err(ReaderError::PayloadLengthMismatch)
    );
    assert_eq!(
        Record::from_record_string(":05000005000001CD28"),
        Err(ReaderError::PayloadLengthMismatch)
    );
}

#[test]
fn test_record_from_record_string_rejects_unsupported_record_types() {
    assert_eq!(
        Record::from_record_string(":0B0010066164647265737320676170A1"),
        Err(ReaderError::UnsupportedRecordType(0x06))
    );
    assert_eq!(
        Record::from_record_string(":0B0010FF6164647265737320676170A8"),
        Err(ReaderError::UnsupportedRecordType(0xFF))
    );
}

#[test]
fn test_record_from_record_string_rejects_invalid_lengths_for_types() {
    assert_eq!(
        Record::from_record_string(":01000001FFFF"),
        Err(ReaderError::InvalidLengthForType)
    );
    assert_eq!(
        Record::from_record_string(":0100000200FD"),
        Err(ReaderError::InvalidLengthForType)
    );
    assert_eq!(
        Record::from_record_string(":03000002FF1200EA"),
        Err(ReaderError::InvalidLengthForType)
    );
    assert_eq!(
        Record::from_record_string(":03000003003800C2"),
        Err(ReaderError::InvalidLengthForType)
    );
    assert_eq!(
        Record::from_record_string(":050000030000003800C0"),
        Err(ReaderError::InvalidLengthForType)
    );
    assert_eq!(
        Record::from_record_string(":01000004FFFC"),
        Err(ReaderError::InvalidLengthForType)
    );
    assert_eq!(
        Record::from_record_string(":03000004FFFFFFFC"),
        Err(ReaderError::InvalidLengthForType)
    );
    assert_eq!(
        Record::from_record_string(":030000050000CD2B"),
        Err(ReaderError::InvalidLengthForType)
    );
    assert_eq!(
        Record::from_record_string(":0500000500000000CD29"),
        Err(ReaderError::InvalidLengthForType)
    );
}

#[test]
fn test_record_from_record_string_parses_valid_data_records() {
    assert_eq!(
        Record::from_record_string(":0B0010006164647265737320676170A7"),
        Ok(Record::Data {
            offset: 0x0010,
            value: vec![0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x20, 0x67, 0x61, 0x70,],
        })
    );

    assert_eq!(
        Record::from_record_string(":00FFFE0003"),
        Ok(Record::Data {
            offset: 0xFFFE,
            value: vec![],
        })
    );
}

#[test]
fn test_record_from_record_string_parses_valid_eof_record() {
    assert_eq!(
        Record::from_record_string(":00000001FF"),
        Ok(Record::EndOfFile)
    );
    assert_eq!(
        Record::from_record_string(":00000001ff"),
        Ok(Record::EndOfFile)
    );
}

#[test]
fn test_record_from_record_string_parses_valid_extended_segment_address() {
    assert_eq!(
        Record::from_record_string(":0200000212FEEC"),
        Ok(Record::ExtendedSegmentAddress(0x12FE))
    );
    assert_eq!(
        Record::from_record_string(":0200000212fEEc"),
        Ok(Record::ExtendedSegmentAddress(0x12FE))
    );
}

#[test]
fn test_record_from_record_string_parses_valid_start_segment_address() {
    assert_eq!(
        Record::from_record_string(":04000003123438007B"),
        Record::from_record_string(":04000003123438007b")
    );
    assert_eq!(
        Record::from_record_string(":04000003123438007B"),
        Ok(Record::StartSegmentAddress {
            cs: 0x1234,
            ip: 0x3800
        })
    );
}

#[test]
fn test_record_from_record_string_parses_valid_extended_linear_address() {
    assert_eq!(
        Record::from_record_string(":02000004ABCD82"),
        Ok(Record::ExtendedLinearAddress(0xABCD))
    );
    assert_eq!(
        Record::from_record_string(":02000004abcd82"),
        Ok(Record::ExtendedLinearAddress(0xABCD))
    );
}

#[test]
fn test_record_from_record_string_parses_valid_start_linear_address() {
    assert_eq!(
        Record::from_record_string(":0400000512345678E3"),
        Ok(Record::StartLinearAddress(0x12345678))
    );
    assert_eq!(
        Record::from_record_string(":0400000512345678e3"),
        Ok(Record::StartLinearAddress(0x12345678))
    );
}

#[test]
fn test_reader_processes_well_formed_ihex_object() {
    let input = String::new()
        + &":0B0010006164647265737320676170A7\n"
        + &":020000021200EA\n"
        + &":0400000300003800C1\n"
        + &":02000004FFFFFC\n"
        + &":04000005000000CD2A\n"
        + &":00000001FF";

    let data_rec = Record::Data {
        offset: 0x0010,
        value: vec![
            0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x20, 0x67, 0x61, 0x70,
        ],
    };
    let esa_rec = Record::ExtendedSegmentAddress(0x1200);
    let ssa_rec = Record::StartSegmentAddress {
        cs: 0x0000,
        ip: 0x3800,
    };
    let ela_rec = Record::ExtendedLinearAddress(0xFFFF);
    let sla_rec = Record::StartLinearAddress(0x000000CD);
    let eof_rec = Record::EndOfFile;

    let mut reader = Reader::new(&input);
    assert_eq!(reader.next(), Some(Ok(data_rec)));
    assert_eq!(reader.next(), Some(Ok(esa_rec)));
    assert_eq!(reader.next(), Some(Ok(ssa_rec)));
    assert_eq!(reader.next(), Some(Ok(ela_rec)));
    assert_eq!(reader.next(), Some(Ok(sla_rec)));
    assert_eq!(reader.next(), Some(Ok(eof_rec)));
    assert_eq!(reader.next(), None);
}

#[test]
fn test_reader_respects_stop_after_first_error_false() {
    let input =
        String::new() + &":0B0010006164647265737320676170A7\n" + &":\n" + &":0400000300003800C1\n";

    let data_rec = Record::Data {
        offset: 0x0010,
        value: vec![
            0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x20, 0x67, 0x61, 0x70,
        ],
    };
    let ssa_rec = Record::StartSegmentAddress {
        cs: 0x0000,
        ip: 0x3800,
    };

    let mut reader = Reader::new_with_options(
        &input,
        ReaderOptions {
            stop_after_first_error: false,
            stop_after_eof: false,
        },
    );
    assert_eq!(reader.next(), Some(Ok(data_rec)));
    assert_eq!(reader.next(), Some(Err(ReaderError::RecordTooShort)));
    assert_eq!(reader.next(), Some(Ok(ssa_rec)));
    assert_eq!(reader.next(), None);
}

#[test]
fn test_reader_respects_stop_after_first_error_true() {
    let input =
        String::new() + &":0B0010006164647265737320676170A7\n" + &":\n" + &":0400000300003800C1\n";

    let data_rec = Record::Data {
        offset: 0x0010,
        value: vec![
            0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x20, 0x67, 0x61, 0x70,
        ],
    };

    let mut reader = Reader::new_with_options(
        &input,
        ReaderOptions {
            stop_after_first_error: true,
            stop_after_eof: false,
        },
    );
    assert_eq!(reader.next(), Some(Ok(data_rec)));
    assert_eq!(reader.next(), Some(Err(ReaderError::RecordTooShort)));
    assert_eq!(reader.next(), None);
}

#[test]
fn test_reader_respects_stop_after_first_eof_false() {
    let input = String::new()
        + &":0B0010006164647265737320676170A7\n"
        + &":00000001FF\n"
        + &":0400000300003800C1\n";

    let data_rec = Record::Data {
        offset: 0x0010,
        value: vec![
            0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x20, 0x67, 0x61, 0x70,
        ],
    };
    let ssa_rec = Record::StartSegmentAddress {
        cs: 0x0000,
        ip: 0x3800,
    };
    let eof_rec = Record::EndOfFile;

    let mut reader = Reader::new_with_options(
        &input,
        ReaderOptions {
            stop_after_first_error: false,
            stop_after_eof: false,
        },
    );
    assert_eq!(reader.next(), Some(Ok(data_rec)));
    assert_eq!(reader.next(), Some(Ok(eof_rec)));
    assert_eq!(reader.next(), Some(Ok(ssa_rec)));
    assert_eq!(reader.next(), None);
}

#[test]
fn test_reader_respects_stop_after_first_eof_true() {
    let input = String::new()
        + &":0B0010006164647265737320676170A7\n"
        + &":00000001FF\n"
        + &":0400000300003800C1\n";

    let data_rec = Record::Data {
        offset: 0x0010,
        value: vec![
            0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x20, 0x67, 0x61, 0x70,
        ],
    };
    let eof_rec = Record::EndOfFile;

    let mut reader = Reader::new_with_options(
        &input,
        ReaderOptions {
            stop_after_first_error: false,
            stop_after_eof: true,
        },
    );
    assert_eq!(reader.next(), Some(Ok(data_rec)));
    assert_eq!(reader.next(), Some(Ok(eof_rec)));
    assert_eq!(reader.next(), None);
}

#[test]
fn test_reader_allow_no_trailing_newlines() {
    let input = String::new()
        + &":0B0010006164647265737320676170A7\n"
        + &":00000001FF\n"
        + &":0400000300003800C1";

    let data_rec = Record::Data {
        offset: 0x0010,
        value: vec![
            0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x20, 0x67, 0x61, 0x70,
        ],
    };
    let eof_rec = Record::EndOfFile;

    let mut reader = Reader::new(&input);
    assert_eq!(reader.next(), Some(Ok(data_rec)));
    assert_eq!(reader.next(), Some(Ok(eof_rec)));
    assert_eq!(reader.next(), None);
}

#[test]
fn test_reader_respects_all_newline_formats() {
    let input = String::new() +
    &":0B0010006164647265737320676170A7\n"   + // Unix LF
    &":0B0010006164647265737320676170A7\r\n" + // Windows CRLF
    &":00000001FF\r"; // MacOS CR

    let data_rec_1 = Record::Data {
        offset: 0x0010,
        value: vec![
            0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x20, 0x67, 0x61, 0x70,
        ],
    };
    let data_rec_2 = Record::Data {
        offset: 0x0010,
        value: vec![
            0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x20, 0x67, 0x61, 0x70,
        ],
    };
    let eof_rec = Record::EndOfFile;

    let mut reader = Reader::new(&input);
    assert_eq!(reader.next(), Some(Ok(data_rec_1)));
    assert_eq!(reader.next(), Some(Ok(data_rec_2)));
    assert_eq!(reader.next(), Some(Ok(eof_rec)));
    assert_eq!(reader.next(), None);
}

#[test]
fn test_reader_allow_no_trailing_newlines_on_one_record() {
    let input = String::from(":0B0010006164647265737320676170A7");

    let data_rec = Record::Data {
        offset: 0x0010,
        value: vec![
            0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x20, 0x67, 0x61, 0x70,
        ],
    };

    let mut reader = Reader::new(&input);
    assert_eq!(reader.next(), Some(Ok(data_rec)));
    assert_eq!(reader.next(), None);
}

#[test]
fn test_reader_respects_ignores_extra_newlines() {
    let input = String::new()
        + &":0B0010006164647265737320676170A7\n"
        + &":00000001FF\n\n\n"
        + &":0400000300003800C1\n\n";

    let data_rec = Record::Data {
        offset: 0x0010,
        value: vec![
            0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x20, 0x67, 0x61, 0x70,
        ],
    };
    let eof_rec = Record::EndOfFile;

    let mut reader = Reader::new_with_options(
        &input,
        ReaderOptions {
            stop_after_first_error: false,
            stop_after_eof: true,
        },
    );
    assert_eq!(reader.next(), Some(Ok(data_rec)));
    assert_eq!(reader.next(), Some(Ok(eof_rec)));
    assert_eq!(reader.next(), None);
}
