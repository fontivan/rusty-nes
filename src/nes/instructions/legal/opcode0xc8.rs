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
use crate::nes::architecture::cpu::Register;
use crate::nes::architecture::memory::Memory;
use crate::nes::instructions::Opcode;

pub struct Opcode0xc8 {}

impl Opcode for Opcode0xc8 {
    fn get_name() -> String {
        "0xc8".to_string()
    }

    fn execute(mut _cpu: &mut Cpu, mut _memory: &mut Memory) {
        // Increment the y index
        _cpu.register_add(Register::YIndex, 1);

        _cpu.check_result_for_zero_and_negative_flags(_cpu.y_index)
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
    fn test_no_flags() {
        // Prep for the test
        let mut cpu: Cpu = get_test_cpu();
        let mut memory: Memory = get_test_memory();
        cpu.y_index = 0x0F;

        // Execute instruction
        Opcode0xc8::execute(&mut cpu, &mut memory);

        // Assert results
        assert_eq!(cpu.y_index, 0x10);
        assert!(!cpu.is_z_set());
        assert!(!cpu.is_n_set());
        assert!(!cpu.is_v_set());
    }

    #[test]
    fn test_n_flag() {
        // Prep for the test
        let mut cpu: Cpu = get_test_cpu();
        let mut memory: Memory = get_test_memory();
        cpu.y_index = 0xF1;

        // Execute instruction
        Opcode0xc8::execute(&mut cpu, &mut memory);

        // Assert results
        assert_eq!(cpu.y_index, 0xF2);
        assert!(!cpu.is_z_set());
        assert!(cpu.is_n_set());
        assert!(!cpu.is_v_set());
    }

    #[test]
    fn test_z_flag() {
        // Prep for the test
        let mut cpu: Cpu = get_test_cpu();
        let mut memory: Memory = get_test_memory();
        cpu.y_index = 0xFF;

        // Execute instruction
        Opcode0xc8::execute(&mut cpu, &mut memory);

        // Assert results
        assert_eq!(cpu.y_index, 0x00);
        assert!(cpu.is_z_set());
        assert!(!cpu.is_n_set());
        assert!(cpu.is_v_set());
    }
}
