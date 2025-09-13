//! Module defining structures and functions to use blocks
use crate::blocks::block_information::EFFECTIVE_BLOCK_SIZE;

use super::{block_information, typed_block::TypedBlock};

use bytemuck::{Pod, Zeroable};

/// This structure represents effective data of a block
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct BlockData(pub [u8; EFFECTIVE_BLOCK_SIZE]);

/// We implement this traits to be able to derive them for block structure
unsafe impl Zeroable for BlockData {}
unsafe impl Pod for BlockData {}

/// This structure represents a block without information for its type and data, with data and information
#[repr(C)]
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
pub struct Block {
    data: BlockData,                            // Block data
    infos: block_information::BlockInformation, // Information of the block
}

impl TypedBlock for Block {
    const KIND: super::block_type::BlockType = super::block_type::BlockType::Unknown;

    fn get_block_information(&self) -> &block_information::BlockInformation {
        &self.infos
    }
}

#[cfg(test)]
mod tests {
    use crate::blocks::block_information::BLOCK_SIZE;

    use super::*;

    /// This test check if the size of the block structure is the right one
    #[test]
    fn block_size() {
        let size = std::mem::size_of::<Block>();
        assert_eq!(BLOCK_SIZE, size);
    }
}
