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
use crate::nes::architecture::memory::Memory;
use crate::nes::architecture::utils::Utils;
use crate::nes::instructions::Opcode;

pub struct Opcode0x60 {}

impl Opcode for Opcode0x60 {
    fn get_name() -> String {
        "0x60".to_string()
    }

    fn execute(mut _cpu: &mut Cpu, mut _memory: &mut Memory) {
        // Return from subroutine

        let low_byte: u8 = Utils::stack_pop(_cpu, _memory);
        let high_byte: u8 = Utils::stack_pop(_cpu, _memory);
        let return_address: u16 = Utils::get_u16_from_u8_pair(high_byte, low_byte);

        // The instruction states to return execution to the specified address plus one
        _cpu.program_counter = return_address + 1;
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
        let mut memory: Memory = get_test_memory(18000);
        memory.write(0, [0x60].to_vec());
        cpu.stack = 0xFC;
        memory.write(cpu.get_stack_pointer().into(), [0x10, 0x44].to_vec());

        // Execute instruction
        cpu.program_counter = 0x01;
        Opcode0x60::execute(&mut cpu, &mut memory);

        // Assert results
        assert_eq!(cpu.program_counter, 0x4411);
        assert_eq!(cpu.stack, 0xFE);
    }
}
