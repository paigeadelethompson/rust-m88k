//! Memory management module for the Motorola 88000 emulator.
//! 
//! This module implements memory access and management, including:
//! - Physical memory access
//! - Virtual memory translation
//! - Page table management
//! - Memory protection

/// Memory error types
#[derive(Debug)]
pub enum MemoryError {
    PageFault(u32),
    WriteProtection(u32),
    InvalidAddress(u32),
}

/// Page table entry for virtual memory translation
#[derive(Debug, Clone, Copy)]
pub struct PageTableEntry {
    pub physical_page: u32,
    pub valid: bool,
    pub writable: bool,
    pub supervisor: bool,
}

impl PageTableEntry {
    pub fn new(physical_page: u32) -> Self {
        Self {
            physical_page: physical_page & 0xFFFFF000,
            valid: true,
            writable: true,
            supervisor: false,
        }
    }

    pub fn to_u32(&self) -> u32 {
        let mut value = self.physical_page & 0xFFFFF000;
        if self.valid { value |= 1 << 0; }
        if self.writable { value |= 1 << 1; }
        if self.supervisor { value |= 1 << 2; }
        value
    }

    pub fn from_u32(value: u32) -> Self {
        Self {
            physical_page: value & 0xFFFFF000,
            valid: value & (1 << 0) != 0,
            writable: value & (1 << 1) != 0,
            supervisor: value & (1 << 2) != 0,
        }
    }
}

/// Memory management unit for the Motorola 88000
#[derive(Debug, Default)]
pub struct Memory {
    /// Physical memory array
    memory: Vec<u8>,
    /// MMU enabled flag
    pub(crate) mmu_enabled: bool,
    /// Page table base register
    page_table_base: u32,
}

impl Memory {
    /// Creates a new memory instance with default 16MB of RAM
    pub fn new() -> Self {
        Self {
            memory: vec![0; 16 * 1024 * 1024],
            mmu_enabled: false,
            page_table_base: 0,
        }
    }

    /// Sets the MMU enabled state
    pub fn set_mmu_enabled(&mut self, enabled: bool) {
        self.mmu_enabled = enabled;
    }

    /// Gets the MMU enabled state
    pub fn is_mmu_enabled(&self) -> bool {
        self.mmu_enabled
    }

    /// Sets the page table base register
    pub fn set_page_table_base(&mut self, base: u32) {
        self.page_table_base = base & 0xFFFFF000;
    }

    /// Reads a word from physical memory
    pub fn read_physical_u32(&mut self, addr: u32) -> Result<u32, MemoryError> {
        let addr = addr as usize;
        if addr + 3 >= self.memory.len() {
            return Err(MemoryError::InvalidAddress(addr as u32));
        }
        let b0 = self.memory[addr] as u32;
        let b1 = self.memory[addr + 1] as u32;
        let b2 = self.memory[addr + 2] as u32;
        let b3 = self.memory[addr + 3] as u32;
        Ok((b0 << 24) | (b1 << 16) | (b2 << 8) | b3)
    }

    /// Writes a word to physical memory
    pub fn write_physical_u32(&mut self, addr: u32, value: u32) -> Result<(), MemoryError> {
        let addr = addr as usize;
        if addr + 3 >= self.memory.len() {
            return Err(MemoryError::InvalidAddress(addr as u32));
        }
        self.memory[addr] = ((value >> 24) & 0xFF) as u8;
        self.memory[addr + 1] = ((value >> 16) & 0xFF) as u8;
        self.memory[addr + 2] = ((value >> 8) & 0xFF) as u8;
        self.memory[addr + 3] = (value & 0xFF) as u8;
        Ok(())
    }

    /// Translates a virtual address to a physical address
    pub fn translate_address(&mut self, virtual_addr: u32) -> Result<usize, MemoryError> {
        if !self.mmu_enabled {
            return Ok(virtual_addr as usize);
        }

        let page_index = (virtual_addr >> 12) as usize;
        let pte_addr = self.page_table_base + (page_index as u32 * 4);
        let pte_value = self.read_physical_u32(pte_addr)?;
        let pte = PageTableEntry::from_u32(pte_value);

        if !pte.valid {
            return Err(MemoryError::PageFault(virtual_addr));
        }

        let offset = virtual_addr & 0xFFF;
        Ok((pte.physical_page as usize & 0xFFFFF000) | (offset as usize))
    }

    /// Reads a byte from memory
    pub fn read_byte(&mut self, addr: u32) -> Result<u8, MemoryError> {
        let physical_addr = self.translate_address(addr)?;
        Ok(self.memory[physical_addr])
    }

    /// Writes a byte to memory
    pub fn write_byte(&mut self, addr: u32, value: u8) -> Result<(), MemoryError> {
        let physical_addr = self.translate_address(addr)?;
        self.memory[physical_addr] = value;
        Ok(())
    }

    /// Reads a word (4 bytes) from memory
    pub fn read_word(&mut self, addr: u32) -> Result<u32, MemoryError> {
        let b0 = self.read_byte(addr)? as u32;
        let b1 = self.read_byte(addr + 1)? as u32;
        let b2 = self.read_byte(addr + 2)? as u32;
        let b3 = self.read_byte(addr + 3)? as u32;
        Ok((b0 << 24) | (b1 << 16) | (b2 << 8) | b3)
    }

    /// Writes a word (4 bytes) to memory
    pub fn write_word(&mut self, addr: u32, value: u32) -> Result<(), MemoryError> {
        self.write_byte(addr, ((value >> 24) & 0xFF) as u8)?;
        self.write_byte(addr + 1, ((value >> 16) & 0xFF) as u8)?;
        self.write_byte(addr + 2, ((value >> 8) & 0xFF) as u8)?;
        self.write_byte(addr + 3, (value & 0xFF) as u8)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_initialization() {
        let memory = Memory::new();
        assert_eq!(memory.memory.len(), 16 * 1024 * 1024);
        assert!(!memory.mmu_enabled);
        assert_eq!(memory.page_table_base, 0);
    }

    #[test]
    fn test_memory_access() {
        let mut memory = Memory::new();
        
        // Test byte access
        memory.write_byte(0x1000, 0x42).unwrap();
        assert_eq!(memory.read_byte(0x1000).unwrap(), 0x42);

        // Test word access
        memory.write_word(0x2000, 0xDEADBEEF).unwrap();
        assert_eq!(memory.read_word(0x2000).unwrap(), 0xDEADBEEF);
    }

    #[test]
    fn test_page_table_entry() {
        let pte = PageTableEntry::new(0x1000);
        let value = pte.to_u32();
        let decoded = PageTableEntry::from_u32(value);

        assert_eq!(decoded.physical_page, 0x1000);
        assert_eq!(decoded.valid, true);
        assert_eq!(decoded.writable, true);
        assert_eq!(decoded.supervisor, false);
    }

    #[test]
    fn test_mmu_translation() {
        let mut memory = Memory::new();
        memory.set_mmu_enabled(true);
        memory.set_page_table_base(0x1000);

        // Set up page table entry
        let pte = PageTableEntry::new(0x2000);
        memory.write_physical_u32(0x1000, pte.to_u32()).unwrap();

        // Test translation
        let physical_addr = memory.translate_address(0x0FFF).unwrap();
        assert_eq!(physical_addr, 0x2FFF);

        // Test page fault
        assert!(matches!(
            memory.translate_address(0x2000),
            Err(MemoryError::PageFault(_))
        ));
    }
} 