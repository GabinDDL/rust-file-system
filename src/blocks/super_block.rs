//! Module defining structures and functions to use super blocks
use super::{block_information, typed_block::TypedBlock};

use bytemuck::{Pod, Zeroable};

pub const NB_ZEROS: usize = 3972; // Size of zeros of super block

/// This structure represents zeros of super block data
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct SpZeros(pub [u8; NB_ZEROS]);

/// We implement this traits to be able to derive them for double indirection structure
unsafe impl Zeroable for SpZeros {}
unsafe impl Pod for SpZeros {}

/// This structure represents super block with its magic value, number of block, number of inodes,
/// number of allocable block, number of free allocable block, number of file, zeros and
/// information
#[repr(C)]
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
pub struct SuperBlock {
    magic: [u8; 8],                             // Magic value
    nb_b: u32,                                  // Number of blocks
    nb_i: u32,                                  // Number of inodes
    nb_a: u32,                                  // Number of allocable block
    nb_fa: u32,                                 // Number of free allocable block
    nb_f: u32,                                  // Number of file
    zeros: SpZeros,                             // Zeros of super block
    infos: block_information::BlockInformation, // Information of the block
}

impl TypedBlock for SuperBlock {
    const KIND: super::block_type::BlockType = super::block_type::BlockType::Super;

    fn get_block_information(&self) -> &block_information::BlockInformation {
        &self.infos
    }
}

#[cfg(test)]
mod tests {
    use crate::blocks::block_information::BLOCK_SIZE;

    use super::*;

    /// This test check if the size of the super block structure is the right one
    #[test]
    fn super_block_size() {
        let size = std::mem::size_of::<SuperBlock>();
        assert_eq!(BLOCK_SIZE, size);
    }
}
