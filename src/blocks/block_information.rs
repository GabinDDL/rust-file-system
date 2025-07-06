//! Module defining structures and functions to use block informations
use bytemuck::{Pod, Zeroable};

use crate::blocks::block_type::BlockType;
use crate::utils::sha1::Sha1;

pub const EXTENSION_SIZE: usize = 72; // Size of block information extension

pub const EFFECTIVE_BLOCK_SIZE: usize = 4000; // Size of effective datas of a block
pub const BLOCK_INFORMATION_SIZE: usize = 96; // Size of block information of a block
pub const BLOCK_SIZE: usize = EFFECTIVE_BLOCK_SIZE + BLOCK_INFORMATION_SIZE; // Size of an entire
// block

/// This structure represents the extension data of block information part
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Extension(pub [u8; EXTENSION_SIZE]);

/// We implement this traits to be able to derive them for block information structure
unsafe impl Zeroable for Extension {}
unsafe impl Pod for Extension {}

/// This structure represents blocks informations with the SHA-1, the type of the block and its
/// extension data
#[repr(C)]
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
pub struct BlockInformation {
    pub sha1: Sha1,           // The SHA-1
    pub btype: BlockType,     // The block type
    pub extension: Extension, // The extension data
}

#[cfg(test)]
mod tests {
    use super::*;

    /// This test check if the size of the block information structure is the right one
    #[test]
    fn block_information_size() {
        let size = std::mem::size_of::<BlockInformation>();
        assert_eq!(BLOCK_INFORMATION_SIZE, size);
    }
}
