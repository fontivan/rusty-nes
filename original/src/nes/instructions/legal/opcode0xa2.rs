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

pub struct Opcode0xa2 {}

impl Opcode for Opcode0xa2 {
    fn get_name() -> String {
        "0xa2".to_string()
    }

    fn execute(mut _cpu: &mut Cpu, mut _memory: &mut Memory) {
        // Get the operand data from the memory
        let instruction_arg: u8 = _memory
            .get_instruction_argument(_cpu.program_counter, 1)
            .try_into()
            .unwrap();

        _cpu.register_add(Register::ProgramCounter, 1);

        // Load the provided byte directly into x index register
        _cpu.x_index = instruction_arg;

        // If the MSB is high then we will need to set N
        if _cpu.x_index & 0b1000_0000 == 0 {
            _cpu.clear_n_flag();
        } else {
            _cpu.set_n_flag();
        }

        // If the value is zero then set Z
        if _cpu.x_index == 0 {
            _cpu.set_z_flag();
        } else {
            _cpu.clear_z_flag();
        }
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
        memory.write(0, [0xa2, 0x44].to_vec());

        // Execute instruction
        Opcode0xa2::execute(&mut cpu, &mut memory);

        // Assert results
        assert_eq!(cpu.x_index, 0x44);
        assert!(!cpu.is_c_set());
        assert!(!cpu.is_z_set());
        assert!(!cpu.is_n_set());
        assert_eq!(cpu.program_counter, 0x02);
    }
}
