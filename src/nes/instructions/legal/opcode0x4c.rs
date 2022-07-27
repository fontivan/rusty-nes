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

pub struct Opcode0x4c {}

impl Opcode for Opcode0x4c {
    fn get_name() -> String {
        "0x4c".to_string()
    }

    fn execute(mut _cpu: &mut Cpu, mut _memory: &mut Memory) {
        // Jump directly to operand address
        let address: u16 = _memory.get_instruction_argument(_cpu.program_counter, 2);
        _cpu.program_counter = address;
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
        let memory_size: usize = 16;
        let memory_result: Result<Memory, usize> = Memory::new(memory_size);
        let mut memory: Memory = memory_result.unwrap();
        return memory;
    }

    #[test]
    fn test_execute() {
        // Prep for the test
        let mut cpu: Cpu = get_test_cpu();
        let mut memory: Memory = get_test_memory();
        memory.write(0, [0x4c, 0x12, 0x34].to_vec());

        // Execute instruction
        cpu.program_counter = 0x01;
        Opcode0x4c::execute(&mut cpu, &mut memory);

        // Assert results
        assert_eq!(cpu.program_counter, 0x3412);
    }
}
