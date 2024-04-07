////////////////////////////////////////////////////////////////////////////////////////////////////
// MIT License
//
// Copyright (c) 2021-2024 fontivan
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
use crate::nes::architecture::utils::Utils;
use crate::nes::instructions::Opcode;

use std::convert::TryInto;

pub struct Opcode0x24 {}

impl Opcode for Opcode0x24 {
    fn get_name() -> String {
        "0x24".to_string()
    }

    fn execute(mut _cpu: &mut Cpu, mut _memory: &mut Memory) {
        // Perform some bit tests and set appropriate flags using a zero paged address

        // Store value from zero paged address into the accumulator
        let instruction_arg: u16 = _memory.get_instruction_argument(_cpu.program_counter, 1);

        // Increase the program counter
        _cpu.register_add(Register::ProgramCounter, 1);

        // Get the zero page address
        let address: u16 = Utils::get_zero_paged_address(instruction_arg.try_into().unwrap(), 0);

        // Get the value from memory
        let value: u8 = _memory.read(address.into(), 1)[0];

        // Test #1 - If the result of the value AND'd with the accumulator is zero then set z
        if _cpu.accumulator & value == 0 {
            _cpu.set_z_flag();
        } else {
            _cpu.clear_z_flag();
        }

        // Test #2 - If bit 7 of the original value from memory was 1 then set n
        if value & 0b1000_0000 == 0b1000_0000 {
            _cpu.set_n_flag();
        } else {
            _cpu.clear_n_flag();
        }

        // Test #3 - If bit 6 of the original value from memory was 1 then set v
        if value & 0b0100_0000 == 0b0100_0000 {
            _cpu.set_v_flag();
        } else {
            _cpu.clear_v_flag();
        }
    }
}

#[cfg(test)]

mod tests {
    use super::*;
    use crate::nes::architecture::cpu::tests::get_test_cpu;
    use crate::nes::architecture::memory::tests::get_test_memory;

    #[test]
    fn test_execute_with_nv() {
        // Prep for the test
        let mut cpu: Cpu = get_test_cpu();
        let mut memory: Memory = get_test_memory(1024);
        cpu.program_counter = 0x01;
        cpu.accumulator = 0xf1;
        memory.write(0, [0x24, 0x44].to_vec());
        memory.write(0x44, [0xfa].to_vec());

        // Execute instruction
        Opcode0x24::execute(&mut cpu, &mut memory);

        // Assert results
        assert!(cpu.is_n_set());
        assert!(cpu.is_v_set());
        assert!(!cpu.is_z_set());
    }

    #[test]
    fn test_execute_with_no_flags() {
        // Prep for the test
        let mut cpu: Cpu = get_test_cpu();
        let mut memory: Memory = get_test_memory(1024);
        cpu.program_counter = 0x01;
        cpu.accumulator = 0xf1;
        memory.write(0, [0x24, 0x44].to_vec());
        memory.write(0x44, [0x0f].to_vec());

        // Execute instruction
        Opcode0x24::execute(&mut cpu, &mut memory);

        // Assert results
        assert!(!cpu.is_n_set());
        assert!(!cpu.is_v_set());
        assert!(!cpu.is_z_set());
    }

    #[test]
    fn test_execute_with_nvz() {
        // Prep for the test
        let mut cpu: Cpu = get_test_cpu();
        let mut memory: Memory = get_test_memory(1024);
        cpu.program_counter = 0x01;
        cpu.accumulator = 0x00;
        memory.write(0, [0x24, 0x44].to_vec());
        memory.write(0x44, [0xff].to_vec());

        // Execute instruction
        Opcode0x24::execute(&mut cpu, &mut memory);

        // Assert results
        assert!(cpu.is_n_set());
        assert!(cpu.is_v_set());
        assert!(cpu.is_z_set());
    }

    #[test]
    fn test_execute_with_n() {
        // Prep for the test
        let mut cpu: Cpu = get_test_cpu();
        let mut memory: Memory = get_test_memory(1024);
        cpu.program_counter = 0x01;
        cpu.accumulator = 0xaa;
        memory.write(0, [0x24, 0x44].to_vec());
        memory.write(0x44, [0b1000_0000].to_vec());

        // Execute instruction
        Opcode0x24::execute(&mut cpu, &mut memory);

        // Assert results
        assert!(cpu.is_n_set());
        assert!(!cpu.is_v_set());
        assert!(!cpu.is_z_set());
    }

    #[test]
    fn test_execute_with_v() {
        // Prep for the test
        let mut cpu: Cpu = get_test_cpu();
        let mut memory: Memory = get_test_memory(1024);
        cpu.program_counter = 0x01;
        cpu.accumulator = 0xff;
        memory.write(0, [0x24, 0x44].to_vec());
        memory.write(0x44, [0b0100_0000].to_vec());

        // Execute instruction
        Opcode0x24::execute(&mut cpu, &mut memory);

        // Assert results
        assert!(!cpu.is_n_set());
        assert!(cpu.is_v_set());
        assert!(!cpu.is_z_set());
    }
}
