//! Instruction module for the Motorola 88000 emulator.
//! 
//! This module contains implementations of all CPU instructions organized by category:
//! - Arithmetic instructions
//! - Logical instructions
//! - Control flow instructions
//! - Memory access instructions
//! - Floating point instructions
//! - Vector instructions
//! - System instructions
//! - MMU instructions

pub mod arithmetic;
pub mod logical;
pub mod control;
pub mod memory_access;
pub mod floating_point;
pub mod vector;
pub mod system;
pub mod mmu;

use crate::cpu::CPU;
use crate::memory::Memory;

/// Trait defining the interface for all CPU instructions.
/// 
/// Each instruction must implement this trait to be executable by the CPU.
/// The execute method performs the actual operation, modifying the CPU and/or
/// memory state as needed.
pub trait Instruction {
    /// Executes the instruction.
    /// 
    /// # Arguments
    /// 
    /// * `cpu` - Mutable reference to the CPU state
    /// * `memory` - Mutable reference to the system memory
    fn execute(&self, cpu: &mut CPU, memory: &mut Memory);
}