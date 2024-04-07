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

pub struct Opcode0x30 {}

impl Opcode for Opcode0x30 {
    fn get_name() -> String {
        "0x30".to_string()
    }

    fn execute(mut _cpu: &mut Cpu, mut _memory: &mut Memory) {
        // Branch on minus (negative set)

        // If n flag is not set then just add to program counter
        if !_cpu.is_n_set() {
            _cpu.register_add(Register::ProgramCounter, 1);
            return;
        }

        _cpu.register_add(
            Register::ProgramCounter,
            _memory.get_branch_relative_jump(_cpu.program_counter),
        );
    }
}

#[cfg(test)]

mod tests {
    use super::*;
    use crate::nes::architecture::cpu::tests::get_test_cpu;
    use crate::nes::architecture::memory::tests::get_test_memory;

    #[test]
    fn test_taking_positive_branch() {
        // Prep for the test
        let mut cpu: Cpu = get_test_cpu();
        let mut memory: Memory = get_test_memory(1024);
        cpu.set_n_flag();
        cpu.program_counter = 0x01;
        memory.write(0, [0xb0, 0x05].to_vec());

        // Execute instruction
        Opcode0x30::execute(&mut cpu, &mut memory);

        // Assert results
        assert_eq!(cpu.program_counter, 0x06);
    }

    #[test]
    fn test_taking_negative_branch() {
        // Prep for the test
        let mut cpu: Cpu = get_test_cpu();
        let mut memory: Memory = get_test_memory(1024);
        cpu.program_counter = 0xf1;
        cpu.set_n_flag();
        memory.write(0xf0, [0xb0, 0xf5].to_vec());

        // Execute instruction
        Opcode0x30::execute(&mut cpu, &mut memory);

        // Assert results
        assert_eq!(cpu.program_counter, 0xe6);
    }

    #[test]
    fn test_not_taking_branch() {
        // Prep for the test
        let mut cpu: Cpu = get_test_cpu();
        let mut memory: Memory = get_test_memory(1024);
        cpu.clear_n_flag();
        cpu.program_counter = 0xf0;
        memory.write(0xf0, [0xb0, 0xf5].to_vec());

        // Execute instruction
        Opcode0x30::execute(&mut cpu, &mut memory);

        // Assert results
        assert_eq!(cpu.program_counter, 0xf1);
    }
}
