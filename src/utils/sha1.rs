//! Module defining functions to use SHA-1
use bytemuck::{Pod, Zeroable};
use sha1::{Digest, Sha1 as Sha1Hasher};
use std::fmt;

/// Structure of a SHA-1 represented with an array of bytes with size 20
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Zeroable, Pod)]
pub struct Sha1 {
    bytes: [u8; 20],
}

/// Implements useful functions for SHA-1
impl Sha1 {
    /// Compute the SHA-1 of an array of data
    ///
    /// * `data` the array to compute the SHA-1
    pub fn compute(data: &[u8]) -> Self {
        let mut hasher = Sha1Hasher::new(); // Hasher to compute the SHA-1
        hasher.update(data); // We update the haser with datas we want to compute
        let res = hasher.finalize(); // We compute the SHA-1 and get the result
        let bytes = res.into(); // We get the result in bytes
        Sha1 { bytes }
    }
}

impl fmt::Display for Sha1 {
    /// Print the SHA-1 with bytes in sequence
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for byte in &self.bytes {
            write!(f, "{byte:02x}")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test which compute SHA-1 of null data
    #[test]
    fn compute_null_sha1() {
        let data: [u8; 4000] = [0x00; 4000];

        let sha1_hash: Sha1 = Sha1::compute(&data);

        let expected_bytes: [u8; 20] = [
            0x8d, 0x82, 0x69, 0x3c, 0x89, 0x7d, 0x1b, 0x9d, 0xd3, 0x8f, 0xfe, 0xbe, 0xd9, 0xa9,
            0xd0, 0x40, 0x79, 0x9d, 0x67, 0x34,
        ];

        let expected_sha1 = Sha1 {
            bytes: expected_bytes,
        };

        assert_eq!(expected_sha1, sha1_hash)
    }

    /// Test which compute SHA-1 of an example of a SHA-1, which is here a sequence of 0xab
    #[test]
    fn compute_example_sha1() {
        let data: [u8; 4000] = [0xab; 4000];

        let sha1_hash: Sha1 = Sha1::compute(&data);

        let expected_bytes: [u8; 20] = [
            0xfd, 0xf2, 0x81, 0xe7, 0x24, 0x03, 0xb1, 0xaa, 0xae, 0x3e, 0xf7, 0xc2, 0xb8, 0xe7,
            0xb3, 0x5f, 0xe2, 0x86, 0xd2, 0xe2,
        ];

        let expected_sha1 = Sha1 {
            bytes: expected_bytes,
        };

        assert_eq!(expected_sha1, sha1_hash)
    }
}
