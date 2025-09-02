//! A pure-Rust #![no_std] crate for parsing Flattened Devicetrees,
//! with the goal of having a very ergonomic and idiomatic API.

#![no_std]

mod standard_nodes;
mod kernel_nodes;
mod error;
mod parsing;
mod node;
mod header;
mod pretty_print;

pub use kernel_nodes::*;
pub use standard_nodes::*;
pub use error::FdtError;
pub use node::FdtNode;
use parsing::{FdtData, BigEndianU32, CStr};
use header::FdtHeader;

/// A flattened devicetree located somewhere in memory
/// 
#[derive(Clone, Copy)]
pub struct LinuxFdt<'a> {
    data: &'a [u8],
    header: FdtHeader,
}

impl core::fmt::Debug for LinuxFdt<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        pretty_print::print_node(f, self.root().node, 0)?;
        Ok(())
    }
}

impl<'a> LinuxFdt<'a> {
    /// Construct a new `Fdt` from a byte buffer
    ///
    /// Note: this function does ***not*** require that the data be 4-byte
    /// aligned
    pub fn new(data: &'a [u8]) -> Result<Self, FdtError> {
        let mut stream = FdtData::new(data);
        let header = FdtHeader::from_bytes(&mut stream).ok_or(FdtError::BufferTooSmall)?;

        if !header.valid_magic() {
            return Err(FdtError::BadMagic);
        } else if data.len() < header.totalsize.get() as usize {
            return Err(FdtError::BufferTooSmall);
        }

        Ok(Self { data, header })
    }

    /// # Safety
    /// This function performs a read to verify the magic value. If the pointer
    /// is invalid this can result in undefined behavior.
    ///
    /// Note: this function does ***not*** require that the data be 4-byte
    /// aligned
    pub unsafe fn from_ptr(ptr: *const u8) -> Result<Self, FdtError> {
        if ptr.is_null() {
            return Err(FdtError::BadPtr);
        }

        // SAFETY: we assume that the pointer is valid and points to a valid FDT
        let tmp_header = unsafe {
                core::slice::from_raw_parts(ptr, core::mem::size_of::<FdtHeader>())
        };

        let real_size =
            FdtHeader::from_bytes(&mut FdtData::new(tmp_header)).unwrap().totalsize.get() as usize;

        unsafe {
            Self::new(core::slice::from_raw_parts(ptr, real_size))
        }
    }

    /// Total size of the devicetree in bytes
    pub fn total_size(&self) -> usize {
        self.header.totalsize.get() as usize
    }

    /// Return the root (`/`) node, which is always available
    pub fn root(&self) -> Root<'_, 'a> {
        Root { node: self.find_node("/").expect("/ is a required node") }
    }

    /// Returns the machine name
    pub fn machine(&self) -> &'a str {
        core::str::from_utf8(self.root().property("model").expect("expected model property").value).unwrap().trim_end_matches('\0')
    }

    /// Returns the chosen node
    pub fn chosen(&self) -> Chosen<'_, 'a> {
        node::find_node(&mut FdtData::new(self.structs_block()), "/chosen", self, None)
            .map(|node| Chosen { node })
            .expect("/chosen is required")
    }

    /// Returns the avaiable mem regions
    pub fn mem_nodes(&self) -> impl Iterator<Item = Memory<'_, 'a>> + '_ {
        self.all_nodes()
            .filter(|node| {
                node.property("device_type")
                .and_then(|p| core::str::from_utf8(p.value).ok())
                .map(|s| s.trim_end_matches('\0') == "memory")
                .unwrap_or(false)
            })
            .filter(|node| node.is_available())
            .map(|node| Memory { node: node.clone() })
    }

    /// Return the `/aliases` node, if one exists
    pub fn aliases(&self) -> Option<Aliases<'_, 'a>> {
        Some(Aliases {
            node: node::find_node(&mut FdtData::new(self.structs_block()), "/aliases", self, None)?,
            header: self,
        })
    }

    /// Returns the first node that matches the node path, if you want all that
    /// match the path, use `find_all_nodes`. This will automatically attempt to
    /// resolve aliases if `path` is not found.
    ///
    /// Node paths must begin with a leading `/` and are ASCII only. Passing in
    /// an invalid node path or non-ASCII node name in the path will return
    /// `None`, as they will not be found within the devicetree structure.
    ///
    /// Note: if the address of a node name is left out, the search will find
    /// the first node that has a matching name, ignoring the address portion if
    /// it exists.
    pub fn find_node(&self, path: &str) -> Option<node::FdtNode<'_, 'a>> {
        let node = node::find_node(&mut FdtData::new(self.structs_block()), path, self, None);
        node.or_else(|| self.aliases()?.resolve_node(path))
    }

    /// Searches for the given `phandle`
    pub fn find_phandle(&self, phandle: u32) -> Option<node::FdtNode<'_, 'a>> {
        self.all_nodes().find(|n| {
            n.properties()
                .find(|p| p.name == "phandle")
                .and_then(|p| Some(BigEndianU32::from_bytes(p.value)?.get() == phandle))
                .unwrap_or(false)
        })
    }

    /// Returns an iterator over all of the nodes in the devicetree, depth-first
    pub fn all_nodes(&self) -> impl Iterator<Item = node::FdtNode<'_, 'a>> {
        node::all_nodes(self)
    }

    fn cstr_at_offset(&self, offset: usize) -> CStr<'a> {
        CStr::new(&self.strings_block()[offset..]).expect("no null terminating string on C str?")
    }

    fn str_at_offset(&self, offset: usize) -> &'a str {
        self.cstr_at_offset(offset).as_str().expect("not utf-8 cstr")
    }

    fn structs_block(&self) -> &'a [u8] {
        &self.data[self.header.struct_range()]
    }

    fn strings_block(&self) -> &'a [u8] {
        &self.data[self.header.strings_range()]
    }
}
