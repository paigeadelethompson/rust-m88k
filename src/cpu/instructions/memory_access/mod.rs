//! Memory access instruction implementations for the Motorola 88000.
//!
//! This module contains implementations of all memory access operations including:
//! - Load and store operations
//! - Different data size variants (byte, half-word, word, double)
//! - Atomic memory operations
//! - Memory access with MMU support

use crate::cpu::instructions::Instruction;
use crate::cpu::CPU;
use crate::memory::{Memory, MemoryError};

/// Load instruction: rd = Memory[rs1 + offset]
#[derive(Debug)]
pub struct Load {
    pub rd: usize,
    pub rs1: usize,
    pub offset: i16,
}

/// Store instruction: Memory[rs1 + offset] = rd
#[derive(Debug)]
pub struct Store {
    pub rd: usize,
    pub rs1: usize,
    pub offset: i16,
}

impl Instruction for Load {
    fn execute(&self, cpu: &mut CPU, memory: &mut Memory) {
        let addr = cpu.registers[self.rs1].wrapping_add(self.offset as u32);
        match memory.read_word(addr) {
            Ok(value) => cpu.registers[self.rd] = value,
            Err(MemoryError::PageFault(_)) => cpu.set_page_fault(),
            Err(MemoryError::WriteProtection(_)) => cpu.set_write_protect_fault(),
            Err(_) => cpu.set_page_fault(),
        }
    }
}

impl Instruction for Store {
    fn execute(&self, cpu: &mut CPU, memory: &mut Memory) {
        let addr = cpu.registers[self.rs1].wrapping_add(self.offset as u32);
        match memory.write_word(addr, cpu.registers[self.rd]) {
            Ok(_) => (),
            Err(MemoryError::PageFault(_)) => cpu.set_page_fault(),
            Err(MemoryError::WriteProtection(_)) => cpu.set_write_protect_fault(),
            Err(_) => cpu.set_page_fault(),
        }
    }
}

/// Load byte instruction: rd = SignExtend(Memory[rs1 + offset])
pub struct LoadByte;

impl Instruction for LoadByte {
    fn execute(&self, cpu: &mut CPU, memory: &mut Memory) {
        let addr = cpu.registers[cpu.s1].wrapping_add(cpu.offset as u32);
        match memory.read_byte(addr) {
            Ok(value) => cpu.registers[cpu.d] = value as u32,
            Err(MemoryError::PageFault(_)) => cpu.set_page_fault(),
            Err(MemoryError::WriteProtection(_)) => cpu.set_write_protect_fault(),
            _ => (),
        }
    }
}

/// Store byte instruction: Memory[rs1 + offset] = rd[7:0]
pub struct StoreByte;

impl Instruction for StoreByte {
    fn execute(&self, cpu: &mut CPU, memory: &mut Memory) {
        let addr = cpu.registers[cpu.s1].wrapping_add(cpu.offset as u32);
        match memory.write_byte(addr, cpu.registers[cpu.d] as u8) {
            Ok(_) => (),
            Err(MemoryError::PageFault(_)) => cpu.set_page_fault(),
            Err(MemoryError::WriteProtection(_)) => cpu.set_write_protect_fault(),
            _ => (),
        }
    }
}

/// Load half-word instruction: rd = SignExtend(Memory[rs1 + offset])
pub struct LoadHalf;

impl Instruction for LoadHalf {
    fn execute(&self, cpu: &mut CPU, memory: &mut Memory) {
        let addr = cpu.registers[cpu.s1].wrapping_add(cpu.offset as u32);
        let mut value = 0u16;

        for i in 0..2 {
            match memory.read_byte(addr + i) {
                Ok(byte) => value = (value << 8) | byte as u16,
                Err(MemoryError::PageFault(_)) => {
                    cpu.set_page_fault();
                    return;
                }
                Err(MemoryError::WriteProtection(_)) => {
                    cpu.set_write_protect_fault();
                    return;
                }
                _ => return,
            }
        }

        cpu.registers[cpu.d] = value as u32;
    }
}

/// Store half-word instruction: Memory[rs1 + offset] = rd[15:0]
pub struct StoreHalf;

impl Instruction for StoreHalf {
    fn execute(&self, cpu: &mut CPU, memory: &mut Memory) {
        let addr = cpu.registers[cpu.s1].wrapping_add(cpu.offset as u32);
        let value = cpu.registers[cpu.d] as u16;

        for i in 0..2 {
            match memory.write_byte(addr + i, ((value >> ((1 - i) * 8)) & 0xFF) as u8) {
                Ok(_) => (),
                Err(MemoryError::PageFault(_)) => {
                    cpu.set_page_fault();
                    return;
                }
                Err(MemoryError::WriteProtection(_)) => {
                    cpu.set_write_protect_fault();
                    return;
                }
                _ => return,
            }
        }
    }
}

/// Load double-word instruction: rd:rd+1 = Memory[rs1 + offset]
pub struct LoadDouble;

impl Instruction for LoadDouble {
    fn execute(&self, cpu: &mut CPU, memory: &mut Memory) {
        let addr = cpu.registers[cpu.s1].wrapping_add(cpu.offset as u32);
        let mut value = 0u64;

        for i in 0..8 {
            match memory.read_byte(addr + i) {
                Ok(byte) => value = (value << 8) | byte as u64,
                Err(MemoryError::PageFault(_)) => {
                    cpu.set_page_fault();
                    return;
                }
                Err(MemoryError::WriteProtection(_)) => {
                    cpu.set_write_protect_fault();
                    return;
                }
                _ => return,
            }
        }

        // Store in consecutive registers
        cpu.registers[cpu.d] = (value >> 32) as u32;
        cpu.registers[cpu.d + 1] = value as u32;
    }
}

/// Store double-word instruction: Memory[rs1 + offset] = rd:rd+1
pub struct StoreDouble;

impl Instruction for StoreDouble {
    fn execute(&self, cpu: &mut CPU, memory: &mut Memory) {
        let addr = cpu.registers[cpu.s1].wrapping_add(cpu.offset as u32);
        let value = ((cpu.registers[cpu.d] as u64) << 32) | (cpu.registers[cpu.d + 1] as u64);

        for i in 0..8 {
            match memory.write_byte(addr + i, ((value >> ((7 - i) * 8)) & 0xFF) as u8) {
                Ok(_) => (),
                Err(MemoryError::PageFault(_)) => {
                    cpu.set_page_fault();
                    return;
                }
                Err(MemoryError::WriteProtection(_)) => {
                    cpu.set_write_protect_fault();
                    return;
                }
                _ => return,
            }
        }
    }
}

/// Exchange instruction: atomically swaps register with memory
pub struct Exchange;

impl Instruction for Exchange {
    fn execute(&self, cpu: &mut CPU, memory: &mut Memory) {
        let addr = cpu.registers[cpu.s1].wrapping_add(cpu.offset as u32);
        let mut old_value = 0u32;

        // Read old value
        for i in 0..4 {
            match memory.read_byte(addr + i) {
                Ok(byte) => old_value = (old_value << 8) | byte as u32,
                Err(MemoryError::PageFault(_)) => {
                    cpu.set_page_fault();
                    return;
                }
                Err(MemoryError::WriteProtection(_)) => {
                    cpu.set_write_protect_fault();
                    return;
                }
                _ => return,
            }
        }

        // Write new value
        let new_value = cpu.registers[cpu.d];
        for i in 0..4 {
            match memory.write_byte(addr + i, ((new_value >> ((3 - i) * 8)) & 0xFF) as u8) {
                Ok(_) => (),
                Err(MemoryError::PageFault(_)) => {
                    cpu.set_page_fault();
                    return;
                }
                Err(MemoryError::WriteProtection(_)) => {
                    cpu.set_write_protect_fault();
                    return;
                }
                _ => return,
            }
        }

        // Store old value
        cpu.registers[cpu.d] = old_value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_store() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();

        // Set up test values
        cpu.registers[1] = 0x1000; // Base address in r1
        cpu.registers[2] = 0xDEADBEEF; // Test value in r2

        // Test store
        let store = Store {
            rd: 2,
            rs1: 1,
            offset: 0x10,
        };
        store.execute(&mut cpu, &mut memory);

        // Test load
        let load = Load {
            rd: 3,
            rs1: 1,
            offset: 0x10,
        };
        load.execute(&mut cpu, &mut memory);

        assert_eq!(cpu.registers[3], 0xDEADBEEF);
    }

    #[test]
    fn test_load_store_with_offset() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();

        // Set up test values
        cpu.registers[1] = 0x1000;
        cpu.registers[2] = 0x12345678;

        // Store at base + offset
        let store = Store {
            rd: 2,
            rs1: 1,
            offset: 0x20,
        };
        store.execute(&mut cpu, &mut memory);

        // Load from base + offset
        let load = Load {
            rd: 3,
            rs1: 1,
            offset: 0x20,
        };
        load.execute(&mut cpu, &mut memory);

        assert_eq!(cpu.registers[3], 0x12345678);
    }

    #[test]
    fn test_load_byte() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();

        let addr = 0x1000;
        let value: u8 = 0x42;

        memory.write_byte(addr, value).unwrap();

        cpu.registers[1] = addr;
        cpu.s1 = 1;
        cpu.d = 2;
        cpu.offset = 0;

        LoadByte.execute(&mut cpu, &mut memory);
        assert_eq!(cpu.registers[2], value as u32);
    }

    #[test]
    fn test_store_byte() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();

        let addr = 0x1000;
        let value: u8 = 0x42;

        cpu.registers[1] = addr;
        cpu.registers[2] = value as u32;
        cpu.s1 = 1;
        cpu.d = 2;
        cpu.offset = 0;

        StoreByte.execute(&mut cpu, &mut memory);
        assert_eq!(memory.read_byte(addr).unwrap(), value);
    }

    #[test]
    fn test_load_half() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();

        let addr = 0x1000;
        let value = 0x1234;

        memory.write_byte(addr, (value >> 8) as u8).unwrap();
        memory.write_byte(addr + 1, value as u8).unwrap();

        cpu.registers[1] = addr;
        cpu.s1 = 1;
        cpu.d = 2;
        cpu.offset = 0;

        LoadHalf.execute(&mut cpu, &mut memory);
        assert_eq!(cpu.registers[2], value as u32);
    }

    #[test]
    fn test_store_half() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();

        let addr = 0x1000;
        let value = 0x1234;

        cpu.registers[1] = addr;
        cpu.registers[2] = value;
        cpu.s1 = 1;
        cpu.d = 2;
        cpu.offset = 0;

        StoreHalf.execute(&mut cpu, &mut memory);

        assert_eq!(memory.read_byte(addr).unwrap(), (value >> 8) as u8);
        assert_eq!(memory.read_byte(addr + 1).unwrap(), value as u8);
    }

    #[test]
    fn test_load_double() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();

        let addr = 0x1000;
        let test_value: u64 = 0x1234567890ABCDEF;

        for i in 0..8 {
            memory
                .write_byte(addr + i, ((test_value >> ((7 - i) * 8)) & 0xFF) as u8)
                .unwrap();
        }

        cpu.registers[1] = addr;
        cpu.s1 = 1;
        cpu.d = 2;
        cpu.offset = 0;

        LoadDouble.execute(&mut cpu, &mut memory);

        let result = ((cpu.registers[2] as u64) << 32) | (cpu.registers[3] as u64);
        assert_eq!(result, test_value);
    }

    #[test]
    fn test_store_double() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();

        let addr = 0x1000;
        let test_value: u64 = 0x1234567890ABCDEF;

        cpu.registers[1] = addr;
        cpu.registers[2] = (test_value >> 32) as u32;
        cpu.registers[3] = test_value as u32;
        cpu.s1 = 1;
        cpu.d = 2;
        cpu.offset = 0;

        StoreDouble.execute(&mut cpu, &mut memory);

        for i in 0..8 {
            let expected = ((test_value >> ((7 - i) * 8)) & 0xFF) as u8;
            assert_eq!(memory.read_byte(addr + i).unwrap(), expected);
        }
    }

    #[test]
    fn test_exchange() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();

        let addr = 0x1000;
        let initial_value = 0x12345678;
        let new_value = 0x90ABCDEF;

        // Set up initial memory value
        for i in 0..4 {
            memory
                .write_byte(addr + i, ((initial_value >> ((3 - i) * 8)) & 0xFF) as u8)
                .unwrap();
        }

        cpu.registers[1] = addr;
        cpu.registers[2] = new_value;
        cpu.s1 = 1;
        cpu.d = 2;
        cpu.offset = 0;

        Exchange.execute(&mut cpu, &mut memory);

        // Check that the old value was stored in the register
        assert_eq!(cpu.registers[2], initial_value);

        // Check that the new value was stored in memory
        let mut mem_value = 0u32;
        for i in 0..4 {
            mem_value = (mem_value << 8) | memory.read_byte(addr + i).unwrap() as u32;
        }
        assert_eq!(mem_value, new_value);
    }

    #[test]
    fn test_load_page_fault() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();
        memory.set_mmu_enabled(true);

        // Try to load from unmapped page
        let load = Load {
            rd: 1,
            rs1: 0,
            offset: 0x1000,
        };
        load.execute(&mut cpu, &mut memory);

        assert!(cpu.cr0 & CPU::CR0_PAGE_FAULT != 0);
    }
}
