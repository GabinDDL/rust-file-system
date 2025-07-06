//! Module defining structures and functions to use bitmap blocks
use super::{
    block_information::{self, EFFECTIVE_BLOCK_SIZE},
    typed_block::TypedBlock,
};

use bytemuck::{Pod, Zeroable};

/// This structure represents bitmap indexes of blocks
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct BmBlocks(pub [u8; EFFECTIVE_BLOCK_SIZE]);

/// We implement this traits to be able to derive them for bitmap structure
unsafe impl Zeroable for BmBlocks {}
unsafe impl Pod for BmBlocks {}

/// This structure represents a bitmap block, with its indexes and information
#[repr(C)]
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
pub struct BitmapBlock {
    blocks: BmBlocks,                           // Indexes of the bitmap
    infos: block_information::BlockInformation, // Information of the block
}

impl TypedBlock for BitmapBlock {
    const KIND: super::block_type::BlockType = super::block_type::BlockType::Bitmap;

    fn get_block_information(&self) -> &block_information::BlockInformation {
        &self.infos
    }
}

#[cfg(test)]
mod tests {
    use crate::blocks::block_information::BLOCK_SIZE;

    use super::*;

    /// This test check if the size of the bitmap structure is the right one
    #[test]
    fn bitmap_block_size() {
        let size = std::mem::size_of::<BitmapBlock>();
        assert_eq!(BLOCK_SIZE, size);
    }
}
