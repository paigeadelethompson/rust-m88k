//! Floating point instruction implementations for the Motorola 88000.
//! 
//! This module contains implementations of all floating point operations including:
//! - Basic arithmetic (add, subtract, multiply, divide)
//! - Comparisons
//! - Type conversions
//! - Special value handling (NaN, infinity)
//! - Exception handling

use crate::cpu::CPU;
use crate::memory::Memory;
use crate::cpu::instructions::Instruction;

/// Floating point add instruction: rd = rs1 + rs2
pub struct FAdd;

impl Instruction for FAdd {
    fn execute(&self, cpu: &mut CPU, _memory: &mut Memory) {
        let a = f32::from_bits(cpu.registers[cpu.s1]);
        let b = f32::from_bits(cpu.registers[cpu.s2]);
        let result = a + b;
        
        // Check for floating point exceptions
        if result.is_infinite() && !a.is_infinite() && !b.is_infinite() {
            cpu.set_fp_flag(CPU::CR0_FP_OVERFLOW);
        }
        if result == 0.0 && (a != 0.0 || b != 0.0) {
            cpu.set_fp_flag(CPU::CR0_FP_UNDERFLOW);
        }
        
        cpu.registers[cpu.d] = result.to_bits();
    }
}

/// Floating point subtract instruction: rd = rs1 - rs2
pub struct FSub;

impl Instruction for FSub {
    fn execute(&self, cpu: &mut CPU, _memory: &mut Memory) {
        let a = f32::from_bits(cpu.registers[cpu.s1]);
        let b = f32::from_bits(cpu.registers[cpu.s2]);
        let result = a - b;
        
        if result.is_nan() {
            cpu.cr0 |= CPU::CR0_FP_INVALID;
        }
        
        cpu.registers[cpu.d] = result.to_bits();
    }
}

/// Floating point multiply instruction: rd = rs1 * rs2
pub struct FMul;

impl Instruction for FMul {
    fn execute(&self, cpu: &mut CPU, _memory: &mut Memory) {
        let a = f32::from_bits(cpu.registers[cpu.s1]);
        let b = f32::from_bits(cpu.registers[cpu.s2]);
        let result = a * b;
        
        // Check for floating point exceptions
        if result.is_infinite() && !a.is_infinite() && !b.is_infinite() {
            cpu.set_fp_flag(CPU::CR0_FP_OVERFLOW);
        }
        if result == 0.0 && a != 0.0 && b != 0.0 {
            cpu.set_fp_flag(CPU::CR0_FP_UNDERFLOW);
        }
        
        cpu.registers[cpu.d] = result.to_bits();
    }
}

/// Floating point divide instruction: rd = rs1 / rs2
pub struct FDiv;

impl Instruction for FDiv {
    fn execute(&self, cpu: &mut CPU, _memory: &mut Memory) {
        let a = f32::from_bits(cpu.registers[cpu.s1]);
        let b = f32::from_bits(cpu.registers[cpu.s2]);
        
        // Check for division by zero
        if b == 0.0 {
            cpu.set_fp_flag(CPU::CR0_FP_DIVZERO);
            if a == 0.0 {
                // 0.0 / 0.0 = NaN
                cpu.registers[cpu.d] = f32::NAN.to_bits();
                cpu.set_fp_flag(CPU::CR0_FP_INVALID);
            } else {
                // x / 0 = infinity with sign of x
                cpu.registers[cpu.d] = if a.is_sign_positive() {
                    f32::INFINITY.to_bits()
                } else {
                    f32::NEG_INFINITY.to_bits()
                };
            }
            return;
        }
        
        let result = a / b;
        
        // Check for floating point exceptions
        if result.is_infinite() && !a.is_infinite() {
            cpu.set_fp_flag(CPU::CR0_FP_OVERFLOW);
        }
        if result == 0.0 && a != 0.0 {
            cpu.set_fp_flag(CPU::CR0_FP_UNDERFLOW);
        }
        
        cpu.registers[cpu.d] = result.to_bits();
    }
}

/// Floating point compare instruction: sets condition codes based on rs1 ? rs2
pub struct FCmp;

impl Instruction for FCmp {
    fn execute(&self, cpu: &mut CPU, _memory: &mut Memory) {
        let a = f32::from_bits(cpu.registers[cpu.s1]);
        let b = f32::from_bits(cpu.registers[cpu.s2]);
        
        cpu.cr0 &= !CPU::CR0_FP_COMPARE_MASK;
        if a.is_nan() || b.is_nan() {
            cpu.cr0 |= CPU::CR0_FP_UNORDERED;
        } else if a < b {
            cpu.cr0 |= CPU::CR0_FP_LESS;
        } else if a > b {
            cpu.cr0 |= CPU::CR0_FP_GREATER;
        } else {
            cpu.cr0 |= CPU::CR0_FP_EQUAL;
        }
    }
}

/// Integer to floating point conversion instruction: rd = float(rs1)
pub struct IntToFp;

impl Instruction for IntToFp {
    fn execute(&self, cpu: &mut CPU, _memory: &mut Memory) {
        let int_val = cpu.registers[cpu.s1] as i32;
        let float_val = int_val as f32;
        cpu.registers[cpu.d] = float_val.to_bits();
    }
}

/// Floating point to integer conversion instruction: rd = int(rs1)
pub struct FpToInt;

impl Instruction for FpToInt {
    fn execute(&self, cpu: &mut CPU, _memory: &mut Memory) {
        let value = f32::from_bits(cpu.registers[cpu.s1]);
        
        // Check for NaN or infinity
        if value.is_nan() || value.is_infinite() {
            cpu.set_fp_flag(CPU::CR0_FP_INVALID);
            cpu.registers[cpu.d] = 0;
            return;
        }
        
        // Check for overflow
        if value > i32::MAX as f32 || value < i32::MIN as f32 {
            cpu.set_fp_flag(CPU::CR0_FP_OVERFLOW);
            cpu.registers[cpu.d] = if value > 0.0 { i32::MAX } else { i32::MIN } as u32;
            return;
        }
        
        // Round to nearest even integer
        let rounded = value.round();
        // If we're exactly halfway between two integers, round to even
        let result = if (value.fract().abs() - 0.5).abs() < f32::EPSILON {
            let floor = value.floor();
            if floor as i32 % 2 == 0 {
                floor
            } else {
                value.ceil()
            }
        } else {
            rounded
        };
        
        cpu.registers[cpu.d] = result as i32 as u32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fadd() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();

        // Test normal addition
        cpu.registers[1] = f32::to_bits(3.14);
        cpu.registers[2] = f32::to_bits(2.86);
        cpu.d = 3;
        cpu.s1 = 1;
        cpu.s2 = 2;

        FAdd.execute(&mut cpu, &mut memory);
        assert_eq!(f32::from_bits(cpu.registers[3]), 6.0);

        // Test with zero
        cpu.registers[1] = f32::to_bits(3.14);
        cpu.registers[2] = f32::to_bits(0.0);
        FAdd.execute(&mut cpu, &mut memory);
        assert_eq!(f32::from_bits(cpu.registers[3]), 3.14);

        // Test overflow
        cpu.registers[1] = f32::to_bits(f32::MAX);
        cpu.registers[2] = f32::to_bits(f32::MAX);
        FAdd.execute(&mut cpu, &mut memory);
        assert!(f32::from_bits(cpu.registers[3]).is_infinite());
        assert_ne!(cpu.cr0 & CPU::CR0_FP_OVERFLOW, 0);
    }

    #[test]
    fn test_fsub() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();
        
        cpu.s1 = 1;
        cpu.s2 = 2;
        cpu.d = 3;
        
        // Test normal subtraction
        cpu.registers[1] = 3.0f32.to_bits();
        cpu.registers[2] = 1.5f32.to_bits();
        FSub.execute(&mut cpu, &mut memory);
        assert_eq!(f32::from_bits(cpu.registers[3]), 1.5);
        
        // Test negative result
        cpu.registers[1] = 1.0f32.to_bits();
        cpu.registers[2] = 2.0f32.to_bits();
        FSub.execute(&mut cpu, &mut memory);
        assert_eq!(f32::from_bits(cpu.registers[3]), -1.0);
        
        // Test subtraction with infinity
        cpu.registers[1] = f32::INFINITY.to_bits();
        cpu.registers[2] = f32::INFINITY.to_bits();
        FSub.execute(&mut cpu, &mut memory);
        assert!(f32::from_bits(cpu.registers[3]).is_nan());
        assert_ne!(cpu.cr0 & CPU::CR0_FP_INVALID, 0);
    }

    #[test]
    fn test_fmul() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();

        // Test normal multiplication
        cpu.registers[1] = f32::to_bits(3.0);
        cpu.registers[2] = f32::to_bits(2.0);
        cpu.d = 3;
        cpu.s1 = 1;
        cpu.s2 = 2;

        FMul.execute(&mut cpu, &mut memory);
        assert_eq!(f32::from_bits(cpu.registers[3]), 6.0);

        // Test with zero
        cpu.registers[1] = f32::to_bits(3.14);
        cpu.registers[2] = f32::to_bits(0.0);
        FMul.execute(&mut cpu, &mut memory);
        assert_eq!(f32::from_bits(cpu.registers[3]), 0.0);

        // Test overflow
        cpu.registers[1] = f32::to_bits(f32::MAX);
        cpu.registers[2] = f32::to_bits(2.0);
        FMul.execute(&mut cpu, &mut memory);
        assert!(f32::from_bits(cpu.registers[3]).is_infinite());
        assert_ne!(cpu.cr0 & CPU::CR0_FP_OVERFLOW, 0);
    }

    #[test]
    fn test_fdiv() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();

        // Test normal division
        cpu.registers[1] = f32::to_bits(6.0);
        cpu.registers[2] = f32::to_bits(2.0);
        cpu.d = 3;
        cpu.s1 = 1;
        cpu.s2 = 2;

        FDiv.execute(&mut cpu, &mut memory);
        assert_eq!(f32::from_bits(cpu.registers[3]), 3.0);

        // Test division by zero
        cpu.registers[1] = f32::to_bits(1.0);
        cpu.registers[2] = f32::to_bits(0.0);
        FDiv.execute(&mut cpu, &mut memory);
        assert!(f32::from_bits(cpu.registers[3]).is_infinite());
        assert_ne!(cpu.cr0 & CPU::CR0_FP_DIVZERO, 0);

        // Test underflow
        cpu.registers[1] = f32::to_bits(f32::MIN_POSITIVE);
        cpu.registers[2] = f32::to_bits(f32::MAX);
        FDiv.execute(&mut cpu, &mut memory);
        assert_ne!(cpu.cr0 & CPU::CR0_FP_UNDERFLOW, 0);
    }

    #[test]
    fn test_fcmp() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();
        
        // Setup registers for comparison
        cpu.s1 = 1;
        cpu.s2 = 2;
        
        // Test equal values
        cpu.registers[1] = 1.0f32.to_bits();
        cpu.registers[2] = 1.0f32.to_bits();
        FCmp.execute(&mut cpu, &mut memory);
        assert_eq!(cpu.cr0 & CPU::CR0_FP_COMPARE_MASK, CPU::CR0_FP_EQUAL);
        
        // Test less than
        cpu.registers[1] = 0.5f32.to_bits();
        cpu.registers[2] = 1.0f32.to_bits();
        FCmp.execute(&mut cpu, &mut memory);
        assert_eq!(cpu.cr0 & CPU::CR0_FP_COMPARE_MASK, CPU::CR0_FP_LESS);
        
        // Test greater than
        cpu.registers[1] = 2.0f32.to_bits();
        cpu.registers[2] = 1.0f32.to_bits();
        FCmp.execute(&mut cpu, &mut memory);
        assert_eq!(cpu.cr0 & CPU::CR0_FP_COMPARE_MASK, CPU::CR0_FP_GREATER);
        
        // Test NaN
        cpu.registers[1] = f32::NAN.to_bits();
        cpu.registers[2] = 1.0f32.to_bits();
        FCmp.execute(&mut cpu, &mut memory);
        assert_eq!(cpu.cr0 & CPU::CR0_FP_COMPARE_MASK, CPU::CR0_FP_UNORDERED);
    }

    #[test]
    fn test_int_to_fp() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();

        // Test positive integer
        cpu.registers[1] = 42;
        cpu.d = 2;
        cpu.s1 = 1;

        IntToFp.execute(&mut cpu, &mut memory);
        assert_eq!(f32::from_bits(cpu.registers[2]), 42.0);

        // Test negative integer
        cpu.registers[1] = -42i32 as u32;
        IntToFp.execute(&mut cpu, &mut memory);
        assert_eq!(f32::from_bits(cpu.registers[2]), -42.0);

        // Test zero
        cpu.registers[1] = 0;
        IntToFp.execute(&mut cpu, &mut memory);
        assert_eq!(f32::from_bits(cpu.registers[2]), 0.0);
    }

    #[test]
    fn test_fp_to_int() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();
        
        cpu.s1 = 1;
        cpu.d = 2;
        
        // Test normal conversion
        cpu.registers[1] = 42.5f32.to_bits();
        FpToInt.execute(&mut cpu, &mut memory);
        assert_eq!(cpu.registers[2], 42);
        
        // Test negative number
        cpu.registers[1] = (-42.5f32).to_bits();
        FpToInt.execute(&mut cpu, &mut memory);
        assert_eq!(cpu.registers[2] as i32, -42);
        
        // Test overflow
        cpu.registers[1] = (2147483648.0f32).to_bits();
        FpToInt.execute(&mut cpu, &mut memory);
        assert_eq!(cpu.registers[2], 0x7FFFFFFF);
        
        // Test underflow
        cpu.registers[1] = (-2147483904.0f32).to_bits();
        FpToInt.execute(&mut cpu, &mut memory);
        assert_eq!(cpu.registers[2], 0x80000000);
        
        // Test NaN
        cpu.registers[1] = f32::NAN.to_bits();
        FpToInt.execute(&mut cpu, &mut memory);
        assert_eq!(cpu.registers[2], 0);
        assert_ne!(cpu.cr0 & CPU::CR0_FP_INVALID, 0);
    }

    #[test]
    fn test_float_div_by_zero() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();
        
        // Set up division by zero
        cpu.registers[1] = f32::to_bits(1.0);
        cpu.registers[2] = f32::to_bits(0.0);
        cpu.d = 3;
        cpu.s1 = 1;
        cpu.s2 = 2;
        
        FDiv.execute(&mut cpu, &mut memory);
        
        // Result should be infinity
        assert!(f32::from_bits(cpu.registers[3]).is_infinite());
    }

    #[test]
    fn test_float_invalid_operations() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();
        
        // Test 0.0 / 0.0 (NaN)
        cpu.registers[1] = f32::to_bits(0.0);
        cpu.registers[2] = f32::to_bits(0.0);
        cpu.d = 3;
        cpu.s1 = 1;
        cpu.s2 = 2;
        
        FDiv.execute(&mut cpu, &mut memory);
        assert!(f32::from_bits(cpu.registers[3]).is_nan());
        
        // Test infinity - infinity (NaN)
        cpu.registers[1] = f32::to_bits(f32::INFINITY);
        cpu.registers[2] = f32::to_bits(f32::INFINITY);
        FSub.execute(&mut cpu, &mut memory);
        assert!(f32::from_bits(cpu.registers[3]).is_nan());
    }

    #[test]
    fn test_float_overflow() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();
        
        // Test multiplication leading to overflow
        cpu.registers[1] = f32::to_bits(f32::MAX);
        cpu.registers[2] = f32::to_bits(2.0);
        cpu.d = 3;
        cpu.s1 = 1;
        cpu.s2 = 2;
        
        FMul.execute(&mut cpu, &mut memory);
        assert!(f32::from_bits(cpu.registers[3]).is_infinite());
    }

    #[test]
    fn test_float_underflow() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();
        
        // Test multiplication leading to underflow
        cpu.registers[1] = f32::to_bits(f32::MIN_POSITIVE);
        cpu.registers[2] = f32::to_bits(0.5);
        cpu.d = 3;
        cpu.s1 = 1;
        cpu.s2 = 2;
        
        FMul.execute(&mut cpu, &mut memory);
        
        // Result should be denormalized or zero
        let result = f32::from_bits(cpu.registers[3]);
        assert!(result.is_subnormal() || result == 0.0);
    }

    #[test]
    fn test_float_rounding_modes() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();
        
        // Test rounding of 1.5 to integer
        cpu.registers[1] = f32::to_bits(1.5);
        cpu.d = 2;
        cpu.s1 = 1;
        
        FpToInt.execute(&mut cpu, &mut memory);
        assert_eq!(cpu.registers[2], 2);  // Should round up
        
        // TODO: Add tests for other rounding modes when implemented
        // - Round toward zero
        // - Round toward +infinity
        // - Round toward -infinity
    }

    #[test]
    fn test_float_compare_special_values() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();
        
        // Test NaN comparisons
        cpu.registers[1] = f32::to_bits(f32::NAN);
        cpu.registers[2] = f32::to_bits(0.0);
        cpu.d = 3;
        cpu.s1 = 1;
        cpu.s2 = 2;
        
        FCmp.execute(&mut cpu, &mut memory);
        assert_ne!(cpu.cr0 & CPU::CR0_FP_UNORDERED, 0);
        
        // Test infinity comparisons
        cpu.registers[1] = f32::to_bits(f32::INFINITY);
        cpu.registers[2] = f32::to_bits(f32::MAX);
        FCmp.execute(&mut cpu, &mut memory);
        assert_ne!(cpu.cr0 & CPU::CR0_FP_GREATER, 0);
        
        // Test -infinity comparisons
        cpu.registers[1] = f32::to_bits(f32::NEG_INFINITY);
        cpu.registers[2] = f32::to_bits(-f32::MAX);
        FCmp.execute(&mut cpu, &mut memory);
        assert_ne!(cpu.cr0 & CPU::CR0_FP_LESS, 0);
    }

    #[test]
    fn test_float_denormal_handling() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();
        
        // Create a denormal number
        let denormal = f32::from_bits(1);  // Smallest possible denormal
        cpu.registers[1] = f32::to_bits(denormal);
        cpu.registers[2] = f32::to_bits(2.0);
        cpu.d = 3;
        cpu.s1 = 1;
        cpu.s2 = 2;
        
        FMul.execute(&mut cpu, &mut memory);
        
        // Result should still be denormal
        let result = f32::from_bits(cpu.registers[3]);
        assert!(result.is_subnormal());
    }
} 