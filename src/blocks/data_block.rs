//! Module defining structures and functions to use data blocks
use super::{block_information, typed_block::TypedBlock};

use bytemuck::{Pod, Zeroable};

pub const DATA_BLOCK_LEN: usize = 4000; // Size of data block data

/// This structure represents data of a data block
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DbData(pub [u8; DATA_BLOCK_LEN]);

/// We implement this traits to be able to derive them for data block structure
unsafe impl Zeroable for DbData {}
unsafe impl Pod for DbData {}

/// This structure represents a data block with its data and informations
#[repr(C)]
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
pub struct DataBlock {
    data: DbData,                               // Data
    infos: block_information::BlockInformation, // Information of the block
}

impl TypedBlock for DataBlock {
    const KIND: super::block_type::BlockType = super::block_type::BlockType::Data;

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
    fn data_block_size() {
        let size = std::mem::size_of::<DataBlock>();
        assert_eq!(BLOCK_SIZE, size);
    }
}
