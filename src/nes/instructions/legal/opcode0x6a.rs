////////////////////////////////////////////////////////////////////////////////////////////////////
// MIT License
//
// Copyright (c) 2021 fontivan
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
////////////////////////////////////////////////////////////////////////////////////////////////////

use crate::nes::architecture::cpu::Cpu;
use crate::nes::architecture::memory::Memory;
use crate::nes::instructions::Opcode;

pub struct Opcode0x6a {}

impl Opcode for Opcode0x6a {
    fn get_name() -> String {
        "0x6a".to_string()
    }

    fn execute(mut _cpu: &mut Cpu, mut _memory: &mut Memory) {
        // Rotate right on the accumulator register

        // Get the value from the accumulator
        let mut value = _cpu.accumulator;

        // Get the high bit from the value
        let low_bit = value & 0b0000_0001;

        // Shift the value
        value >>= 1;

        // Set the highest bit to be the value from the carry flag
        if _cpu.is_c_set() {
            value |= 0b1000_0000;
        }

        // Save the value back to the accumulator
        _cpu.accumulator = value;

        // Save the low bit into the carry
        if low_bit != 0 {
            _cpu.set_c_flag();
        } else {
            _cpu.clear_c_flag();
        }

        _cpu.check_result_for_zero_and_negative_flags(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_cpu() -> Cpu {
        // Get a cpu
        let mut cpu: Cpu = Cpu::new();
        cpu.program_counter = 0x00;
        return cpu;
    }

    fn get_test_memory() -> Memory {
        // Get a memory
        let memory_size: usize = 1;
        let memory_result: Result<Memory, usize> = Memory::new(memory_size);
        let mut memory: Memory = memory_result.unwrap();
        return memory;
    }

    #[test]
    fn test_without_carry_and_without_rotate() {
        // Prep for the test
        let mut cpu: Cpu = get_test_cpu();
        let mut memory: Memory = get_test_memory();

        cpu.accumulator = 0b1000_0100;
        cpu.clear_c_flag();

        // Execute instruction
        Opcode0x6a::execute(&mut cpu, &mut memory);

        // Assert results
        assert_eq!(cpu.accumulator, 0b0100_0010);
        assert!(!cpu.is_c_set());
    }

    #[test]
    fn test_without_carry_and_with_rotate() {
        // Prep for the test
        let mut cpu: Cpu = get_test_cpu();
        let mut memory: Memory = get_test_memory();

        cpu.accumulator = 0b1000_0100;
        cpu.set_c_flag();

        // Execute instruction
        Opcode0x6a::execute(&mut cpu, &mut memory);

        // Assert results
        assert_eq!(cpu.accumulator, 0b1100_0010);
        assert!(!cpu.is_c_set());
    }

    #[test]
    fn test_with_carry_and_without_rotate() {
        // Prep for the test
        let mut cpu: Cpu = get_test_cpu();
        let mut memory: Memory = get_test_memory();

        cpu.accumulator = 0b1000_0101;
        cpu.clear_c_flag();

        // Execute instruction
        Opcode0x6a::execute(&mut cpu, &mut memory);

        // Assert results
        assert_eq!(cpu.accumulator, 0b0100_0010);
        assert!(cpu.is_c_set());
    }

    #[test]
    fn test_with_carry_and_with_rotate() {
        // Prep for the test
        let mut cpu: Cpu = get_test_cpu();
        let mut memory: Memory = get_test_memory();

        cpu.accumulator = 0b1000_0101;
        cpu.set_c_flag();

        // Execute instruction
        Opcode0x6a::execute(&mut cpu, &mut memory);

        // Assert results
        assert_eq!(cpu.accumulator, 0b1100_0010);
        assert!(cpu.is_c_set());
    }
}
