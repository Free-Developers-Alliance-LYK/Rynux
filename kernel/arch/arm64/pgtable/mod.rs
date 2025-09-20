//! Arm64 Page table

pub mod pgdir;
pub mod pgprot;
pub mod pte;

pub mod config;
pub mod idmap;
pub mod pmd;
pub mod pud;

pub use config::Arm64PgtableConfig;
pub use pgdir::{PgdirEntry, PgdirTable};
pub use pgprot::PtePgProt;
pub use pmd::{PmdEntry, PmdTable};
pub use pte::{PteEntry, PteTable};
pub use pud::{PudEntry, PudTable};

use crate::mm::PhysAddr;
/// Page table entry
pub trait PgTableEntry {
    /// is none
    #[inline(always)]
    fn is_none(&self) -> bool {
        self.read() == 0
    }
    /// Get the value of the entry
    fn value(&self) -> u64;
    /// Read the value of the entry no cache
    fn read(&self) -> u64;
    /// Write the value of the entry no cache
    fn write(&mut self, val: u64);
    /// Convert to physical address
    fn to_phys(&self) -> PhysAddr;
    /// Convert from physical address
    fn from_phys(pa: PhysAddr) -> Self;
}
