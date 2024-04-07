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

pub struct Opcode0x85 {}

impl Opcode for Opcode0x85 {
    fn get_name() -> String {
        "0x85".to_string()
    }

    fn execute(mut _cpu: &mut Cpu, mut _memory: &mut Memory) {
        // Store value from zero paged address into the accumulator
        let instruction_arg: u16 = _memory.get_instruction_argument(_cpu.program_counter, 1);

        // Increase the program counter
        _cpu.register_add(Register::ProgramCounter, 1);

        // Get the zero page address
        let address: u16 = Utils::get_zero_paged_address(instruction_arg.try_into().unwrap(), 0);

        // Get the value from memory
        let value: u8 = _memory.read(address.into(), 1)[0];

        // Save the value in the accumulator
        _cpu.accumulator = value;
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
        let mut memory: Memory = get_test_memory(1024);
        cpu.program_counter = 0x01;
        memory.write(0, [0x85, 0x44].to_vec());
        memory.write(0x44, [0xfa].to_vec());

        // Execute instruction
        Opcode0x85::execute(&mut cpu, &mut memory);

        // Assert results
        assert_eq!(cpu.accumulator, 0xfa);
    }
}
