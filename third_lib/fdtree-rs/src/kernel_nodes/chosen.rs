//! Linux kernel chosen nodes

use crate::node::FdtNode;
use crate::standard_nodes::RegIter;

/// Represents the `/chosen` node with specific helper methods
#[derive(Debug, Clone, Copy)]
pub struct Stdout<'b, 'a> {
    /// stdout node
    pub node: FdtNode<'b, 'a>,
    /// options
    pub options: Option<&'a str>,
}

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
    pub fn stdout(self) -> Option<Stdout<'b, 'a>> {
        let mut stdout_path = self.node
            .properties()
            .find(|n| n.name == "stdout-path");

        if stdout_path.is_none() {
            // try linux,stdout-path
            stdout_path = self.node
                .properties()
                .find(|n| n.name == "linux,stdout-path");
            if stdout_path.is_none() {
                return None;
            }
        }

        let stdout_path = stdout_path.unwrap();
        let stdout_path = core::str::from_utf8(&stdout_path.value[..stdout_path.value.len() - 1]).ok()?;
        let (node_name, options) = stdout_path.split_once(':').unwrap_or((stdout_path, ""));
        let node = self.node.header.find_node(node_name)?;

        if options.is_empty() {
            return Some(Stdout { node, options: None });
        } else {
            return Some(Stdout { node, options: Some(options) });
        }
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
