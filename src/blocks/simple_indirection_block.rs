//! Module defining structures and functions to use simple indirection blocks
use crate::blocks::address_block::AddressBlock;

use super::{address_block::ADDRESS_BLOCK_LEN, block_information, typed_block::TypedBlock};

use bytemuck::{Pod, Zeroable};

/// This structure represents addresses of a simple indirection block
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct SibAddresses(pub [i32; ADDRESS_BLOCK_LEN]);

/// We implement this traits to be able to derive them for simple indirection structure
unsafe impl Zeroable for SibAddresses {}
unsafe impl Pod for SibAddresses {}

/// This structure represents simple indirection block, with its addresses and information
#[repr(C)]
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
pub struct SimpleIndirectionBlock {
    addresses: SibAddresses,                    // Addresses
    infos: block_information::BlockInformation, // Information of the block
}

impl TypedBlock for SimpleIndirectionBlock {
    const KIND: super::block_type::BlockType = super::block_type::BlockType::SimpleIndirection;

    fn get_block_information(&self) -> &block_information::BlockInformation {
        &self.infos
    }
}

impl AddressBlock for SimpleIndirectionBlock {
    fn get_address(&self, index: usize) -> i32 {
        self.addresses.0[index]
    }
}

#[cfg(test)]
mod tests {
    use crate::blocks::block_information::BLOCK_SIZE;

    use super::*;

    /// This test check if the size of the simple indirection structure is the right one
    #[test]
    fn simple_indirection_block_size() {
        let size = std::mem::size_of::<SimpleIndirectionBlock>();
        assert_eq!(BLOCK_SIZE, size);
    }
}
