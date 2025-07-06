//! Module defining functions to use typed blocks
use crate::blocks::raw_block::RawBlock;

use bytemuck::{Pod, Zeroable};

use super::{block_information::BlockInformation, block_type::BlockType};

/// This trait representes blocks with a type
///
/// TypedBlock implementations has to provide utility functions to interact with block
/// information
pub trait TypedBlock: Pod + Zeroable + Sized {
    const KIND: BlockType;

    fn get_block_information(&self) -> &BlockInformation;
}

/// Returns a TypedBlock block from a RawBlock
pub fn view_as<'a, T: TypedBlock>(raw: &'a RawBlock) -> &'a T {
    bytemuck::from_bytes::<T>(raw.as_bytes())
}

/// Returns a mutable TypedBlock from a RawBlock
pub fn view_as_mut<'a, T: TypedBlock>(raw: &'a mut RawBlock) -> &'a mut T {
    bytemuck::from_bytes_mut::<T>(raw.as_bytes_mut())
}
