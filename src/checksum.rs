//
// Copyright 2016 The IHEX Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>.
// All files in the project carrying such notice may not be copied, modified, or 
// distributed except according to those terms.
//

/**
 The Intel HEX checksum is computed by summing all relevant bytes in a
 record and taking the two's complement of the least significant byte of the sum.
 @param data Data to checksum.
 @result The correct checksum byte for the record.
 */
pub fn checksum(data: &[u8]) -> u8 {
  let sum: usize =
    data
      .iter()
      .fold(0, |acc, &value| acc.wrapping_add(value as usize));

  let lsb = (sum & 0xFF) as u8;
  let checksum = (0 as u8).wrapping_sub(lsb);
  checksum
}

#[cfg(test)]
mod tests {

  use super::*;

  #[test]
  fn test_checksum_empty() {
    assert_eq!(checksum(&[]), 0x00);
  }

  #[test]
  fn test_checksum_eof_record() {
    assert_eq!(checksum(&[0x00,0x00,0x00,0x01]), 0xFF);
  }

  #[test]
  fn test_checksum_ela_record() {
    assert_eq!(checksum(&[0x02,0x00,0x00,0x04,0xFF,0xFF]), 0xFC);
  }

  #[test]
  fn test_checksum_sla_record() {
    assert_eq!(checksum(&[0x04,0x00,0x00,0x05,0x00,0x00,0x00,0xCD]), 0x2A);
  }

}
