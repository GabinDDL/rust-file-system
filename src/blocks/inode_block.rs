//! Module defining structures and functions to use inode blocks
use super::{block_information, typed_block::TypedBlock};

use bitflags::bitflags;
use bytemuck::{Pod, Zeroable};

pub const INODE_NAME_LEN: usize = 256; // Length of an inode name
pub const INODE_ADDRESSES_SIZE: usize = 900; // Size of inode addresses part
pub const INODE_EXTENSION_SIZE: usize = 120; // Size of inode extension part

bitflags! {
    /// This structure represents flags of inode
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct InodeFlags : u32 {
        const IS_EXISTING_FILE = 1 << 31; // 1 if it exists
        const READ = 1 << 30; // 1 if there is right to read the file
        const WRITE = 1 << 29; // 1 if there is right to write the file
        const READ_LOCKED = 1 << 28; // 1 if the file is locked
        const WRITE_LOCKED = 1 << 27; // 1 if the file is locked
        const IS_REPERTORY = 1 << 26; // 1 if it is a reperetory
    }
}

/// This structure represents an inode name
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct InodeName(pub [u8; INODE_NAME_LEN]);

// This structure represents an inode addresses part
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct InodeAddresses(pub [i32; INODE_ADDRESSES_SIZE]);

/// This structure represents an inode extension part
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct InodeExtension(pub [u8; INODE_EXTENSION_SIZE]);

/// We implement this traits to be able to derive them for inode structure
unsafe impl Zeroable for InodeFlags {}
unsafe impl Pod for InodeFlags {}

unsafe impl Zeroable for InodeName {}
unsafe impl Pod for InodeName {}

unsafe impl Zeroable for InodeAddresses {}
unsafe impl Pod for InodeAddresses {}

unsafe impl Zeroable for InodeExtension {}
unsafe impl Pod for InodeExtension {}

// This structure represents an inode, with its flags, size, creation date, access date,
// modification date, name, addresses, extension data and information
#[repr(C)]
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
pub struct InodeBlock {
    flags: InodeFlags,
    file_size: u32,
    creation_date: u32,
    access_date: u32,
    modification_date: u32,
    name: InodeName,
    adresses: InodeAddresses,
    double_indirection_address: i32,
    extension_data: InodeExtension,
    infos: block_information::BlockInformation, // Information of the block
}

impl TypedBlock for InodeBlock {
    const KIND: super::block_type::BlockType = super::block_type::BlockType::Inode;

    fn get_block_information(&self) -> &block_information::BlockInformation {
        &self.infos
    }
}

#[cfg(test)]
mod tests {
    use crate::blocks::block_information::BLOCK_SIZE;

    use super::*;

    /// This test check if the size of the inode structure is the right one
    #[test]
    fn inode_block_size() {
        let size = std::mem::size_of::<InodeBlock>();
        assert_eq!(BLOCK_SIZE, size);
    }
}
