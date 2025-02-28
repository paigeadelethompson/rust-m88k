//! CPU module for the Motorola 88000 emulator.
//!
//! This module implements the core CPU functionality including register management,
//! control flags, and MMU support.

pub mod instructions;

use instructions::system::PrivilegeLevel;

/// CPU state for the Motorola 88000.
///
/// Maintains the processor state including general purpose registers,
/// program counter, control registers, and MMU state.
#[derive(Debug, Default)]
pub struct CPU {
    /// General purpose registers (r0-r31)
    pub registers: [u32; 32],
    /// Program counter
    pub pc: u32,
    /// Control register 0 (Processor Status Register)
    pub cr0: u32,
    /// Current instruction's destination register
    pub d: usize,
    /// Current instruction's first source register
    pub s1: usize,
    /// Current instruction's second source register
    pub s2: usize,
    /// Current instruction's immediate value
    pub imm: i16,
    /// Current instruction's branch offset
    pub offset: i16,
    /// Shadow execution instruction pointer
    pub sxip: u32,
    /// Shadow next instruction pointer
    pub snip: u32,
    /// Shadow fetch instruction pointer
    pub sfip: u32,
    /// Next instruction pointer
    pub nip: u32,
    /// Fetch instruction pointer
    pub fip: u32,
    /// Current vector number
    pub vector: u8,
    /// Current trap vector number
    pub trap_vector: u8,
    /// Page Table Base Register
    pub ptbr: u32,
    /// MMU Control Register
    pub mmu_control: u32,
    /// Current privilege level
    privilege_level: PrivilegeLevel,
}

impl CPU {
    /// Condition code flag: Equal
    pub const CR0_EQUAL: u32 = 1 << 0;
    /// Condition code flag: Less Than
    pub const CR0_LESS: u32 = 1 << 1;
    /// Condition code flag: Greater Than
    pub const CR0_GREATER: u32 = 1 << 2;
    /// Condition code flag: Unordered Comparison
    pub const CR0_UNORDERED: u32 = 1 << 3;

    /// Floating point flag: Division by Zero
    pub const CR0_FP_DIVZERO: u32 = 1 << 4;
    /// Floating point flag: Inexact Result
    pub const CR0_FP_INEXACT: u32 = 1 << 5;
    /// Floating point flag: Invalid Operation
    pub const CR0_FP_INVALID: u32 = 1 << 6;
    /// Floating point flag: Overflow
    pub const CR0_FP_OVERFLOW: u32 = 1 << 7;
    /// Floating point flag: Underflow
    pub const CR0_FP_UNDERFLOW: u32 = 1 << 8;
    /// Floating point flag: Equal
    pub const CR0_FP_EQUAL: u32 = 1 << 9;
    /// Floating point flag: Less Than
    pub const CR0_FP_LESS: u32 = 1 << 10;
    /// Floating point flag: Greater Than
    pub const CR0_FP_GREATER: u32 = 1 << 11;
    /// Floating point flag: Unordered Comparison
    pub const CR0_FP_UNORDERED: u32 = 1 << 12;
    /// Floating point comparison mask
    pub const CR0_FP_COMPARE_MASK: u32 =
        Self::CR0_FP_EQUAL | Self::CR0_FP_LESS | Self::CR0_FP_GREATER | Self::CR0_FP_UNORDERED;

    /// Exception flag: Bounds Check Violation
    pub const CR0_BOUNDS_CHECK: u32 = 1 << 13;
    /// Exception flag: Trap
    pub const CR0_TRAP: u32 = 1 << 14;
    /// Exception flag: Page Fault
    pub const CR0_PAGE_FAULT: u32 = 1 << 15;
    /// Exception flag: Write Protection Violation
    pub const CR0_WRITE_PROTECT: u32 = 1 << 16;
    /// Exception flag: Privilege Violation
    pub const CR0_PRIVILEGE_VIOLATION: u32 = 1 << 17;

    /// MMU control bit: Enable MMU
    #[allow(dead_code)]
    pub const MMU_ENABLE: u32 = 1 << 0;
    /// MMU control bit: Supervisor Mode
    #[allow(dead_code)]
    pub const MMU_SUPERVISOR: u32 = 1 << 1;
    /// MMU control bit: Write Protection
    #[allow(dead_code)]
    pub const MMU_WRITE_PROTECT: u32 = 1 << 2;

    /// Creates a new CPU instance with default values.
    ///
    /// # Returns
    ///
    /// A new CPU instance with all registers and flags initialized to zero.
    pub fn new() -> Self {
        Self {
            privilege_level: PrivilegeLevel::User,
            ..Default::default()
        }
    }

    /// Sets a floating point flag in CR0.
    ///
    /// # Arguments
    ///
    /// * `flag` - The flag(s) to set
    pub fn set_fp_flag(&mut self, flag: u32) {
        self.cr0 |= flag;
    }

    /// Clears a floating point flag in CR0.
    ///
    /// # Arguments
    ///
    /// * `flag` - The flag(s) to clear
    pub fn clear_fp_flag(&mut self, flag: u32) {
        self.cr0 &= !flag;
    }

    /// Sets the page fault flag in CR0.
    pub fn set_page_fault(&mut self) {
        self.cr0 |= Self::CR0_PAGE_FAULT;
    }

    /// Sets the write protection fault flag in CR0.
    pub fn set_write_protect_fault(&mut self) {
        self.cr0 |= Self::CR0_WRITE_PROTECT;
    }

    /// Sets the privilege violation flag in CR0.
    pub fn set_privilege_violation(&mut self) {
        self.cr0 |= Self::CR0_PRIVILEGE_VIOLATION;
    }

    /// Clears the privilege violation flag in CR0.
    pub fn clear_privilege_violation(&mut self) {
        self.cr0 &= !Self::CR0_PRIVILEGE_VIOLATION;
    }

    /// Checks if a privilege violation has occurred.
    pub fn has_privilege_violation(&self) -> bool {
        (self.cr0 & Self::CR0_PRIVILEGE_VIOLATION) != 0
    }

    /// Gets the current privilege level.
    pub fn get_privilege_level(&self) -> PrivilegeLevel {
        self.privilege_level
    }

    /// Sets the current privilege level.
    pub fn set_privilege_level(&mut self, level: PrivilegeLevel) {
        self.privilege_level = level;
    }

    /// Checks if the MMU is enabled.
    ///
    /// # Returns
    ///
    /// `true` if the MMU is enabled, `false` otherwise.
    pub fn mmu_enabled(&self) -> bool {
        (self.mmu_control & Self::MMU_ENABLE) != 0
    }

    /// Sets the MMU enabled state.
    ///
    /// # Arguments
    ///
    /// * `enabled` - Whether to enable or disable the MMU
    pub fn set_mmu_enabled(&mut self, enabled: bool) {
        if enabled {
            self.mmu_control |= Self::MMU_ENABLE;
        } else {
            self.mmu_control &= !Self::MMU_ENABLE;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cpu_initialization() {
        let cpu = CPU::new();
        assert_eq!(cpu.registers[0], 0);
        assert_eq!(cpu.pc, 0);
        assert_eq!(cpu.cr0, 0);
        assert_eq!(cpu.ptbr, 0);
        assert_eq!(cpu.mmu_control, 0);
        assert_eq!(cpu.get_privilege_level(), PrivilegeLevel::User);
    }

    #[test]
    fn test_set_fp_flag() {
        let mut cpu = CPU::new();

        // Test setting a single flag
        cpu.set_fp_flag(CPU::CR0_FP_OVERFLOW);
        assert_eq!(cpu.cr0 & CPU::CR0_FP_OVERFLOW, CPU::CR0_FP_OVERFLOW);

        // Test setting multiple flags
        cpu.set_fp_flag(CPU::CR0_FP_OVERFLOW | CPU::CR0_FP_INEXACT);
        assert_eq!(cpu.cr0 & CPU::CR0_FP_OVERFLOW, CPU::CR0_FP_OVERFLOW);
        assert_eq!(cpu.cr0 & CPU::CR0_FP_INEXACT, CPU::CR0_FP_INEXACT);

        // Test clearing a flag
        cpu.clear_fp_flag(CPU::CR0_FP_OVERFLOW);
        assert_eq!(cpu.cr0 & CPU::CR0_FP_OVERFLOW, 0);
        assert_eq!(cpu.cr0 & CPU::CR0_FP_INEXACT, CPU::CR0_FP_INEXACT);
    }

    #[test]
    fn test_mmu_control() {
        let mut cpu = CPU::new();

        // Test MMU enable/disable
        assert!(!cpu.mmu_enabled());
        cpu.set_mmu_enabled(true);
        assert!(cpu.mmu_enabled());
        cpu.set_mmu_enabled(false);
        assert!(!cpu.mmu_enabled());

        // Test MMU fault flags
        cpu.set_page_fault();
        assert_eq!(cpu.cr0 & CPU::CR0_PAGE_FAULT, CPU::CR0_PAGE_FAULT);

        cpu.set_write_protect_fault();
        assert_eq!(cpu.cr0 & CPU::CR0_WRITE_PROTECT, CPU::CR0_WRITE_PROTECT);
    }

    #[test]
    fn test_privilege_level() {
        let mut cpu = CPU::new();

        // Test initial privilege level
        assert_eq!(cpu.get_privilege_level(), PrivilegeLevel::User);

        // Test switching to supervisor mode
        cpu.set_privilege_level(PrivilegeLevel::Supervisor);
        assert_eq!(cpu.get_privilege_level(), PrivilegeLevel::Supervisor);

        // Test switching back to user mode
        cpu.set_privilege_level(PrivilegeLevel::User);
        assert_eq!(cpu.get_privilege_level(), PrivilegeLevel::User);
    }

    #[test]
    fn test_privilege_violation() {
        let mut cpu = CPU::new();

        // Test setting privilege violation
        assert!(!cpu.has_privilege_violation());
        cpu.set_privilege_violation();
        assert!(cpu.has_privilege_violation());

        // Test clearing privilege violation
        cpu.clear_privilege_violation();
        assert!(!cpu.has_privilege_violation());
    }

    #[test]
    fn test_error_handling() {
        let mut cpu = CPU::new();

        // Test multiple error conditions
        cpu.set_page_fault();
        cpu.set_write_protect_fault();
        cpu.set_privilege_violation();

        assert_ne!(cpu.cr0 & CPU::CR0_PAGE_FAULT, 0);
        assert_ne!(cpu.cr0 & CPU::CR0_WRITE_PROTECT, 0);
        assert_ne!(cpu.cr0 & CPU::CR0_PRIVILEGE_VIOLATION, 0);

        // Test clearing individual errors
        cpu.clear_privilege_violation();
        assert_eq!(cpu.cr0 & CPU::CR0_PRIVILEGE_VIOLATION, 0);
        assert_ne!(cpu.cr0 & CPU::CR0_PAGE_FAULT, 0);
        assert_ne!(cpu.cr0 & CPU::CR0_WRITE_PROTECT, 0);
    }
}
