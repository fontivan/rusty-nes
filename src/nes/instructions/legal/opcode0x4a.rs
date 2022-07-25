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

pub struct Opcode0x4a {}

impl Opcode for Opcode0x4a {
    fn get_name() -> String {
        "0x4a".to_string()
    }

    fn execute(mut _cpu: &mut Cpu, mut _memory: &mut Memory) {
        // Fetch the rightmost bit
        let carry: bool = _cpu.accumulator & 0b0000_0001 == 0b0000_0001;

        // Rotate the bits in the accumlator to the right by 1 bit
        _cpu.accumulator >>= 1;

        // If data is now zero, then set the zero flag high
        if _cpu.accumulator == 0 {
            _cpu.set_z_flag();
        } else {
            _cpu.clear_z_flag();
        }

        // Set carry flag to the value of the rightmost bit
        if carry {
            _cpu.set_c_flag();
        } else {
            _cpu.clear_c_flag();
        }

        // Shift right inserts a 0 into bit 7, so N will always be cleared
        _cpu.clear_n_flag();
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
    fn test_without_carry() {
        // Prep for the test
        let mut cpu: Cpu = get_test_cpu();
        let mut memory: Memory = get_test_memory();

        cpu.accumulator = 0b1100_0110;
        cpu.clear_c_flag();

        // Execute instruction
        Opcode0x4a::execute(&mut cpu, &mut memory);

        // Assert results
        assert_eq!(cpu.accumulator, 0b0110_0011);
        assert!(!cpu.is_c_set());
    }

    #[test]
    fn test_with_carry() {
        // Prep for the test
        let mut cpu: Cpu = get_test_cpu();
        let mut memory: Memory = get_test_memory();

        cpu.accumulator = 0b1100_0011;
        cpu.clear_c_flag();

        // Execute instruction
        Opcode0x4a::execute(&mut cpu, &mut memory);

        // Assert results
        assert_eq!(cpu.accumulator, 0b0110_0001);
        assert!(cpu.is_c_set());
    }

    // #[test]
    // fn test_with_carry_and_rotate_zero() {
    //     // Prep for the test
    //     let mut cpu: Cpu = get_test_cpu();
    //     let mut memory: Memory = get_test_memory();

    //     cpu.accumulator = 0b1101_0101;
    //     cpu.clear_c_flag();

    //     // Execute instruction
    //     Opcode0x4a::execute(&mut cpu, &mut memory);

    //     // Assert results
    //     assert_eq!(cpu.accumulator, 0b0110_1010);
    //     assert!(cpu.is_c_set());
    // }

    // #[test]
    // fn test_with_carry_and_rotate_one() {
    //     // Prep for the test
    //     let mut cpu: Cpu = get_test_cpu();
    //     let mut memory: Memory = get_test_memory();

    //     cpu.accumulator = 0b1101_0101;
    //     cpu.set_c_flag();

    //     // Execute instruction
    //     Opcode0x4a::execute(&mut cpu, &mut memory);

    //     // Assert results
    //     assert_eq!(cpu.accumulator, 0b1110_1010);
    //     assert!(cpu.is_c_set());
    // }
}
