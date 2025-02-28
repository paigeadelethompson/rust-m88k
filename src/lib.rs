//! Motorola 88000 Emulator
//!
//! This crate provides an emulator for the Motorola 88000 processor architecture.
//! It implements the core CPU functionality, memory management, and instruction set
//! of the M88000 family of processors.
//!
//! # Features
//!
//! - Complete M88000 instruction set implementation
//! - Memory Management Unit (MMU) support
//! - Floating point operations
//! - Privilege levels and system/user mode
//! - Exception handling
//!
//! # Example
//!
//! ```rust,no_run
//! use motorola88k::{CPU, Memory};
//!
//! let mut cpu = CPU::new();
//! let mut memory = Memory::new();
//!
//! // Initialize CPU state
//! cpu.registers[1] = 42;
//!
//! // Memory operations can be performed using the Memory interface
//! memory.write_word(0x1000, 0x12345678).unwrap();
//! let value = memory.read_word(0x1000).unwrap();
//! ```
//!
//! # Architecture
//!
//! The emulator is organized into several key modules:
//!
//! - [`cpu`]: Core CPU implementation including registers and control flags
//! - [`memory`]: Memory management and MMU implementation
//!
//! The CPU module contains submodules for different instruction categories:
//!
//! - Arithmetic operations
//! - Floating point operations
//! - System instructions
//! - Memory access instructions
//!
//! # Implementation Details
//!
//! The emulator accurately models the M88000's:
//!
//! - 32 general-purpose registers
//! - Program counter and status registers
//! - Memory management with paging support
//! - Privilege levels and protection mechanisms
//! - IEEE 754 floating-point operations
//!
//! For more information about specific components, see the module-level
//! documentation.

pub mod cpu;
pub mod memory;

// Re-export main types for convenience
pub use cpu::CPU;
pub use memory::Memory;