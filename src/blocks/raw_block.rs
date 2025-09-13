//! Module defining structures and functions to use raw blocks
use crate::blocks::block_information;

/// This structure represents a raw block without information except its size
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct RawBlock {
    pub bytes: [u8; block_information::BLOCK_SIZE],
}

impl RawBlock {
    /// Returns bytes array of the block
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    // Returns mutable bytes array of the block
    pub fn as_bytes_mut(&mut self) -> &mut [u8] {
        &mut self.bytes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// This test check if the size of the raw block structure is the right one
    #[test]
    fn raw_block_size() {
        let size = std::mem::size_of::<RawBlock>();
        assert_eq!(block_information::BLOCK_SIZE, size);
    }
}
