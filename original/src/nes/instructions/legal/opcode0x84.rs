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
use crate::nes::instructions::Opcode;

use std::convert::TryInto;

pub struct Opcode0x84 {}

impl Opcode for Opcode0x84 {
    fn get_name() -> String {
        "0x84".to_string()
    }

    fn execute(mut _cpu: &mut Cpu, mut _memory: &mut Memory) {
        // Store immediate value into x register

        // Get the value from memory
        let value: u8 = _memory
            .get_instruction_argument(_cpu.program_counter, 1)
            .try_into()
            .unwrap();

        // Add one to the program counter
        _cpu.register_add(Register::ProgramCounter, 1);

        // Save the value to the x register
        _cpu.y_index = value;
    }
}

#[cfg(test)]

mod tests {
    use super::*;
    use crate::nes::architecture::cpu::tests::get_test_cpu;
    use crate::nes::architecture::memory::tests::get_test_memory;

    #[test]
    fn test_execute() {
        // Prep for the test
        let mut cpu: Cpu = get_test_cpu();
        let mut memory: Memory = get_test_memory(1);
        memory.write(0, [0x55].to_vec());

        // Execute instruction
        Opcode0x84::execute(&mut cpu, &mut memory);

        // Assert results
        assert_eq!(cpu.program_counter, 0x01);
        assert_eq!(cpu.y_index, 0x55);
    }
}
