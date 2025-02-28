use crate::cpu::instructions::Instruction;
use crate::cpu::CPU;
use crate::memory::Memory;

// Vector add instruction
pub struct VAdd;

// Vector subtract instruction
pub struct VSub;

// Vector multiply instruction
pub struct VMul;

// Vector divide instruction
pub struct VDiv;

// Vector move instruction
pub struct VMove;

// Vector Equal instruction
pub struct VEq;

// Vector Greater Than instruction
pub struct VGt;

// Vector Less Than instruction
pub struct VLt;

// Vector Maximum instruction
pub struct VMax;

// Vector Minimum instruction
pub struct VMin;

// Vector Shuffle instruction
pub struct VShuffle;

// Vector Interleave High instruction
pub struct VInterleaveHigh;

// Vector Interleave Low instruction
pub struct VInterleaveLow;

// Vector Extract Byte instruction
pub struct VExtractByte;

// Vector Insert Byte instruction
pub struct VInsertByte;

// Vector Pack Bytes to Halfwords instruction
pub struct VPackBytesToHalfwords;

// Vector Pack Halfwords to Word instruction
pub struct VPackHalfwordsToWord;

// Vector Unpack Bytes to Halfwords instruction
pub struct VUnpackBytesToHalfwords;

// Vector Unpack Halfwords to Word instruction
pub struct VUnpackHalfwordsToWord;

const VECTOR_SIZE: usize = 4;

impl Instruction for VAdd {
    fn execute(&self, cpu: &mut CPU, _memory: &mut Memory) {
        for i in 0..VECTOR_SIZE {
            let base_s1 = cpu.s1 + i;
            let base_s2 = cpu.s2 + i;
            let base_d = cpu.d + i;

            let a = f32::from_bits(cpu.registers[base_s1]);
            let b = f32::from_bits(cpu.registers[base_s2]);
            let result = a + b;

            if result.is_nan() {
                cpu.cr0 |= CPU::CR0_FP_INVALID;
            }

            cpu.registers[base_d] = result.to_bits();
        }
    }
}

impl Instruction for VSub {
    fn execute(&self, cpu: &mut CPU, _memory: &mut Memory) {
        for i in 0..VECTOR_SIZE {
            let base_s1 = cpu.s1 + i;
            let base_s2 = cpu.s2 + i;
            let base_d = cpu.d + i;

            let a = f32::from_bits(cpu.registers[base_s1]);
            let b = f32::from_bits(cpu.registers[base_s2]);
            let result = a - b;

            if result.is_nan() {
                cpu.cr0 |= CPU::CR0_FP_INVALID;
            }

            cpu.registers[base_d] = result.to_bits();
        }
    }
}

impl Instruction for VMul {
    fn execute(&self, cpu: &mut CPU, _memory: &mut Memory) {
        for i in 0..VECTOR_SIZE {
            let base_s1 = cpu.s1 + i;
            let base_s2 = cpu.s2 + i;
            let base_d = cpu.d + i;

            let a = f32::from_bits(cpu.registers[base_s1]);
            let b = f32::from_bits(cpu.registers[base_s2]);
            let result = a * b;

            if result.is_nan() {
                cpu.cr0 |= CPU::CR0_FP_INVALID;
            }

            cpu.registers[base_d] = result.to_bits();
        }
    }
}

impl Instruction for VDiv {
    fn execute(&self, cpu: &mut CPU, _memory: &mut Memory) {
        for i in 0..VECTOR_SIZE {
            let base_s1 = cpu.s1 + i;
            let base_s2 = cpu.s2 + i;
            let base_d = cpu.d + i;

            let a = f32::from_bits(cpu.registers[base_s1]);
            let b = f32::from_bits(cpu.registers[base_s2]);

            if b == 0.0 {
                cpu.cr0 |= CPU::CR0_FP_DIVZERO;
                cpu.registers[base_d] = f32::NAN.to_bits();
                continue;
            }

            let result = a / b;
            if result.is_nan() {
                cpu.cr0 |= CPU::CR0_FP_INVALID;
            }

            cpu.registers[base_d] = result.to_bits();
        }
    }
}

impl Instruction for VMove {
    fn execute(&self, cpu: &mut CPU, _memory: &mut Memory) {
        for i in 0..VECTOR_SIZE {
            let base_s1 = cpu.s1 + i;
            let base_d = cpu.d + i;
            cpu.registers[base_d] = cpu.registers[base_s1];
        }
    }
}

impl Instruction for VEq {
    fn execute(&self, cpu: &mut CPU, _memory: &mut Memory) {
        let a = cpu.registers[cpu.s1];
        let b = cpu.registers[cpu.s2];

        // Compare each byte and set result bits
        let mut result = 0u32;
        for i in 0..4 {
            let shift = (3 - i) * 8;
            let byte_a = (a >> shift) & 0xFF;
            let byte_b = (b >> shift) & 0xFF;
            if byte_a == byte_b {
                result |= 0xFF << shift;
            }
        }
        cpu.registers[cpu.d] = result;
    }
}

impl Instruction for VGt {
    fn execute(&self, cpu: &mut CPU, _memory: &mut Memory) {
        let a = cpu.registers[cpu.s1];
        let b = cpu.registers[cpu.s2];

        // Compare each byte and set result bits
        let mut result = 0u32;
        for i in 0..4 {
            let shift = (3 - i) * 8;
            let byte_a = (a >> shift) & 0xFF;
            let byte_b = (b >> shift) & 0xFF;
            if byte_a > byte_b {
                result |= 0xFF << shift;
            }
        }
        cpu.registers[cpu.d] = result;
    }
}

impl Instruction for VLt {
    fn execute(&self, cpu: &mut CPU, _memory: &mut Memory) {
        let a = cpu.registers[cpu.s1];
        let b = cpu.registers[cpu.s2];

        // Compare each byte and set result bits
        let mut result = 0u32;
        for i in 0..4 {
            let shift = (3 - i) * 8;
            let byte_a = (a >> shift) & 0xFF;
            let byte_b = (b >> shift) & 0xFF;
            if byte_a < byte_b {
                result |= 0xFF << shift;
            }
        }
        cpu.registers[cpu.d] = result;
    }
}

impl Instruction for VMax {
    fn execute(&self, cpu: &mut CPU, _memory: &mut Memory) {
        let a = cpu.registers[cpu.s1];
        let b = cpu.registers[cpu.s2];

        // Find maximum of each byte
        let mut result = 0u32;
        for i in 0..4 {
            let byte_a = (a >> (24 - i * 8)) & 0xFF;
            let byte_b = (b >> (24 - i * 8)) & 0xFF;
            let max = if byte_a > byte_b { byte_a } else { byte_b };
            result |= max << (24 - i * 8);
        }
        cpu.registers[cpu.d] = result;
    }
}

impl Instruction for VMin {
    fn execute(&self, cpu: &mut CPU, _memory: &mut Memory) {
        let a = cpu.registers[cpu.s1];
        let b = cpu.registers[cpu.s2];

        // Find minimum of each byte
        let mut result = 0u32;
        for i in 0..4 {
            let byte_a = (a >> (24 - i * 8)) & 0xFF;
            let byte_b = (b >> (24 - i * 8)) & 0xFF;
            let min = if byte_a < byte_b { byte_a } else { byte_b };
            result |= min << (24 - i * 8);
        }
        cpu.registers[cpu.d] = result;
    }
}

impl Instruction for VShuffle {
    fn execute(&self, cpu: &mut CPU, _memory: &mut Memory) {
        let a = cpu.registers[cpu.s1];
        let pattern = cpu.registers[cpu.s2];

        let mut result = 0u32;
        for i in 0..4 {
            let shift = (3 - i) * 8;
            let pattern_shift = (3 - i) * 2;
            let byte_select = (pattern >> pattern_shift) & 0x3;
            let byte = (a >> (byte_select * 8)) & 0xFF;
            result |= byte << shift;
        }
        cpu.registers[cpu.d] = result;
    }
}

impl Instruction for VInterleaveHigh {
    fn execute(&self, cpu: &mut CPU, _memory: &mut Memory) {
        let a = cpu.registers[cpu.s1];
        let b = cpu.registers[cpu.s2];

        // Interleave high bytes: a[0],b[0],a[1],b[1]
        let result = (a & 0xFF000000)
            | ((b & 0xFF000000) >> 8)
            | ((a & 0x00FF0000) >> 8)
            | ((b & 0x00FF0000) >> 16);

        cpu.registers[cpu.d] = result;
    }
}

impl Instruction for VInterleaveLow {
    fn execute(&self, cpu: &mut CPU, _memory: &mut Memory) {
        let a = cpu.registers[cpu.s1];
        let b = cpu.registers[cpu.s2];

        // Interleave low bytes: a[2],b[2],a[3],b[3]
        let result = ((a & 0x0000FF00) << 16)
            | ((b & 0x0000FF00) << 8)
            | ((a & 0x000000FF) << 8)
            | (b & 0x000000FF);

        cpu.registers[cpu.d] = result;
    }
}

impl Instruction for VExtractByte {
    fn execute(&self, cpu: &mut CPU, _memory: &mut Memory) {
        let a = cpu.registers[cpu.s1];
        let pos = cpu.registers[cpu.s2] & 0x3; // Only use bottom 2 bits for position
        let byte = (a >> ((3 - pos) * 8)) & 0xFF;
        cpu.registers[cpu.d] = byte;
    }
}

impl Instruction for VInsertByte {
    fn execute(&self, cpu: &mut CPU, _memory: &mut Memory) {
        let a = cpu.registers[cpu.s1]; // Vector
        let b = cpu.registers[cpu.s2]; // Byte to insert
        let pos = cpu.imm as u32 & 0x3; // Position from immediate value
        let shift = (3 - pos) * 8;
        let mask = !(0xFF << shift);
        let result = (a & mask) | ((b & 0xFF) << shift);
        cpu.registers[cpu.d] = result;
    }
}

impl Instruction for VPackBytesToHalfwords {
    fn execute(&self, cpu: &mut CPU, _memory: &mut Memory) {
        let a = cpu.registers[cpu.s1];
        let b = cpu.registers[cpu.s2];

        // Pack bytes into halfwords with saturation
        let mut result = 0u32;
        for i in 0..2 {
            let byte_a = ((a >> ((3 - i) * 8)) & 0xFF) as u16;
            let byte_b = ((b >> ((3 - i) * 8)) & 0xFF) as u16;
            let halfword = (byte_a << 8) | byte_b;
            result |= (halfword as u32) << ((1 - i) * 16);
        }
        cpu.registers[cpu.d] = result;
    }
}

impl Instruction for VPackHalfwordsToWord {
    fn execute(&self, cpu: &mut CPU, _memory: &mut Memory) {
        let a = cpu.registers[cpu.s1];
        let b = cpu.registers[cpu.s2];

        // Pack halfwords into word with saturation
        let high = a & 0xFFFF;
        let low = b & 0xFFFF;
        let result = (high << 16) | low;
        cpu.registers[cpu.d] = result;
    }
}

impl Instruction for VUnpackBytesToHalfwords {
    fn execute(&self, cpu: &mut CPU, _memory: &mut Memory) {
        let a = cpu.registers[cpu.s1];

        // Unpack bytes to halfwords
        let mut result = 0u32;
        for i in 0..2 {
            let byte = (a >> ((3 - i) * 8)) & 0xFF;
            let halfword = byte << ((1 - i) * 16);
            result |= halfword;
        }
        cpu.registers[cpu.d] = result;
    }
}

impl Instruction for VUnpackHalfwordsToWord {
    fn execute(&self, cpu: &mut CPU, _memory: &mut Memory) {
        let a = cpu.registers[cpu.s1];

        // Unpack halfwords to word
        let high = (a >> 16) & 0xFFFF;
        let low = a & 0xFFFF;
        cpu.registers[cpu.d] = high;
        cpu.registers[cpu.d + 1] = low;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vadd() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();

        cpu.s1 = 1;
        cpu.s2 = 5;
        cpu.d = 9;

        // Initialize source vectors
        for i in 0..VECTOR_SIZE {
            cpu.registers[1 + i] = (i as f32).to_bits();
            cpu.registers[5 + i] = ((i + 1) as f32).to_bits();
        }

        VAdd.execute(&mut cpu, &mut memory);

        // Check results
        for i in 0..VECTOR_SIZE {
            let result = f32::from_bits(cpu.registers[9 + i]);
            assert_eq!(result, (2 * i + 1) as f32);
        }
    }

    #[test]
    fn test_vsub() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();

        cpu.s1 = 1;
        cpu.s2 = 5;
        cpu.d = 9;

        // Initialize source vectors
        for i in 0..VECTOR_SIZE {
            cpu.registers[1 + i] = (i as f32).to_bits();
            cpu.registers[5 + i] = ((i + 1) as f32).to_bits();
        }

        VSub.execute(&mut cpu, &mut memory);

        // Check results
        for i in 0..VECTOR_SIZE {
            let result = f32::from_bits(cpu.registers[9 + i]);
            assert_eq!(result, -1.0);
        }
    }

    #[test]
    fn test_vmul() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();

        cpu.s1 = 1;
        cpu.s2 = 5;
        cpu.d = 9;

        // Initialize source vectors
        for i in 0..VECTOR_SIZE {
            cpu.registers[1 + i] = (2.0f32).to_bits();
            cpu.registers[5 + i] = ((i + 1) as f32).to_bits();
        }

        VMul.execute(&mut cpu, &mut memory);

        // Check results
        for i in 0..VECTOR_SIZE {
            let result = f32::from_bits(cpu.registers[9 + i]);
            assert_eq!(result, 2.0 * (i + 1) as f32);
        }
    }

    #[test]
    fn test_vdiv() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();

        cpu.s1 = 1;
        cpu.s2 = 5;
        cpu.d = 9;

        // Initialize source vectors
        for i in 0..VECTOR_SIZE {
            cpu.registers[1 + i] = ((i + 1) as f32 * 2.0).to_bits();
            cpu.registers[5 + i] = (2.0f32).to_bits();
        }

        VDiv.execute(&mut cpu, &mut memory);

        // Check results
        for i in 0..VECTOR_SIZE {
            let result = f32::from_bits(cpu.registers[9 + i]);
            assert_eq!(result, (i + 1) as f32);
        }

        // Test division by zero
        cpu.registers[5] = (0.0f32).to_bits();
        VDiv.execute(&mut cpu, &mut memory);
        assert_ne!(cpu.cr0 & CPU::CR0_FP_DIVZERO, 0);
        assert!(f32::from_bits(cpu.registers[9]).is_nan());
    }

    #[test]
    fn test_vmove() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();

        cpu.s1 = 1;
        cpu.d = 5;

        // Initialize source vector
        for i in 0..VECTOR_SIZE {
            cpu.registers[1 + i] = (i as f32).to_bits();
        }

        VMove.execute(&mut cpu, &mut memory);

        // Check results
        for i in 0..VECTOR_SIZE {
            assert_eq!(cpu.registers[5 + i], cpu.registers[1 + i]);
        }
    }

    #[test]
    fn test_veq() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();

        // Test with 0x12345678 vs 0x12FF5678
        // Bytes: [0x12, 0x34, 0x56, 0x78] vs [0x12, 0xFF, 0x56, 0x78]
        // Match:   yes   no    yes   yes
        cpu.registers[1] = 0x12345678;
        cpu.registers[2] = 0x12FF5678;
        cpu.d = 3;
        cpu.s1 = 1;
        cpu.s2 = 2;

        VEq.execute(&mut cpu, &mut memory);
        assert_eq!(cpu.registers[3], 0xFF00FFFF);
    }

    #[test]
    fn test_vgt() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();

        // Test with 0x12345678 vs 0x11335577
        // Bytes: [0x12, 0x34, 0x56, 0x78] vs [0x11, 0x33, 0x55, 0x77]
        // GT:      yes   yes   yes   yes
        cpu.registers[1] = 0x12345678;
        cpu.registers[2] = 0x11335577;
        cpu.d = 3;
        cpu.s1 = 1;
        cpu.s2 = 2;

        VGt.execute(&mut cpu, &mut memory);
        assert_eq!(cpu.registers[3], 0xFFFFFFFF);
    }

    #[test]
    fn test_vlt() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();

        // Test with 0x11335577 vs 0x12345678
        // Bytes: [0x11, 0x33, 0x55, 0x77] vs [0x12, 0x34, 0x56, 0x78]
        // LT:      yes   yes   yes   yes
        cpu.registers[1] = 0x11335577;
        cpu.registers[2] = 0x12345678;
        cpu.d = 3;
        cpu.s1 = 1;
        cpu.s2 = 2;

        VLt.execute(&mut cpu, &mut memory);
        assert_eq!(cpu.registers[3], 0xFFFFFFFF);
    }

    #[test]
    fn test_vmax() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();

        cpu.registers[1] = 0x12345678;
        cpu.registers[2] = 0x11335577;
        cpu.d = 3;
        cpu.s1 = 1;
        cpu.s2 = 2;

        VMax.execute(&mut cpu, &mut memory);
        assert_eq!(cpu.registers[3], 0x12345678);
    }

    #[test]
    fn test_vmin() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();

        cpu.registers[1] = 0x12345678;
        cpu.registers[2] = 0x11335577;
        cpu.d = 3;
        cpu.s1 = 1;
        cpu.s2 = 2;

        VMin.execute(&mut cpu, &mut memory);
        assert_eq!(cpu.registers[3], 0x11335577);
    }

    #[test]
    fn test_vshuffle() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();

        // Test data: 0x12345678
        // Pattern: 0b00011011 (3,2,1,0)
        cpu.registers[1] = 0x12345678;
        cpu.registers[2] = 0b00011011;
        cpu.d = 3;
        cpu.s1 = 1;
        cpu.s2 = 2;

        VShuffle.execute(&mut cpu, &mut memory);
        assert_eq!(cpu.registers[3], 0x78563412);
    }

    #[test]
    fn test_vinterleave_high() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();

        cpu.registers[1] = 0x12345678;
        cpu.registers[2] = 0xABCDEFFF;
        cpu.d = 3;
        cpu.s1 = 1;
        cpu.s2 = 2;

        VInterleaveHigh.execute(&mut cpu, &mut memory);
        assert_eq!(cpu.registers[3], 0x12AB34CD);
    }

    #[test]
    fn test_vinterleave_low() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();

        cpu.registers[1] = 0x12345678;
        cpu.registers[2] = 0xABCDEFFF;
        cpu.d = 3;
        cpu.s1 = 1;
        cpu.s2 = 2;

        VInterleaveLow.execute(&mut cpu, &mut memory);
        assert_eq!(cpu.registers[3], 0x56EF78FF);
    }

    #[test]
    fn test_vextract_byte() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();

        cpu.registers[1] = 0x12345678;
        cpu.registers[2] = 0; // Extract first byte
        cpu.d = 3;
        cpu.s1 = 1;
        cpu.s2 = 2;

        VExtractByte.execute(&mut cpu, &mut memory);
        assert_eq!(cpu.registers[3], 0x12);

        cpu.registers[2] = 3; // Extract last byte
        VExtractByte.execute(&mut cpu, &mut memory);
        assert_eq!(cpu.registers[3], 0x78);
    }

    #[test]
    fn test_vinsert_byte() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();

        cpu.registers[1] = 0x12345678; // Original vector
        cpu.registers[2] = 0xFF; // Byte to insert
        cpu.imm = 0; // Insert at first position
        cpu.d = 3;
        cpu.s1 = 1;
        cpu.s2 = 2;

        VInsertByte.execute(&mut cpu, &mut memory);
        assert_eq!(cpu.registers[3], 0xFF345678);

        cpu.registers[1] = 0x12345678;
        cpu.imm = 3; // Insert at last position
        VInsertByte.execute(&mut cpu, &mut memory);
        assert_eq!(cpu.registers[3], 0x123456FF);
    }

    #[test]
    fn test_vpack_bytes_to_halfwords() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();

        cpu.registers[1] = 0x12345678; // First vector
        cpu.registers[2] = 0xABCDEFFF; // Second vector
        cpu.d = 3;
        cpu.s1 = 1;
        cpu.s2 = 2;

        VPackBytesToHalfwords.execute(&mut cpu, &mut memory);
        assert_eq!(cpu.registers[3], 0x12AB34CD);
    }

    #[test]
    fn test_vpack_halfwords_to_word() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();

        cpu.registers[1] = 0x00001234; // First halfword
        cpu.registers[2] = 0x00005678; // Second halfword
        cpu.d = 3;
        cpu.s1 = 1;
        cpu.s2 = 2;

        VPackHalfwordsToWord.execute(&mut cpu, &mut memory);
        assert_eq!(cpu.registers[3], 0x12345678);
    }

    #[test]
    fn test_vunpack_bytes_to_halfwords() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();

        cpu.registers[1] = 0x12345678;
        cpu.d = 2;
        cpu.s1 = 1;

        VUnpackBytesToHalfwords.execute(&mut cpu, &mut memory);
        assert_eq!(cpu.registers[2], 0x00120034);
    }

    #[test]
    fn test_vunpack_halfwords_to_word() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();

        cpu.registers[1] = 0x12345678;
        cpu.d = 2;
        cpu.s1 = 1;

        VUnpackHalfwordsToWord.execute(&mut cpu, &mut memory);
        assert_eq!(cpu.registers[2], 0x00001234);
        assert_eq!(cpu.registers[3], 0x00005678);
    }
}
