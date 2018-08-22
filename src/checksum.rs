//
// Copyright 2016 The IHEX Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>.
// All files in the project carrying such notice may not be copied, modified, or 
// distributed except according to those terms.
//

/**
 Computes the Intel HEX checksum of `data`. This is done by summing all the bytes `data
 and taking the two's complement of the least significant byte of the sum.
 */
pub fn checksum(data: &[u8]) -> u8 {
    let sum: u8 = data.iter()
        .fold(0, |acc, &value| acc.wrapping_add(value as u8));

    let checksum = (0 as u8).wrapping_sub(sum);
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
