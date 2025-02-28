//! Logical instruction implementations for the Motorola 88000.
//! 
//! This module contains implementations of all logical operations including:
//! - Basic bitwise operations (AND, OR, XOR, NOT)
//! - Immediate variants of logical operations
//! - Bit field operations (extract, insert, rotate)
//! - Bit manipulation operations (clear, set, test)

use crate::cpu::CPU;
use crate::memory::Memory;
use crate::cpu::instructions::Instruction;

/// AND instruction: rd = rs1 & rs2
pub struct And;

impl Instruction for And {
    fn execute(&self, cpu: &mut CPU, _memory: &mut Memory) {
        cpu.registers[cpu.d] = cpu.registers[cpu.s1] & cpu.registers[cpu.s2];
    }
}

/// AND immediate instruction: rd = rs1 & immediate
pub struct AndImmediate;

impl Instruction for AndImmediate {
    fn execute(&self, cpu: &mut CPU, _memory: &mut Memory) {
        cpu.registers[cpu.d] = cpu.registers[cpu.s1] & (cpu.imm as u32);
    }
}

/// OR instruction: rd = rs1 | rs2
pub struct Or;

impl Instruction for Or {
    fn execute(&self, cpu: &mut CPU, _memory: &mut Memory) {
        cpu.registers[cpu.d] = cpu.registers[cpu.s1] | cpu.registers[cpu.s2];
    }
}

/// OR immediate instruction: rd = rs1 | immediate
pub struct OrImmediate;

impl Instruction for OrImmediate {
    fn execute(&self, cpu: &mut CPU, _memory: &mut Memory) {
        cpu.registers[cpu.d] = cpu.registers[cpu.s1] | (cpu.imm as u32);
    }
}

/// XOR instruction: rd = rs1 ^ rs2
pub struct Xor;

impl Instruction for Xor {
    fn execute(&self, cpu: &mut CPU, _memory: &mut Memory) {
        cpu.registers[cpu.d] = cpu.registers[cpu.s1] ^ cpu.registers[cpu.s2];
    }
}

/// XOR immediate instruction: rd = rs1 ^ immediate
pub struct XorImmediate;

impl Instruction for XorImmediate {
    fn execute(&self, cpu: &mut CPU, _memory: &mut Memory) {
        cpu.registers[cpu.d] = cpu.registers[cpu.s1] ^ (cpu.imm as u32);
    }
}

/// NOT instruction: rd = ~rs1
pub struct Not;

impl Instruction for Not {
    fn execute(&self, cpu: &mut CPU, _memory: &mut Memory) {
        cpu.registers[cpu.d] = !cpu.registers[cpu.s1];
    }
}

/// Clear bit instruction: rd = rs1 & ~(1 << rs2)
pub struct Clr;

impl Instruction for Clr {
    fn execute(&self, cpu: &mut CPU, _memory: &mut Memory) {
        let bit = cpu.registers[cpu.s2] & 0x1F; // Only use lower 5 bits for bit position
        cpu.registers[cpu.d] = cpu.registers[cpu.s1] & !(1 << bit);
    }
}

/// Set bit instruction: rd = rs1 | (1 << rs2)
pub struct Set;

impl Instruction for Set {
    fn execute(&self, cpu: &mut CPU, _memory: &mut Memory) {
        let bit = cpu.registers[cpu.s2] & 0x1F; // Only use lower 5 bits for bit position
        cpu.registers[cpu.d] = cpu.registers[cpu.s1] | (1 << bit);
    }
}

/// Extract field instruction: extracts a bit field from rs1
pub struct Ext;

impl Instruction for Ext {
    fn execute(&self, cpu: &mut CPU, _memory: &mut Memory) {
        let width = cpu.registers[cpu.s2] & 0x1F; // Only use lower 5 bits for width
        let offset = (cpu.registers[cpu.s2] >> 5) & 0x1F; // Next 5 bits for offset
        let mask = if width == 0 { 0 } else { (1u32 << width) - 1 };
        cpu.registers[cpu.d] = (cpu.registers[cpu.s1] >> offset) & mask;
    }
}

/// Extract unsigned field instruction: extracts an unsigned bit field from rs1
pub struct ExtU;

impl Instruction for ExtU {
    fn execute(&self, cpu: &mut CPU, _memory: &mut Memory) {
        let width = cpu.registers[cpu.s2] & 0x1F; // Only use lower 5 bits for width
        let offset = (cpu.registers[cpu.s2] >> 5) & 0x1F; // Next 5 bits for offset
        let mask = if width == 0 { 0 } else { (1u32 << width) - 1 };
        cpu.registers[cpu.d] = (cpu.registers[cpu.s1] >> offset) & mask;
    }
}

/// Make field instruction: creates a bit field
pub struct Mak;

impl Instruction for Mak {
    fn execute(&self, cpu: &mut CPU, _memory: &mut Memory) {
        let width = cpu.registers[cpu.s2] & 0x1F; // Only use lower 5 bits for width
        let offset = (cpu.registers[cpu.s2] >> 5) & 0x1F; // Next 5 bits for offset
        let mask = if width == 0 { 0 } else { (1u32 << width) - 1 };
        cpu.registers[cpu.d] = (cpu.registers[cpu.s1] & mask) << offset;
    }
}

/// Rotate instruction: rotates rs1 right by rs2 bits
pub struct Rot;

impl Instruction for Rot {
    fn execute(&self, cpu: &mut CPU, _memory: &mut Memory) {
        let shift = cpu.registers[cpu.s2] & 0x1F; // Only use lower 5 bits for rotation
        cpu.registers[cpu.d] = cpu.registers[cpu.s1].rotate_right(shift);
    }
}

/// Extract unsigned halfword instruction: rd = rs1[15:0]
pub struct ExtUHalf;

impl Instruction for ExtUHalf {
    fn execute(&self, cpu: &mut CPU, _memory: &mut Memory) {
        cpu.registers[cpu.d] = (cpu.registers[cpu.s1] & 0xFFFF) as u32;
    }
}

/// Extract unsigned byte instruction: rd = rs1[7:0]
pub struct ExtUByte;

impl Instruction for ExtUByte {
    fn execute(&self, cpu: &mut CPU, _memory: &mut Memory) {
        cpu.registers[cpu.d] = (cpu.registers[cpu.s1] & 0xFF) as u32;
    }
}

/// Extract halfword instruction (sign-extended): rd = SignExtend(rs1[15:0])
pub struct ExtHalf;

impl Instruction for ExtHalf {
    fn execute(&self, cpu: &mut CPU, _memory: &mut Memory) {
        let value = (cpu.registers[cpu.s1] & 0xFFFF) as i16;
        cpu.registers[cpu.d] = value as i32 as u32;
    }
}

/// Extract byte instruction (sign-extended): rd = SignExtend(rs1[7:0])
pub struct ExtByte;

impl Instruction for ExtByte {
    fn execute(&self, cpu: &mut CPU, _memory: &mut Memory) {
        let value = (cpu.registers[cpu.s1] & 0xFF) as i8;
        cpu.registers[cpu.d] = value as i32 as u32;
    }
}

/// Make field N bits instruction: creates an N-bit field
pub struct MakN;

impl Instruction for MakN {
    fn execute(&self, cpu: &mut CPU, _memory: &mut Memory) {
        let n = cpu.registers[cpu.s2] & 0x1F;  // Get width (0-31)
        let offset = (cpu.registers[cpu.s2] >> 5) & 0x1F;  // Get offset (0-31)
        let mask = if n == 0 { 0 } else { (1u32 << n) - 1 };
        let value = cpu.registers[cpu.s1] & mask;
        cpu.registers[cpu.d] = value << offset;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_and() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();

        cpu.registers[1] = 0xFF00;
        cpu.registers[2] = 0x0FF0;
        cpu.d = 3;
        cpu.s1 = 1;
        cpu.s2 = 2;

        And.execute(&mut cpu, &mut memory);

        assert_eq!(cpu.registers[3], 0x0F00);
    }

    #[test]
    fn test_or() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();

        cpu.registers[1] = 0xFF00;
        cpu.registers[2] = 0x0FF0;
        cpu.d = 3;
        cpu.s1 = 1;
        cpu.s2 = 2;

        Or.execute(&mut cpu, &mut memory);

        assert_eq!(cpu.registers[3], 0xFFF0);
    }

    #[test]
    fn test_extu_half() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();

        cpu.registers[1] = 0xFFFF1234;
        cpu.d = 2;
        cpu.s1 = 1;

        ExtUHalf.execute(&mut cpu, &mut memory);
        assert_eq!(cpu.registers[2], 0x1234);
    }

    #[test]
    fn test_extu_byte() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();

        cpu.registers[1] = 0xFFFFFF12;
        cpu.d = 2;
        cpu.s1 = 1;

        ExtUByte.execute(&mut cpu, &mut memory);
        assert_eq!(cpu.registers[2], 0x12);
    }

    #[test]
    fn test_ext_half() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();

        // Test positive number
        cpu.registers[1] = 0x00001234;
        cpu.d = 2;
        cpu.s1 = 1;

        ExtHalf.execute(&mut cpu, &mut memory);
        assert_eq!(cpu.registers[2] as i32, 0x1234);

        // Test negative number
        cpu.registers[1] = 0x0000F234;
        ExtHalf.execute(&mut cpu, &mut memory);
        assert_eq!(cpu.registers[2] as i32, -3532);  // 0xFFFFF234
    }

    #[test]
    fn test_ext_byte() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();

        // Test positive number
        cpu.registers[1] = 0x00000012;
        cpu.d = 2;
        cpu.s1 = 1;

        ExtByte.execute(&mut cpu, &mut memory);
        assert_eq!(cpu.registers[2] as i32, 0x12);

        // Test negative number
        cpu.registers[1] = 0x000000F2;
        ExtByte.execute(&mut cpu, &mut memory);
        assert_eq!(cpu.registers[2] as i32, -14);  // 0xFFFFFFF2
    }

    #[test]
    fn test_makn() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();

        // Test making a 4-bit field at offset 8
        cpu.registers[1] = 0x0000000F;  // Value
        cpu.registers[2] = (8 << 5) | 4;  // offset=8, width=4
        cpu.d = 3;
        cpu.s1 = 1;
        cpu.s2 = 2;

        MakN.execute(&mut cpu, &mut memory);
        assert_eq!(cpu.registers[3], 0x00000F00);

        // Test with zero width
        cpu.registers[2] = 0;  // offset=0, width=0
        MakN.execute(&mut cpu, &mut memory);
        assert_eq!(cpu.registers[3], 0);

        // Test with maximum width
        cpu.registers[1] = 0xFFFFFFFF;
        cpu.registers[2] = 31;  // offset=0, width=31
        MakN.execute(&mut cpu, &mut memory);
        assert_eq!(cpu.registers[3], 0x7FFFFFFF);
    }

    // Add more tests following the same pattern...
} 