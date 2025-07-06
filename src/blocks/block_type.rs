//! Module defining enum and functions to use block type
use bytemuck::{Pod, Zeroable};

/// This enum represents different types for blocks of the system
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockType {
    Unknown = 0,
    Super = 1,
    Bitmap = 2,
    Inode = 3,
    FreeAllocable = 4,
    Data = 5,
    SimpleIndirection = 6,
    DoubleIndirection = 7,
}

/// We implement this traits to be able to derive them for block type enum
unsafe impl Zeroable for BlockType {}
unsafe impl Pod for BlockType {}

#[cfg(test)]
mod tests {
    use super::*;

    /// This test check if the size of the block type enum is the right one
    #[test]
    fn block_type_size() {
        let expected_size = std::mem::size_of::<u32>();
        let size = std::mem::size_of::<BlockType>();
        assert_eq!(expected_size, size);
    }
}
