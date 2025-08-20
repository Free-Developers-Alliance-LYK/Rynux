//! Linux kernel memory nodes


use crate::{
    node::FdtNode,
    parsing::FdtData,
    standard_nodes::RegIter,
};

/// Represents the `device_type="memory"` node with specific helper methods
#[derive(Debug, Clone, Copy)]
pub struct Memory<'b, 'a> {
    pub(crate) node: FdtNode<'b, 'a>,
}

impl<'b, 'a: 'b> Memory<'b, 'a> {
    /// Returns an iterator over all of the available memory regions
    pub fn regions(&self) -> Option<RegIter<'a>> {
        if let Some(usable_mem) = self.node.property("linux,usable-memory") {
            let sizes = self.node.parent_cell_sizes();
            return usable_mem.as_reg(sizes);
        } else {
            return self.node.reg();
        }
    }

    /// Returns the initial mapped area, if it exists
    pub fn initial_mapped_area(&self) -> Option<MappedArea> {
        let mut mapped_area = None;

        if let Some(init_mapped_area) = self.node.property("initial_mapped_area") {
            let mut stream = FdtData::new(init_mapped_area.value);
            let effective_address = stream.u64().expect("effective address");
            let physical_address = stream.u64().expect("physical address");
            let size = stream.u32().expect("size");

            mapped_area = Some(MappedArea {
                effective_address: effective_address.get() as usize,
                physical_address: physical_address.get() as usize,
                size: size.get() as usize,
            });
        }
        mapped_area
    }
}

/// An area described by the `initial-mapped-area` property of the `/memory`
/// node
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct MappedArea {
    /// Effective address of the mapped area
    pub effective_address: usize,
    /// Physical address of the mapped area
    pub physical_address: usize,
    /// Size of the mapped area
    pub size: usize,
}

