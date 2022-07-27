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

pub struct Opcode0x0e {}

impl Opcode for Opcode0x0e {
    fn get_name() -> String {
        "0x0e".to_string()
    }

    fn execute(mut _cpu: &mut Cpu, mut _memory: &mut Memory) {
        // Arithmetic shift left on an absolute page address

        // Get the address offset
        let address: u16 = _memory.get_instruction_argument(_cpu.program_counter, 2);

        // Increase PC by amount of bytes read
        _cpu.register_add(Register::ProgramCounter, 2);

        // Read the value
        let mut value = _memory.read(address.into(), 1)[0];

        // The carry would be lose by the bitwise shift below so we need to snag it first
        let carry = value & 0b1000_0000 == 0b1000_0000;

        // Shift left
        value <<= 1;

        // Check for one in bit 0
        if carry {
            // Set the carry bit
            _cpu.set_c_flag();
        }

        // Check for 0 or negative flags
        _cpu.check_result_for_zero_and_negative_flags(value);

        // Write the new data back into memory
        _memory.write(address.into(), [value].to_vec());
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
        let memory_size: usize = 18000;
        let memory_result: Result<Memory, usize> = Memory::new(memory_size);
        let mut memory: Memory = memory_result.unwrap();
        return memory;
    }

    #[test]
    fn test_without_carry() {
        // Prep for the test
        let mut cpu: Cpu = get_test_cpu();
        let mut memory: Memory = get_test_memory();
        cpu.program_counter = 0x01;
        memory.write(0, [0x06, 0x00, 0x44].to_vec());
        memory.write(0x4400, [0b0101_1010].to_vec());

        // Execute instruction
        Opcode0x0e::execute(&mut cpu, &mut memory);

        // Assert results
        let result: u8 = memory.read(0x4400, 1)[0];
        assert_eq!(result, 0b1011_0100);
        assert!(!cpu.is_c_set());
        assert!(!cpu.is_z_set());
        assert!(cpu.is_n_set());
    }

    #[test]
    fn test_with_carry() {
        // Prep for the test
        let mut cpu: Cpu = get_test_cpu();
        let mut memory: Memory = get_test_memory();
        cpu.program_counter = 0x01;
        memory.write(0, [0x06, 0x00, 0x44].to_vec());
        memory.write(0x4400, [0b1101_1010].to_vec());

        // Execute instruction
        Opcode0x0e::execute(&mut cpu, &mut memory);

        // Assert results
        let result: u8 = memory.read(0x4400, 1)[0];
        assert_eq!(result, 0b1011_0100);
        assert!(cpu.is_c_set());
        assert!(!cpu.is_z_set());
        assert!(cpu.is_n_set());
    }
}
