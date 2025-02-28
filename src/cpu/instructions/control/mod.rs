//! Control flow instruction implementations for the Motorola 88000.
//! 
//! This module contains implementations of all control flow operations including:
//! - Conditional branches
//! - Jump instructions
//! - Control register operations
//! - Exception handling
//! - Trap instructions

use crate::cpu::CPU;
use crate::memory::Memory;
use crate::cpu::instructions::Instruction;

/// Branch if equal instruction: if rs1 == rs2 then PC += offset
pub struct Beq;

impl Instruction for Beq {
    fn execute(&self, cpu: &mut CPU, _memory: &mut Memory) {
        if cpu.registers[cpu.s1] == cpu.registers[cpu.s2] {
            cpu.pc = cpu.pc.wrapping_add(cpu.offset as u32);
        }
    }
}

/// Branch if not equal instruction: if rs1 != rs2 then PC += offset
pub struct Bne;

impl Instruction for Bne {
    fn execute(&self, cpu: &mut CPU, _memory: &mut Memory) {
        if cpu.registers[cpu.s1] != cpu.registers[cpu.s2] {
            cpu.pc = cpu.pc.wrapping_add(cpu.offset as u32);
        }
    }
}

/// Jump register instruction: PC = rs1
pub struct Jr;

impl Instruction for Jr {
    fn execute(&self, cpu: &mut CPU, _memory: &mut Memory) {
        cpu.pc = cpu.registers[cpu.s1];
    }
}

/// Jump and link instruction: rd = PC + 4; PC = rs1
pub struct Jal;

impl Instruction for Jal {
    fn execute(&self, cpu: &mut CPU, _memory: &mut Memory) {
        let return_addr = cpu.pc.wrapping_add(4);
        cpu.pc = cpu.registers[cpu.s1];
        cpu.registers[cpu.s1] = return_addr;
    }
}

/// Load control register instruction: rd = cr0
pub struct Ldcr;

impl Instruction for Ldcr {
    fn execute(&self, cpu: &mut CPU, _memory: &mut Memory) {
        cpu.registers[cpu.d] = cpu.cr0;
    }
}

/// Store control register instruction: cr0 = rs1
pub struct Stcr;

impl Instruction for Stcr {
    fn execute(&self, cpu: &mut CPU, _memory: &mut Memory) {
        cpu.cr0 = cpu.registers[cpu.s1];
    }
}

/// Return from exception instruction: restores execution state
pub struct Rte;

impl Instruction for Rte {
    fn execute(&self, cpu: &mut CPU, _memory: &mut Memory) {
        cpu.pc = cpu.sxip;
        cpu.nip = cpu.snip;
        cpu.fip = cpu.sfip;
    }
}

/// Trap instruction: generates a software trap
pub struct Trap;

impl Instruction for Trap {
    fn execute(&self, cpu: &mut CPU, _memory: &mut Memory) {
        cpu.cr0 |= CPU::CR0_TRAP;
        cpu.trap_vector = cpu.vector;
    }
}

/// Trap bound instruction: checks if rs1 is within bounds
pub struct Tbnd;

impl Instruction for Tbnd {
    fn execute(&self, cpu: &mut CPU, _memory: &mut Memory) {
        if cpu.registers[cpu.s1] > cpu.registers[cpu.s2] {
            cpu.cr0 |= CPU::CR0_BOUNDS_CHECK;
        }
    }
}

/// Branch if greater than instruction: if rs1 > rs2 then PC += offset
pub struct Bgt;

impl Instruction for Bgt {
    fn execute(&self, cpu: &mut CPU, _memory: &mut Memory) {
        if (cpu.registers[cpu.s1] as i32) > (cpu.registers[cpu.s2] as i32) {
            cpu.pc = cpu.pc.wrapping_add(cpu.offset as u32);
        }
    }
}

/// Branch if less than instruction: if rs1 < rs2 then PC += offset
pub struct Blt;

impl Instruction for Blt {
    fn execute(&self, cpu: &mut CPU, _memory: &mut Memory) {
        if (cpu.registers[cpu.s1] as i32) < (cpu.registers[cpu.s2] as i32) {
            cpu.pc = cpu.pc.wrapping_add(cpu.offset as u32);
        }
    }
}

/// Branch if greater than or equal instruction: if rs1 >= rs2 then PC += offset
pub struct Bge;

impl Instruction for Bge {
    fn execute(&self, cpu: &mut CPU, _memory: &mut Memory) {
        if (cpu.registers[cpu.s1] as i32) >= (cpu.registers[cpu.s2] as i32) {
            cpu.pc = cpu.pc.wrapping_add(cpu.offset as u32);
        }
    }
}

/// Branch if less than or equal instruction: if rs1 <= rs2 then PC += offset
pub struct Ble;

impl Instruction for Ble {
    fn execute(&self, cpu: &mut CPU, _memory: &mut Memory) {
        if (cpu.registers[cpu.s1] as i32) <= (cpu.registers[cpu.s2] as i32) {
            cpu.pc = cpu.pc.wrapping_add(cpu.offset as u32);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_beq() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();

        // Test branch taken
        cpu.registers[1] = 10;
        cpu.registers[2] = 10;
        cpu.s1 = 1;
        cpu.s2 = 2;
        cpu.offset = 100;
        cpu.pc = 1000;

        Beq.execute(&mut cpu, &mut memory);
        assert_eq!(cpu.pc, 1100);  // PC + offset

        // Test branch not taken
        cpu.registers[1] = 10;
        cpu.registers[2] = 20;
        cpu.pc = 1000;

        Beq.execute(&mut cpu, &mut memory);
        assert_eq!(cpu.pc, 1000);  // PC unchanged
    }

    #[test]
    fn test_bne() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();

        // Test branch taken
        cpu.registers[1] = 10;
        cpu.registers[2] = 20;
        cpu.s1 = 1;
        cpu.s2 = 2;
        cpu.offset = 100;
        cpu.pc = 1000;

        Bne.execute(&mut cpu, &mut memory);
        assert_eq!(cpu.pc, 1100);  // PC + offset

        // Test branch not taken
        cpu.registers[1] = 10;
        cpu.registers[2] = 10;
        cpu.pc = 1000;

        Bne.execute(&mut cpu, &mut memory);
        assert_eq!(cpu.pc, 1000);  // PC unchanged
    }

    #[test]
    fn test_bgt() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();

        // Test branch taken
        cpu.registers[1] = 20;
        cpu.registers[2] = 10;
        cpu.s1 = 1;
        cpu.s2 = 2;
        cpu.offset = 100;
        cpu.pc = 1000;

        Bgt.execute(&mut cpu, &mut memory);
        assert_eq!(cpu.pc, 1100);  // PC + offset

        // Test branch not taken
        cpu.registers[1] = 10;
        cpu.registers[2] = 20;
        cpu.pc = 1000;

        Bgt.execute(&mut cpu, &mut memory);
        assert_eq!(cpu.pc, 1000);  // PC unchanged
    }

    #[test]
    fn test_blt() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();

        // Test branch taken
        cpu.registers[1] = 10;
        cpu.registers[2] = 20;
        cpu.s1 = 1;
        cpu.s2 = 2;
        cpu.offset = 100;
        cpu.pc = 1000;

        Blt.execute(&mut cpu, &mut memory);
        assert_eq!(cpu.pc, 1100);  // PC + offset

        // Test branch not taken
        cpu.registers[1] = 20;
        cpu.registers[2] = 10;
        cpu.pc = 1000;

        Blt.execute(&mut cpu, &mut memory);
        assert_eq!(cpu.pc, 1000);  // PC unchanged
    }

    #[test]
    fn test_bge() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();

        // Test branch taken (greater)
        cpu.registers[1] = 20;
        cpu.registers[2] = 10;
        cpu.s1 = 1;
        cpu.s2 = 2;
        cpu.offset = 100;
        cpu.pc = 1000;

        Bge.execute(&mut cpu, &mut memory);
        assert_eq!(cpu.pc, 1100);  // PC + offset

        // Test branch taken (equal)
        cpu.registers[1] = 10;
        cpu.registers[2] = 10;
        cpu.pc = 1000;

        Bge.execute(&mut cpu, &mut memory);
        assert_eq!(cpu.pc, 1100);  // PC + offset

        // Test branch not taken
        cpu.registers[1] = 10;
        cpu.registers[2] = 20;
        cpu.pc = 1000;

        Bge.execute(&mut cpu, &mut memory);
        assert_eq!(cpu.pc, 1000);  // PC unchanged
    }

    #[test]
    fn test_ble() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();

        // Test branch taken (less)
        cpu.registers[1] = 10;
        cpu.registers[2] = 20;
        cpu.s1 = 1;
        cpu.s2 = 2;
        cpu.offset = 100;
        cpu.pc = 1000;

        Ble.execute(&mut cpu, &mut memory);
        assert_eq!(cpu.pc, 1100);  // PC + offset

        // Test branch taken (equal)
        cpu.registers[1] = 10;
        cpu.registers[2] = 10;
        cpu.pc = 1000;

        Ble.execute(&mut cpu, &mut memory);
        assert_eq!(cpu.pc, 1100);  // PC + offset

        // Test branch not taken
        cpu.registers[1] = 20;
        cpu.registers[2] = 10;
        cpu.pc = 1000;

        Ble.execute(&mut cpu, &mut memory);
        assert_eq!(cpu.pc, 1000);  // PC unchanged
    }

    #[test]
    fn test_jr() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();

        cpu.registers[1] = 0x1000;
        cpu.s1 = 1;

        Jr.execute(&mut cpu, &mut memory);
        assert_eq!(cpu.pc, 0x1000);
    }

    #[test]
    fn test_jal() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();

        cpu.registers[1] = 0x1000;
        cpu.s1 = 1;
        cpu.pc = 0x500;

        Jal.execute(&mut cpu, &mut memory);
        assert_eq!(cpu.pc, 0x1000);
        assert_eq!(cpu.registers[1], 0x504);  // PC + 4
    }

    #[test]
    fn test_ldcr() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();

        cpu.cr0 = 0xFFFFFFFF;
        cpu.d = 1;

        Ldcr.execute(&mut cpu, &mut memory);
        assert_eq!(cpu.registers[1], 0xFFFFFFFF);
    }

    #[test]
    fn test_stcr() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();

        cpu.registers[1] = 0xFFFFFFFF;
        cpu.s1 = 1;

        Stcr.execute(&mut cpu, &mut memory);
        assert_eq!(cpu.cr0, 0xFFFFFFFF);
    }

    #[test]
    fn test_rte() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();

        cpu.sxip = 0x1000;
        cpu.snip = 0x1004;
        cpu.sfip = 0x1008;
        cpu.pc = 0x500;

        Rte.execute(&mut cpu, &mut memory);
        assert_eq!(cpu.pc, 0x1000);
        assert_eq!(cpu.nip, 0x1004);
        assert_eq!(cpu.fip, 0x1008);
    }

    #[test]
    fn test_tbnd() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();

        // Test within bounds
        cpu.registers[1] = 100;
        cpu.registers[2] = 200;
        cpu.s1 = 1;
        cpu.s2 = 2;

        Tbnd.execute(&mut cpu, &mut memory);
        assert_eq!(cpu.cr0 & CPU::CR0_BOUNDS_CHECK, 0);

        // Test out of bounds
        cpu.registers[1] = 300;
        cpu.registers[2] = 200;

        Tbnd.execute(&mut cpu, &mut memory);
        assert_ne!(cpu.cr0 & CPU::CR0_BOUNDS_CHECK, 0);
    }

    #[test]
    fn test_trap() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();

        // Test trap vector 5
        cpu.vector = 5;

        Trap.execute(&mut cpu, &mut memory);
        assert_ne!(cpu.cr0 & CPU::CR0_TRAP, 0);
        assert_eq!(cpu.trap_vector, 5);
    }
} 