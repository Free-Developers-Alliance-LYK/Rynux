//! This module provides functionality for managing CPU.

pub mod cpu_mask;
mod cpu_state;

use cpu_state::CpuStateManager;
use cpu_mask::CpuMask;

pub mod processor;
