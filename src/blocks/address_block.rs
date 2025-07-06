//! Module defining functions to use address blocks
use bytemuck::{Pod, Zeroable};

pub const ADDRESS_BLOCK_LEN: usize = 1000; // Number of addresses for a block

/// This trait represents an address block
///
/// AddressBlock implementations has to provide utility functions to interact with the
/// addresses
pub trait AddressBlock: Pod + Zeroable + Sized {
    /// Give the address of a block of a precised position
    ///
    /// * `index` - the position of the block in the block
    fn get_address(&self, index: usize) -> i32;
}
