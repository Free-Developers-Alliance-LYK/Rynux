//! Linux kernel reserved-memory nodes
//!
//! Reference: https://www.kernel.org/doc/Documentation/devicetree/bindings/reserved-memory/reserved-memory.yaml

use crate::node::FdtNode;
use crate::standard_nodes::RegIter;

/// Represents the `/reserved-memory/*` node, it status is ok and have `reg` property
#[derive(Debug, Clone, Copy)]
pub struct ValidReservedMemoryNode<'b, 'a> {
    /// node
    pub node: FdtNode<'b, 'a>,
}

impl <'b, 'a: 'b> ValidReservedMemoryNode<'b, 'a> {
    /// Returns an iterator over all of the valid regs
    pub fn regions(&self) -> Option<RegIter<'a>> {
        self.node.reg()
    }

    /// return nomap property
    pub fn nomap(&self) -> bool {
        self.node.property("no-map").is_some()
    }
}

/// Represents the `/reserved-memory/*` node, it status is ok and don't have `reg` property
/// but have `size` property represent need to be dynamic allocated
#[derive(Debug, Clone, Copy)]
pub struct DynamicReservedMemoryNode<'b, 'a> {
    /// node
    pub node: FdtNode<'b, 'a>,
    size: usize,
}

impl <'b, 'a: 'b> DynamicReservedMemoryNode<'b, 'a> {
    /// return size
    #[inline]
    pub fn size(&self) -> usize {
        self.size
    }

    /// return alignment
    pub fn alignment(&self) -> usize {
        // if no alignment, default is 0
        self.node.property("alignment").map(|p| p.as_usize().unwrap()).unwrap_or(0)
    }

    /// return nomap
    pub fn nomap(&self) -> bool {
        self.node.property("no-map").is_some()
    }

    /// alloc-ranges
    /// Address and Length pairs. Specifies regions of memory that are
    /// acceptable to allocate from.
    pub fn alloc_ranges(&self) -> Option<RegIter<'a>> {
        self.node.property("alloc-ranges").map(|p| p.as_reg(self.node.parent_cell_sizes()).unwrap())
    }

    /// reusable property
    pub fn reusable(&self) -> bool {
        self.node.property("reusable").is_some()
    }

    /// shared_dma_pool compatible
    pub fn shared_dma_pool(&self) -> bool {
        self.node.compatible().map(|p| p.first() == "shared-dma-pool").unwrap_or(false)
    }

}

/// Represents the `/reserved-memory` node with specific helper methods
#[derive(Debug, Clone, Copy)]
pub struct ReservedMemory<'b, 'a> {
    pub(crate) node: FdtNode<'b, 'a>,
}

impl<'b, 'a: 'b> ReservedMemory<'b, 'a> {
    /// Contains the bootargs, if they exist
    pub(crate) fn check_root(self) -> Result<(), ()> {
        // reserved mem root should have address-cells and size-cells and 
        // ranges property
        if self.node.property("#address-cells").is_none() {
            return Err(());
        }
        if self.node.property("#size-cells").is_none() {
            return Err(());
        }
        if self.node.property("ranges").is_none() {
            return Err(());
        }

        // check size and address cell size is eque root node cell
        if self.node.cell_sizes() != self.node.parent_cell_sizes()  {
            return Err(());
        }
        Ok(())
    }

    /// Return valid ReservedNode
    pub fn valid_reserved_nodes(self) -> impl Iterator<Item = ValidReservedMemoryNode<'b, 'a>> + 'b {
        self.node.children().filter_map(|node| {
            if node.is_available() {
                if node.reg().is_some() {
                    return Some(ValidReservedMemoryNode { node });
                }
            }
            None
        })
    }

    /// Return dynamic nodes
    pub fn dynamic_nodes(self) -> impl Iterator<Item = DynamicReservedMemoryNode<'b, 'a>> + 'b {
        self.node.children().filter_map(|node| {
            if node.is_available() {
                if let Some(size) = node.property("size") && node.reg().is_none() {
                    return Some(DynamicReservedMemoryNode { node, size: size.as_usize().unwrap() });
                }
            }
            None
        })
    }
}
