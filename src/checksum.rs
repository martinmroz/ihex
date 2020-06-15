//
// Copyright 2016 ihex Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

///
/// Computes the Intel HEX checksum of `data`. This is done by summing all the bytes of `data`
/// and taking the two's complement of the least significant byte of the sum.
///
pub fn checksum<T>(data: T) -> u8
where
    T: AsRef<[u8]>,
{
    (0 as u8).wrapping_sub(
        data.as_ref()
            .iter()
            .fold(0, |acc, &value| acc.wrapping_add(value as u8)),
    )
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
        assert_eq!(checksum(&[0x00, 0x00, 0x00, 0x01]), 0xFF);
    }

    #[test]
    fn test_checksum_ela_record() {
        assert_eq!(checksum(&[0x02, 0x00, 0x00, 0x04, 0xFF, 0xFF]), 0xFC);
    }

    #[test]
    fn test_checksum_sla_record() {
        assert_eq!(
            checksum(&[0x04, 0x00, 0x00, 0x05, 0x00, 0x00, 0x00, 0xCD]),
            0x2A
        );
    }
}
