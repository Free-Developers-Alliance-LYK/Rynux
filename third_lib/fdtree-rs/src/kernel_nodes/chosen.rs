//! Linux kernel chosen nodes

use crate::node::FdtNode;
use crate::standard_nodes::RegIter;

/// Represents the `/chosen` node with specific helper methods
#[derive(Debug, Clone, Copy)]
pub struct Chosen<'b, 'a> {
    pub(crate) node: FdtNode<'b, 'a>,
}

impl<'b, 'a: 'b> Chosen<'b, 'a> {
    /// Contains the bootargs, if they exist
    pub fn bootargs(self) -> Option<&'a str> {
        self.node
            .properties()
            .find(|n| n.name == "bootargs")
            .and_then(|n| core::str::from_utf8(&n.value[..n.value.len() - 1]).ok())
    }

    /// Searches for the node representing `stdout`, if the property exists,
    /// attempting to resolve aliases if the node name doesn't exist as-is
    pub fn stdout(self) -> Option<FdtNode<'b, 'a>> {
        self.node
            .properties()
            .find(|n| n.name == "stdout-path")
            .and_then(|n| core::str::from_utf8(&n.value[..n.value.len() - 1]).ok())
            .and_then(|name| self.node.header.find_node(name))
    }

    /// Searches for the node representing `stdout`, if the property exists,
    /// attempting to resolve aliases if the node name doesn't exist as-is. If
    /// no `stdin` property exists, but `stdout` is present, it will return the
    /// node specified by the `stdout` property.
    pub fn stdin(self) -> Option<FdtNode<'b, 'a>> {
        self.node
            .properties()
            .find(|n| n.name == "stdin-path")
            .and_then(|n| core::str::from_utf8(&n.value[..n.value.len() - 1]).ok())
            .and_then(|name| self.node.header.find_node(name))
            .or_else(|| self.stdout())
    }

    /// `linux,usable-memory-range` property
    ///
    /// Important: this method assumes that the value(s) inside the `linux,usable-memory-range`
    /// property represent CPU-addressable addresses that are able to fit within
    /// the platform's pointer size (e.g. `#address-cells` and `#size-cells` are
    /// less than or equal to 2 for a 64-bit platform). If this is not the case
    /// or you're unsure of whether this applies to the node 
    pub fn usable_mem_region(self) -> Option<RegIter<'a>> {
        let sizes = self.node.parent_cell_sizes();
        for prop in self.node.properties() {
            if prop.name == "linux,usable-memory-range" {
                return prop.as_reg(sizes)
            }
        }
        None
    }
}
