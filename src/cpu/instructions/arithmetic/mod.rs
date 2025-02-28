//! Arithmetic instruction implementations for the Motorola 88000.
//!
//! This module contains implementations of all arithmetic operations including:
//! - Basic integer arithmetic (add, subtract, multiply, divide)
//! - Immediate variants of arithmetic operations
//! - Unsigned arithmetic operations
//! - Special arithmetic operations (mask, find first one/zero)

use crate::cpu::instructions::Instruction;
use crate::cpu::CPU;
use crate::memory::Memory;

/// Add instruction: rd = rs1 + rs2
pub struct Add;

impl Instruction for Add {
    fn execute(&self, cpu: &mut CPU, _memory: &mut Memory) {
        cpu.registers[cpu.d] = cpu.registers[cpu.s1].wrapping_add(cpu.registers[cpu.s2]);
    }
}

/// Add immediate instruction: rd = rs1 + immediate
pub struct AddImmediate;

impl Instruction for AddImmediate {
    fn execute(&self, cpu: &mut CPU, _memory: &mut Memory) {
        cpu.registers[cpu.d] = cpu.registers[cpu.s1].wrapping_add(cpu.imm as u32);
    }
}

/// Subtract instruction: rd = rs1 - rs2
pub struct Sub;

impl Instruction for Sub {
    fn execute(&self, cpu: &mut CPU, _memory: &mut Memory) {
        cpu.registers[cpu.d] = cpu.registers[cpu.s1].wrapping_sub(cpu.registers[cpu.s2]);
    }
}

/// Subtract immediate instruction: rd = rs1 - immediate
pub struct SubImmediate;

impl Instruction for SubImmediate {
    fn execute(&self, cpu: &mut CPU, _memory: &mut Memory) {
        cpu.registers[cpu.d] = cpu.registers[cpu.s1].wrapping_sub(cpu.imm as u32);
    }
}

/// Multiply instruction: rd = rs1 * rs2 (signed)
pub struct Mul;

impl Instruction for Mul {
    fn execute(&self, cpu: &mut CPU, _memory: &mut Memory) {
        cpu.registers[cpu.d] = cpu.registers[cpu.s1].wrapping_mul(cpu.registers[cpu.s2]);
    }
}

/// Unsigned multiply instruction: rd = rs1 * rs2 (unsigned)
pub struct MulU;

impl Instruction for MulU {
    fn execute(&self, cpu: &mut CPU, _memory: &mut Memory) {
        let result = (cpu.registers[cpu.s1] as u64 * cpu.registers[cpu.s2] as u64) as u32;
        cpu.registers[cpu.d] = result;
    }
}

/// Divide instruction: rd = rs1 / rs2 (signed)
pub struct Div;

impl Instruction for Div {
    fn execute(&self, cpu: &mut CPU, _memory: &mut Memory) {
        let a = cpu.registers[cpu.s1] as i32;
        let b = cpu.registers[cpu.s2] as i32;
        if b == 0 {
            cpu.cr0 |= CPU::CR0_FP_DIVZERO;
            cpu.registers[cpu.d] = 0;
        } else if a == i32::MIN && b == -1 {
            // Handle MIN_INT / -1 overflow case
            cpu.registers[cpu.d] = a as u32;
        } else {
            cpu.registers[cpu.d] = (a / b) as u32;
        }
    }
}

/// Unsigned divide instruction: rd = rs1 / rs2 (unsigned)
pub struct DivU;

impl Instruction for DivU {
    fn execute(&self, cpu: &mut CPU, _memory: &mut Memory) {
        let a = cpu.registers[cpu.s1];
        let b = cpu.registers[cpu.s2];
        if b == 0 {
            cpu.cr0 |= CPU::CR0_FP_DIVZERO;
            cpu.registers[cpu.d] = 0;
        } else {
            cpu.registers[cpu.d] = a / b;
        }
    }
}

/// Mask register instruction: rd = rs1 & rs2
pub struct Mask;

impl Instruction for Mask {
    fn execute(&self, cpu: &mut CPU, _memory: &mut Memory) {
        let value = cpu.registers[cpu.s1];
        let mask = cpu.registers[cpu.s2];
        cpu.registers[cpu.d] = value & mask;
    }
}

/// Find first 1 instruction: finds position of first set bit
pub struct FF1;

impl Instruction for FF1 {
    fn execute(&self, cpu: &mut CPU, _memory: &mut Memory) {
        let value = cpu.registers[cpu.s1];
        let mut pos = 0;
        while pos < 32 && (value & (1 << pos)) == 0 {
            pos += 1;
        }
        cpu.registers[cpu.d] = pos;
    }
}

/// Find first 0 instruction: finds position of first clear bit
pub struct FF0;

impl Instruction for FF0 {
    fn execute(&self, cpu: &mut CPU, _memory: &mut Memory) {
        let value = cpu.registers[cpu.s1];
        let mut pos = 0;
        while pos < 32 && (value & (1 << pos)) != 0 {
            pos += 1;
        }
        cpu.registers[cpu.d] = pos;
    }
}

/// Add unsigned instruction: rd = rs1 + rs2 (unsigned)
pub struct AddU;

impl Instruction for AddU {
    fn execute(&self, cpu: &mut CPU, _memory: &mut Memory) {
        cpu.registers[cpu.d] = cpu.registers[cpu.s1].wrapping_add(cpu.registers[cpu.s2]);
    }
}

/// Add unsigned immediate instruction: rd = rs1 + immediate (unsigned)
pub struct AddUImmediate;

impl Instruction for AddUImmediate {
    fn execute(&self, cpu: &mut CPU, _memory: &mut Memory) {
        cpu.registers[cpu.d] = cpu.registers[cpu.s1].wrapping_add(cpu.imm as u32);
    }
}

/// Subtract unsigned instruction: rd = rs1 - rs2 (unsigned)
pub struct SubU;

impl Instruction for SubU {
    fn execute(&self, cpu: &mut CPU, _memory: &mut Memory) {
        cpu.registers[cpu.d] = cpu.registers[cpu.s1].wrapping_sub(cpu.registers[cpu.s2]);
    }
}

/// Subtract unsigned immediate instruction: rd = rs1 - immediate (unsigned)
pub struct SubUImmediate;

impl Instruction for SubUImmediate {
    fn execute(&self, cpu: &mut CPU, _memory: &mut Memory) {
        cpu.registers[cpu.d] = cpu.registers[cpu.s1].wrapping_sub(cpu.imm as u32);
    }
}

/// Compare instruction: sets condition codes based on signed comparison
pub struct Cmp;

impl Instruction for Cmp {
    fn execute(&self, cpu: &mut CPU, _memory: &mut Memory) {
        let a = cpu.registers[cpu.s1] as i32;
        let b = cpu.registers[cpu.s2] as i32;

        // Set condition codes
        if a == b {
            cpu.cr0 |= CPU::CR0_EQUAL;
            cpu.cr0 &= !CPU::CR0_LESS;
            cpu.cr0 &= !CPU::CR0_GREATER;
        } else if a < b {
            cpu.cr0 |= CPU::CR0_LESS;
            cpu.cr0 &= !CPU::CR0_EQUAL;
            cpu.cr0 &= !CPU::CR0_GREATER;
        } else {
            cpu.cr0 |= CPU::CR0_GREATER;
            cpu.cr0 &= !CPU::CR0_EQUAL;
            cpu.cr0 &= !CPU::CR0_LESS;
        }
    }
}

/// Compare unsigned instruction: sets condition codes based on unsigned comparison
pub struct CmpU;

impl Instruction for CmpU {
    fn execute(&self, cpu: &mut CPU, _memory: &mut Memory) {
        let a = cpu.registers[cpu.s1];
        let b = cpu.registers[cpu.s2];

        // Set condition codes
        if a == b {
            cpu.cr0 |= CPU::CR0_EQUAL;
            cpu.cr0 &= !CPU::CR0_LESS;
            cpu.cr0 &= !CPU::CR0_GREATER;
        } else if a < b {
            cpu.cr0 |= CPU::CR0_LESS;
            cpu.cr0 &= !CPU::CR0_EQUAL;
            cpu.cr0 &= !CPU::CR0_GREATER;
        } else {
            cpu.cr0 |= CPU::CR0_GREATER;
            cpu.cr0 &= !CPU::CR0_EQUAL;
            cpu.cr0 &= !CPU::CR0_LESS;
        }
    }
}

/// Long multiply instruction: 64-bit result in rd:rd+1
pub struct LMul;

impl Instruction for LMul {
    fn execute(&self, cpu: &mut CPU, _memory: &mut Memory) {
        let a = cpu.registers[cpu.s1] as i32 as i64;
        let b = cpu.registers[cpu.s2] as i32 as i64;
        let result = a.wrapping_mul(b);

        // Store high 32 bits in d, low 32 bits in d+1
        cpu.registers[cpu.d] = (result >> 32) as u32;
        cpu.registers[cpu.d.wrapping_add(1)] = result as u32;
    }
}

/// Long multiply unsigned instruction: 64-bit result in rd:rd+1
pub struct LMulU;

impl Instruction for LMulU {
    fn execute(&self, cpu: &mut CPU, _memory: &mut Memory) {
        let a = cpu.registers[cpu.s1] as u64;
        let b = cpu.registers[cpu.s2] as u64;
        let result = a.wrapping_mul(b);

        // Store high 32 bits in d, low 32 bits in d+1
        cpu.registers[cpu.d] = (result >> 32) as u32;
        cpu.registers[cpu.d.wrapping_add(1)] = result as u32;
    }
}

/// Double-precision divide unsigned instruction: quotient in rd, remainder in rd+1
pub struct DivUD;

impl Instruction for DivUD {
    fn execute(&self, cpu: &mut CPU, _memory: &mut Memory) {
        let dividend =
            ((cpu.registers[cpu.s1] as u64) << 32) | cpu.registers[cpu.s1.wrapping_add(1)] as u64;
        let divisor = cpu.registers[cpu.s2];

        if divisor == 0 {
            cpu.cr0 |= CPU::CR0_FP_DIVZERO;
            cpu.registers[cpu.d] = 0;
            cpu.registers[cpu.d.wrapping_add(1)] = 0;
        } else {
            let quotient = dividend / divisor as u64;
            let remainder = dividend % divisor as u64;

            cpu.registers[cpu.d] = quotient as u32;
            cpu.registers[cpu.d.wrapping_add(1)] = remainder as u32;
        }
    }
}

/// Remainder instruction: rd = rs1 % rs2
pub struct Rem;

impl Instruction for Rem {
    fn execute(&self, cpu: &mut CPU, _memory: &mut Memory) {
        let a = cpu.registers[cpu.s1] as i32;
        let b = cpu.registers[cpu.s2] as i32;

        if b == 0 {
            cpu.cr0 |= CPU::CR0_FP_DIVZERO;
            cpu.registers[cpu.d] = 0;
        } else {
            cpu.registers[cpu.d] = (a % b) as u32;
        }
    }
}

/// Remainder Unsigned instruction
pub struct RemU;

impl Instruction for RemU {
    fn execute(&self, cpu: &mut CPU, _memory: &mut Memory) {
        let a = cpu.registers[cpu.s1];
        let b = cpu.registers[cpu.s2];

        if b == 0 {
            cpu.cr0 |= CPU::CR0_FP_DIVZERO;
            cpu.registers[cpu.d] = 0;
        } else {
            cpu.registers[cpu.d] = a % b;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();

        cpu.registers[1] = 10;
        cpu.registers[2] = 20;
        cpu.d = 3;
        cpu.s1 = 1;
        cpu.s2 = 2;

        Add.execute(&mut cpu, &mut memory);
        assert_eq!(cpu.registers[3], 30);

        // Test overflow
        cpu.registers[1] = u32::MAX;
        cpu.registers[2] = 1;
        Add.execute(&mut cpu, &mut memory);
        assert_eq!(cpu.registers[3], 0);
    }

    #[test]
    fn test_add_immediate() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();

        cpu.registers[1] = 10;
        cpu.d = 2;
        cpu.s1 = 1;
        cpu.imm = 20;

        AddImmediate.execute(&mut cpu, &mut memory);
        assert_eq!(cpu.registers[2], 30);

        // Test negative immediate
        cpu.registers[1] = 30;
        cpu.imm = -10;
        AddImmediate.execute(&mut cpu, &mut memory);
        assert_eq!(cpu.registers[2], 20);
    }

    #[test]
    fn test_sub() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();

        cpu.registers[1] = 30;
        cpu.registers[2] = 20;
        cpu.d = 3;
        cpu.s1 = 1;
        cpu.s2 = 2;

        Sub.execute(&mut cpu, &mut memory);
        assert_eq!(cpu.registers[3], 10);

        // Test underflow
        cpu.registers[1] = 0;
        cpu.registers[2] = 1;
        Sub.execute(&mut cpu, &mut memory);
        assert_eq!(cpu.registers[3], u32::MAX);
    }

    #[test]
    fn test_sub_immediate() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();

        cpu.registers[1] = 30;
        cpu.d = 2;
        cpu.s1 = 1;
        cpu.imm = 20;

        SubImmediate.execute(&mut cpu, &mut memory);
        assert_eq!(cpu.registers[2], 10);

        // Test negative immediate
        cpu.registers[1] = 20;
        cpu.imm = -10;
        SubImmediate.execute(&mut cpu, &mut memory);
        assert_eq!(cpu.registers[2], 30);
    }

    #[test]
    fn test_mul() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();

        cpu.registers[1] = 5;
        cpu.registers[2] = 4;
        cpu.d = 3;
        cpu.s1 = 1;
        cpu.s2 = 2;

        Mul.execute(&mut cpu, &mut memory);
        assert_eq!(cpu.registers[3], 20);

        // Test signed multiplication
        cpu.registers[1] = -5i32 as u32;
        cpu.registers[2] = 4;
        Mul.execute(&mut cpu, &mut memory);
        assert_eq!(cpu.registers[3] as i32, -20);
    }

    #[test]
    fn test_mulu() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();

        cpu.registers[1] = 5;
        cpu.registers[2] = 4;
        cpu.d = 3;
        cpu.s1 = 1;
        cpu.s2 = 2;

        MulU.execute(&mut cpu, &mut memory);
        assert_eq!(cpu.registers[3], 20);

        // Test large numbers
        cpu.registers[1] = 0xFFFFFFFF;
        cpu.registers[2] = 2;
        MulU.execute(&mut cpu, &mut memory);
        assert_eq!(cpu.registers[3], 0xFFFFFFFE);
    }

    #[test]
    fn test_mask() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();

        cpu.registers[1] = 0xFFFFFFFF;
        cpu.registers[2] = 0x0000FFFF;
        cpu.d = 3;
        cpu.s1 = 1;
        cpu.s2 = 2;

        Mask.execute(&mut cpu, &mut memory);
        assert_eq!(cpu.registers[3], 0x0000FFFF);
    }

    #[test]
    fn test_ff0() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();

        // Test finding first 0 in various positions
        cpu.registers[1] = 0xFFFFFFFE; // First 0 at position 0
        cpu.d = 2;
        cpu.s1 = 1;

        FF0.execute(&mut cpu, &mut memory);
        assert_eq!(cpu.registers[2], 0);

        cpu.registers[1] = 0xFFFFFEFF; // First 0 at position 8
        FF0.execute(&mut cpu, &mut memory);
        assert_eq!(cpu.registers[2], 8);

        cpu.registers[1] = 0x7FFFFFFF; // First 0 at position 31
        FF0.execute(&mut cpu, &mut memory);
        assert_eq!(cpu.registers[2], 31);

        // Test with no 0s
        cpu.registers[1] = 0xFFFFFFFF;
        FF0.execute(&mut cpu, &mut memory);
        assert_eq!(cpu.registers[2], 32);
    }

    #[test]
    fn test_ff1() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();

        // Test finding first 1 in various positions
        cpu.registers[1] = 0x00000001; // First 1 at position 0
        cpu.d = 2;
        cpu.s1 = 1;

        FF1.execute(&mut cpu, &mut memory);
        assert_eq!(cpu.registers[2], 0);

        cpu.registers[1] = 0x00000100; // First 1 at position 8
        FF1.execute(&mut cpu, &mut memory);
        assert_eq!(cpu.registers[2], 8);

        cpu.registers[1] = 0x80000000; // First 1 at position 31
        FF1.execute(&mut cpu, &mut memory);
        assert_eq!(cpu.registers[2], 31);

        // Test with no 1s
        cpu.registers[1] = 0;
        FF1.execute(&mut cpu, &mut memory);
        assert_eq!(cpu.registers[2], 32);
    }

    #[test]
    fn test_div() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();

        // Test normal division
        cpu.registers[1] = 20;
        cpu.registers[2] = 5;
        cpu.d = 3;
        cpu.s1 = 1;
        cpu.s2 = 2;

        Div.execute(&mut cpu, &mut memory);
        assert_eq!(cpu.registers[3], 4);

        // Test negative division
        cpu.registers[1] = (-20i32) as u32;
        cpu.registers[2] = 5;
        Div.execute(&mut cpu, &mut memory);
        assert_eq!(cpu.registers[3] as i32, -4);

        // Test division by zero
        cpu.registers[2] = 0;
        Div.execute(&mut cpu, &mut memory);
        assert_ne!(cpu.cr0 & CPU::CR0_FP_DIVZERO, 0);
    }

    #[test]
    fn test_divu() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();

        // Test normal division
        cpu.registers[1] = 20;
        cpu.registers[2] = 5;
        cpu.d = 3;
        cpu.s1 = 1;
        cpu.s2 = 2;

        DivU.execute(&mut cpu, &mut memory);
        assert_eq!(cpu.registers[3], 4);

        // Test large numbers
        cpu.registers[1] = 0xFFFFFFFF;
        cpu.registers[2] = 2;
        DivU.execute(&mut cpu, &mut memory);
        assert_eq!(cpu.registers[3], 0x7FFFFFFF);

        // Test division by zero
        cpu.registers[2] = 0;
        DivU.execute(&mut cpu, &mut memory);
        assert_ne!(cpu.cr0 & CPU::CR0_FP_DIVZERO, 0);
    }

    #[test]
    fn test_addu() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();

        cpu.registers[1] = 0xFFFFFFFF;
        cpu.registers[2] = 1;
        cpu.d = 3;
        cpu.s1 = 1;
        cpu.s2 = 2;

        AddU.execute(&mut cpu, &mut memory);
        assert_eq!(cpu.registers[3], 0); // Unsigned overflow wraps
    }

    #[test]
    fn test_subu() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();

        cpu.registers[1] = 0;
        cpu.registers[2] = 1;
        cpu.d = 3;
        cpu.s1 = 1;
        cpu.s2 = 2;

        SubU.execute(&mut cpu, &mut memory);
        assert_eq!(cpu.registers[3], 0xFFFFFFFF); // Unsigned underflow wraps
    }

    #[test]
    fn test_cmp() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();

        // Test equal
        cpu.registers[1] = 10;
        cpu.registers[2] = 10;
        cpu.s1 = 1;
        cpu.s2 = 2;

        Cmp.execute(&mut cpu, &mut memory);
        assert_ne!(cpu.cr0 & CPU::CR0_EQUAL, 0);
        assert_eq!(cpu.cr0 & CPU::CR0_LESS, 0);
        assert_eq!(cpu.cr0 & CPU::CR0_GREATER, 0);

        // Test less than
        cpu.registers[1] = -10i32 as u32;
        cpu.registers[2] = 10;
        cpu.cr0 = 0;

        Cmp.execute(&mut cpu, &mut memory);
        assert_eq!(cpu.cr0 & CPU::CR0_EQUAL, 0);
        assert_ne!(cpu.cr0 & CPU::CR0_LESS, 0);
        assert_eq!(cpu.cr0 & CPU::CR0_GREATER, 0);

        // Test greater than
        cpu.registers[1] = 20;
        cpu.registers[2] = 10;
        cpu.cr0 = 0;

        Cmp.execute(&mut cpu, &mut memory);
        assert_eq!(cpu.cr0 & CPU::CR0_EQUAL, 0);
        assert_eq!(cpu.cr0 & CPU::CR0_LESS, 0);
        assert_ne!(cpu.cr0 & CPU::CR0_GREATER, 0);
    }

    #[test]
    fn test_lmul() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();

        // Test normal multiplication
        cpu.registers[1] = 0x12345678;
        cpu.registers[2] = 0x11111111;
        cpu.d = 3;
        cpu.s1 = 1;
        cpu.s2 = 2;

        LMul.execute(&mut cpu, &mut memory);

        // Expected result: 0x12345678 * 0x11111111
        let expected = (0x12345678i64 * 0x11111111i64) as u64;
        let actual = ((cpu.registers[3] as u64) << 32) | cpu.registers[4] as u64;
        assert_eq!(actual, expected as u64);

        // Test negative numbers
        cpu.registers[1] = (-1i32) as u32;
        cpu.registers[2] = 2;

        LMul.execute(&mut cpu, &mut memory);
        let expected = (-2i64) as u64;
        let actual = ((cpu.registers[3] as u64) << 32) | cpu.registers[4] as u64;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_divud() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();

        // Set up a 64-bit dividend
        cpu.registers[1] = 0x00000000; // High word
        cpu.registers[2] = 0x00000064; // Low word (100 in decimal)
        cpu.registers[3] = 0x00000002; // Divisor
        cpu.s1 = 1;
        cpu.s2 = 3;
        cpu.d = 4;

        DivUD.execute(&mut cpu, &mut memory);
        assert_eq!(cpu.registers[4], 50); // Quotient
        assert_eq!(cpu.registers[5], 0); // Remainder

        // Test division by zero
        cpu.registers[3] = 0; // Divisor
        cpu.cr0 = 0;

        DivUD.execute(&mut cpu, &mut memory);
        assert_ne!(cpu.cr0 & CPU::CR0_FP_DIVZERO, 0);
        assert_eq!(cpu.registers[4], 0);
        assert_eq!(cpu.registers[5], 0);
    }

    #[test]
    fn test_rem() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();

        // Test positive numbers
        cpu.registers[1] = 100;
        cpu.registers[2] = 30;
        cpu.d = 3;
        cpu.s1 = 1;
        cpu.s2 = 2;

        Rem.execute(&mut cpu, &mut memory);
        assert_eq!(cpu.registers[3], 10);

        // Test negative dividend
        cpu.registers[1] = (-100i32) as u32;
        cpu.registers[2] = 30;

        Rem.execute(&mut cpu, &mut memory);
        assert_eq!(cpu.registers[3] as i32, -10);

        // Test division by zero
        cpu.registers[2] = 0;
        cpu.cr0 = 0;

        Rem.execute(&mut cpu, &mut memory);
        assert_ne!(cpu.cr0 & CPU::CR0_FP_DIVZERO, 0);
        assert_eq!(cpu.registers[3], 0);
    }

    #[test]
    fn test_remu() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();

        // Test normal case
        cpu.registers[1] = 100;
        cpu.registers[2] = 30;
        cpu.d = 3;
        cpu.s1 = 1;
        cpu.s2 = 2;

        RemU.execute(&mut cpu, &mut memory);
        assert_eq!(cpu.registers[3], 10);

        // Test large numbers
        cpu.registers[1] = 0xFFFFFFFF;
        cpu.registers[2] = 0x10000000;

        RemU.execute(&mut cpu, &mut memory);
        assert_eq!(cpu.registers[3], 0x0FFFFFFF);

        // Test division by zero
        cpu.registers[2] = 0;
        cpu.cr0 = 0;

        RemU.execute(&mut cpu, &mut memory);
        assert_ne!(cpu.cr0 & CPU::CR0_FP_DIVZERO, 0);
        assert_eq!(cpu.registers[3], 0);
    }

    #[test]
    fn test_add_overflow() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();

        // Test positive overflow
        cpu.registers[1] = 0x7FFFFFFF; // Max positive 32-bit int
        cpu.registers[2] = 1;
        cpu.d = 3;
        cpu.s1 = 1;
        cpu.s2 = 2;

        Add.execute(&mut cpu, &mut memory);
        assert_eq!(cpu.registers[3], 0x80000000); // Should wrap to negative

        // Test negative overflow
        cpu.registers[1] = 0x80000000; // Min negative 32-bit int
        cpu.registers[2] = 0xFFFFFFFF; // -1 in two's complement
        Add.execute(&mut cpu, &mut memory);
        assert_eq!(cpu.registers[3], 0x7FFFFFFF); // Should wrap to positive
    }

    #[test]
    fn test_sub_underflow() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();

        // Test positive to negative underflow
        cpu.registers[1] = 0;
        cpu.registers[2] = 1;
        cpu.d = 3;
        cpu.s1 = 1;
        cpu.s2 = 2;

        Sub.execute(&mut cpu, &mut memory);
        assert_eq!(cpu.registers[3], 0xFFFFFFFF); // -1 in two's complement

        // Test negative to positive underflow
        cpu.registers[1] = 0x80000000; // Min negative 32-bit int
        cpu.registers[2] = 0xFFFFFFFF; // -1 in two's complement
        Sub.execute(&mut cpu, &mut memory);
        assert_eq!(cpu.registers[3], 0x80000001);
    }

    #[test]
    fn test_div_by_zero() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();

        // Test division by zero
        cpu.registers[1] = 42;
        cpu.registers[2] = 0;
        cpu.d = 3;
        cpu.s1 = 1;
        cpu.s2 = 2;

        Div.execute(&mut cpu, &mut memory);
        assert_eq!(cpu.registers[3], 0);

        // Test unsigned division by zero
        DivU.execute(&mut cpu, &mut memory);
        assert_eq!(cpu.registers[3], 0);
    }

    #[test]
    fn test_div_overflow() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();

        // Test division overflow (MIN_INT / -1)
        cpu.registers[1] = 0x80000000; // Min negative 32-bit int
        cpu.registers[2] = 0xFFFFFFFF; // -1 in two's complement
        cpu.d = 3;
        cpu.s1 = 1;
        cpu.s2 = 2;

        Div.execute(&mut cpu, &mut memory);
        assert_eq!(cpu.registers[3], 0x80000000); // Should remain MIN_INT
    }

    #[test]
    fn test_mul_overflow() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();

        // Test multiplication overflow
        cpu.registers[1] = 0x7FFFFFFF; // Max positive 32-bit int
        cpu.registers[2] = 2;
        cpu.d = 3;
        cpu.s1 = 1;
        cpu.s2 = 2;

        Mul.execute(&mut cpu, &mut memory);
        assert_eq!(cpu.registers[3], 0xFFFFFFFE); // Wrapped result

        // Test negative multiplication overflow
        cpu.registers[1] = 0x80000000; // Min negative 32-bit int
        cpu.registers[2] = 2;
        Mul.execute(&mut cpu, &mut memory);
        assert_eq!(cpu.registers[3], 0); // Wrapped result
    }

    #[test]
    fn test_rem_by_zero() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();

        // Test remainder by zero
        cpu.registers[1] = 42;
        cpu.registers[2] = 0;
        cpu.d = 3;
        cpu.s1 = 1;
        cpu.s2 = 2;

        Rem.execute(&mut cpu, &mut memory);
        assert_eq!(cpu.registers[3], 0);

        // Test unsigned remainder by zero
        RemU.execute(&mut cpu, &mut memory);
        assert_eq!(cpu.registers[3], 0);
    }

    #[test]
    fn test_addu_wraparound() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();

        // Test unsigned addition wraparound
        cpu.registers[1] = 0xFFFFFFFF;
        cpu.registers[2] = 1;
        cpu.d = 3;
        cpu.s1 = 1;
        cpu.s2 = 2;

        AddU.execute(&mut cpu, &mut memory);
        assert_eq!(cpu.registers[3], 0); // Should wrap to 0
    }

    #[test]
    fn test_subu_wraparound() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();

        // Test unsigned subtraction wraparound
        cpu.registers[1] = 0;
        cpu.registers[2] = 1;
        cpu.d = 3;
        cpu.s1 = 1;
        cpu.s2 = 2;

        SubU.execute(&mut cpu, &mut memory);
        assert_eq!(cpu.registers[3], 0xFFFFFFFF); // Should wrap to max unsigned
    }

    #[test]
    fn test_mul_boundary_cases() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();

        // Test multiplication by 0
        cpu.registers[1] = 0xFFFFFFFF;
        cpu.registers[2] = 0;
        cpu.d = 3;
        cpu.s1 = 1;
        cpu.s2 = 2;

        Mul.execute(&mut cpu, &mut memory);
        assert_eq!(cpu.registers[3], 0);

        // Test multiplication by 1
        cpu.registers[1] = 0x12345678;
        cpu.registers[2] = 1;
        Mul.execute(&mut cpu, &mut memory);
        assert_eq!(cpu.registers[3], 0x12345678);

        // Test multiplication by -1
        cpu.registers[1] = 0x12345678;
        cpu.registers[2] = 0xFFFFFFFF; // -1 in two's complement
        Mul.execute(&mut cpu, &mut memory);
        assert_eq!(cpu.registers[3], 0xEDCBA988); // Negated value
    }

    #[test]
    fn test_div_boundary_cases() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();

        // Test division by 1
        cpu.registers[1] = 0x12345678;
        cpu.registers[2] = 1;
        cpu.d = 3;
        cpu.s1 = 1;
        cpu.s2 = 2;

        Div.execute(&mut cpu, &mut memory);
        assert_eq!(cpu.registers[3], 0x12345678);

        // Test division by -1 (normal case)
        cpu.registers[1] = 0x12345678;
        cpu.registers[2] = 0xFFFFFFFF; // -1 in two's complement
        Div.execute(&mut cpu, &mut memory);
        assert_eq!(cpu.registers[3], 0xEDCBA988); // Negated value

        // Test 0 divided by any number
        cpu.registers[1] = 0;
        cpu.registers[2] = 0x12345678;
        Div.execute(&mut cpu, &mut memory);
        assert_eq!(cpu.registers[3], 0);
    }
}
