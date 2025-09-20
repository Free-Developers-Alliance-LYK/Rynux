//! This module provides functionality for managing CPU.

pub mod cpu_mask;
mod cpu_state;

use cpu_mask::CpuMask;
use cpu_state::CpuStateManager;

pub mod processor;
