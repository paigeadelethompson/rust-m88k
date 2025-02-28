//! System instruction implementations for the Motorola 88000.
//! 
//! This module contains implementations of all system-level operations including:
//! - Cache management
//! - System control operations
//! - Privileged operations
//! - System maintenance functions

use crate::cpu::CPU;
use crate::memory::Memory;
use crate::cpu::instructions::Instruction;

// Cache Control Instructions

/// Instruction cache control instruction
#[derive(Debug)]
pub struct ICache {
    pub operation: CacheOperation,
}

/// Data cache control instruction
#[derive(Debug)]
pub struct DCache {
    pub operation: CacheOperation,
}

/// Cache operation types
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum CacheOperation {
    Invalidate,
    Flush,
    LoadLock,
    StoreLock,
    Prefetch,
    ClearLock,
}

/// Cache flush instruction: flushes both instruction and data caches
#[derive(Debug)]
pub struct FlushCache;

/// Cache invalidate instruction: invalidates cache entries
#[derive(Debug)]
pub struct CacheInvalidate;

/// Cache flush instruction: flushes specific cache entries
#[derive(Debug)]
pub struct CacheFlush;

/// Cache prefetch instruction: prefetches data into cache
#[derive(Debug)]
pub struct CachePrefetch;

/// Privilege level for system operations
#[derive(Debug, PartialEq, Clone, Copy, Default)]
pub enum PrivilegeLevel {
    #[default]
    User = 0,
    Supervisor = 1,
}

impl ICache {
    pub fn new(operation: CacheOperation) -> Self {
        Self { operation }
    }
}

impl DCache {
    pub fn new(operation: CacheOperation) -> Self {
        Self { operation }
    }
}

impl Instruction for ICache {
    fn execute(&self, cpu: &mut CPU, _memory: &mut Memory) {
        // Check privilege level
        if cpu.get_privilege_level() != PrivilegeLevel::Supervisor {
            cpu.set_privilege_violation();
            return;
        }
        // Cache operations are no-ops in this emulator
    }
}

impl Instruction for DCache {
    fn execute(&self, cpu: &mut CPU, _memory: &mut Memory) {
        // Check privilege level
        if cpu.get_privilege_level() != PrivilegeLevel::Supervisor {
            cpu.set_privilege_violation();
            return;
        }
        // Cache operations are no-ops in this emulator
    }
}

impl Instruction for FlushCache {
    fn execute(&self, cpu: &mut CPU, _memory: &mut Memory) {
        // Check privilege level
        if cpu.get_privilege_level() != PrivilegeLevel::Supervisor {
            cpu.set_privilege_violation();
            return;
        }
        // Cache flush operations are no-ops in this emulator
    }
}

impl Instruction for CacheInvalidate {
    fn execute(&self, cpu: &mut CPU, _memory: &mut Memory) {
        // Check privilege level
        if cpu.get_privilege_level() != PrivilegeLevel::Supervisor {
            cpu.set_privilege_violation();
            return;
        }
        // Cache invalidate operations are no-ops in this emulator
    }
}

impl Instruction for CacheFlush {
    fn execute(&self, cpu: &mut CPU, _memory: &mut Memory) {
        // Check privilege level
        if cpu.get_privilege_level() != PrivilegeLevel::Supervisor {
            cpu.set_privilege_violation();
            return;
        }
        // Cache flush operations are no-ops in this emulator
    }
}

impl Instruction for CachePrefetch {
    fn execute(&self, _cpu: &mut CPU, _memory: &mut Memory) {
        // Cache prefetch operations are allowed in user mode
        // but are no-ops in this emulator
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_icache_privilege_levels() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();
        
        // Test in user mode (should fail)
        cpu.set_privilege_level(PrivilegeLevel::User);
        let icache = ICache::new(CacheOperation::Invalidate);
        let initial_state = cpu.cr0;
        icache.execute(&mut cpu, &mut memory);
        assert_ne!(cpu.cr0, initial_state); // Should have privilege violation flag set
        
        // Test in supervisor mode (should succeed)
        cpu.set_privilege_level(PrivilegeLevel::Supervisor);
        cpu.cr0 = initial_state;
        icache.execute(&mut cpu, &mut memory);
        assert_eq!(cpu.cr0, initial_state); // Should not change state
    }

    #[test]
    fn test_dcache_operations() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();
        cpu.set_privilege_level(PrivilegeLevel::Supervisor);
        
        // Test all cache operations
        let operations = vec![
            CacheOperation::Invalidate,
            CacheOperation::Flush,
            CacheOperation::LoadLock,
            CacheOperation::StoreLock,
            CacheOperation::Prefetch,
            CacheOperation::ClearLock,
        ];
        
        for op in operations {
            let dcache = DCache::new(op.clone());
            let initial_state = cpu.cr0;
            dcache.execute(&mut cpu, &mut memory);
            assert_eq!(cpu.cr0, initial_state, "Cache operation {:?} modified CPU state", op);
        }
    }

    #[test]
    fn test_cache_flush_sequence() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();
        cpu.set_privilege_level(PrivilegeLevel::Supervisor);
        
        // Test complete cache flush sequence
        let initial_state = cpu.cr0;
        
        // 1. Invalidate instruction cache
        let icache = ICache::new(CacheOperation::Invalidate);
        icache.execute(&mut cpu, &mut memory);
        
        // 2. Flush data cache
        let dcache = DCache::new(CacheOperation::Flush);
        dcache.execute(&mut cpu, &mut memory);
        
        // 3. Final flush
        FlushCache.execute(&mut cpu, &mut memory);
        
        assert_eq!(cpu.cr0, initial_state, "Cache flush sequence modified CPU state");
    }

    #[test]
    fn test_cache_lock_operations() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();
        cpu.set_privilege_level(PrivilegeLevel::Supervisor);
        
        // Test cache line locking sequence
        let initial_state = cpu.cr0;
        
        // 1. Load lock
        let dcache_load = DCache::new(CacheOperation::LoadLock);
        dcache_load.execute(&mut cpu, &mut memory);
        
        // 2. Store lock
        let dcache_store = DCache::new(CacheOperation::StoreLock);
        dcache_store.execute(&mut cpu, &mut memory);
        
        // 3. Clear lock
        let dcache_clear = DCache::new(CacheOperation::ClearLock);
        dcache_clear.execute(&mut cpu, &mut memory);
        
        assert_eq!(cpu.cr0, initial_state, "Cache lock operations modified CPU state");
    }

    #[test]
    fn test_cache_prefetch_user_mode() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();
        
        // Test prefetch in user mode (should be allowed)
        cpu.set_privilege_level(PrivilegeLevel::User);
        let initial_state = cpu.cr0;
        CachePrefetch.execute(&mut cpu, &mut memory);
        assert_eq!(cpu.cr0, initial_state, "Prefetch in user mode modified CPU state");
    }

    #[test]
    fn test_cache_invalidate_error_handling() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();
        
        // Test invalid privilege level
        cpu.set_privilege_level(PrivilegeLevel::User);
        CacheInvalidate.execute(&mut cpu, &mut memory);
        assert!(cpu.has_privilege_violation(), "Privilege violation not detected");
        
        // Test supervisor mode
        cpu.set_privilege_level(PrivilegeLevel::Supervisor);
        cpu.clear_privilege_violation();
        CacheInvalidate.execute(&mut cpu, &mut memory);
        assert!(!cpu.has_privilege_violation(), "False privilege violation detected");
    }

    #[test]
    fn test_cache_flush_error_handling() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();
        
        // Test invalid privilege level
        cpu.set_privilege_level(PrivilegeLevel::User);
        CacheFlush.execute(&mut cpu, &mut memory);
        assert!(cpu.has_privilege_violation(), "Privilege violation not detected");
        
        // Test supervisor mode
        cpu.set_privilege_level(PrivilegeLevel::Supervisor);
        cpu.clear_privilege_violation();
        CacheFlush.execute(&mut cpu, &mut memory);
        assert!(!cpu.has_privilege_violation(), "False privilege violation detected");
    }

    #[test]
    fn test_cache_operations_state_preservation() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();
        cpu.set_privilege_level(PrivilegeLevel::Supervisor);
        
        // Set up initial CPU state
        cpu.registers[1] = 0xDEADBEEF;
        cpu.registers[2] = 0x12345678;
        let initial_registers = cpu.registers;
        
        // Execute all cache operations
        ICache::new(CacheOperation::Invalidate).execute(&mut cpu, &mut memory);
        DCache::new(CacheOperation::Flush).execute(&mut cpu, &mut memory);
        FlushCache.execute(&mut cpu, &mut memory);
        CacheInvalidate.execute(&mut cpu, &mut memory);
        CacheFlush.execute(&mut cpu, &mut memory);
        CachePrefetch.execute(&mut cpu, &mut memory);
        
        // Verify register state is preserved
        assert_eq!(cpu.registers, initial_registers, "Cache operations modified register state");
    }
} 