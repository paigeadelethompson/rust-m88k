//! MMU instruction implementations for the Motorola 88000.
//! 
//! This module contains implementations of Memory Management Unit (MMU) instructions,
//! including page table management and address translation operations.

use crate::cpu::CPU;
use crate::memory::Memory;
use crate::cpu::instructions::Instruction;
use crate::cpu::instructions::system::PrivilegeLevel;

/// Load Page Table Base Register instruction
#[derive(Debug)]
pub struct PTBR {
    pub rd: usize,
}

/// TLB Invalidate instruction
#[derive(Debug)]
pub struct TLBInvalidate;

/// Translate Virtual Address instruction
#[derive(Debug)]
pub struct Translate {
    pub rd: usize,
    pub rs1: usize,
}

impl Instruction for PTBR {
    fn execute(&self, cpu: &mut CPU, memory: &mut Memory) {
        if cpu.get_privilege_level() != PrivilegeLevel::Supervisor {
            cpu.set_privilege_violation();
            return;
        }
        memory.set_mmu_enabled(true);
        memory.set_page_table_base(cpu.registers[self.rd]);
    }
}

impl Instruction for TLBInvalidate {
    fn execute(&self, cpu: &mut CPU, memory: &mut Memory) {
        if cpu.get_privilege_level() != PrivilegeLevel::Supervisor {
            cpu.set_privilege_violation();
            return;
        }
        memory.set_mmu_enabled(false);
    }
}

/// TLB Load instruction: loads a TLB entry
pub struct TLBLoad;

impl Instruction for TLBLoad {
    fn execute(&self, cpu: &mut CPU, memory: &mut Memory) {
        // Load a TLB entry - in our implementation this is handled automatically
        // by the memory subsystem during address translation
        memory.mmu_enabled = cpu.mmu_enabled();
    }
}

impl Instruction for Translate {
    fn execute(&self, cpu: &mut CPU, memory: &mut Memory) {
        match memory.translate_address(cpu.registers[self.rs1]) {
            Ok(physical_addr) => cpu.registers[self.rd] = physical_addr as u32,
            Err(_) => cpu.set_page_fault(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ptbr_privilege() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();

        // Test in user mode (should fail)
        cpu.set_privilege_level(PrivilegeLevel::User);
        let ptbr = PTBR { rd: 1 };
        ptbr.execute(&mut cpu, &mut memory);
        assert!(cpu.has_privilege_violation());
        assert!(!memory.is_mmu_enabled());

        // Test in supervisor mode (should succeed)
        cpu.set_privilege_level(PrivilegeLevel::Supervisor);
        cpu.clear_privilege_violation();
        cpu.registers[1] = 0x1000;
        ptbr.execute(&mut cpu, &mut memory);
        assert!(!cpu.has_privilege_violation());
        assert!(memory.is_mmu_enabled());
    }

    #[test]
    fn test_tlb_invalidate() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();
        cpu.set_privilege_level(PrivilegeLevel::Supervisor);
        memory.set_mmu_enabled(true);

        // Test TLB invalidate
        TLBInvalidate.execute(&mut cpu, &mut memory);
        assert!(!memory.is_mmu_enabled());
    }

    #[test]
    fn test_translate() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();

        // Set up virtual address in rs1
        cpu.registers[1] = 0x1000;

        // Test translation with MMU disabled
        let translate = Translate { rd: 2, rs1: 1 };
        translate.execute(&mut cpu, &mut memory);
        assert_eq!(cpu.registers[2], 0x1000);
        assert!(!cpu.cr0 & CPU::CR0_PAGE_FAULT != 0);

        // Test translation with MMU enabled but no page table
        memory.set_mmu_enabled(true);
        translate.execute(&mut cpu, &mut memory);
        assert!(cpu.cr0 & CPU::CR0_PAGE_FAULT != 0);
    }
} 