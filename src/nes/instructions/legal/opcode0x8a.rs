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

pub struct Opcode0x8a {}

impl Opcode for Opcode0x8a {
    fn get_name() -> String {
        "0x8a".to_string()
    }

    fn execute(mut _cpu: &mut Cpu, mut _memory: &mut Memory) {
        // Transfer from x register to the accumulator
        _cpu.accumulator = _cpu.x_index;

        _cpu.check_result_for_zero_and_negative_flags(_cpu.accumulator)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::nes::architecture::cpu::tests::get_test_cpu;
    use crate::nes::architecture::memory::tests::get_test_memory;

    #[test]
    fn test_no_flags() {
        // Prep for the test
        let mut cpu: Cpu = get_test_cpu();
        let mut memory: Memory = get_test_memory(1);
        cpu.x_index = 0x0F;

        // Execute instruction
        Opcode0x8a::execute(&mut cpu, &mut memory);

        // Assert results
        assert_eq!(cpu.accumulator, 0x0F);
        assert!(!cpu.is_z_set());
        assert!(!cpu.is_n_set());
    }

    #[test]
    fn test_n_flag() {
        // Prep for the test
        let mut cpu: Cpu = get_test_cpu();
        let mut memory: Memory = get_test_memory(1);
        cpu.x_index = 0xF0;

        // Execute instruction
        Opcode0x8a::execute(&mut cpu, &mut memory);

        // Assert results
        assert_eq!(cpu.accumulator, 0xF0);
        assert!(!cpu.is_z_set());
        assert!(cpu.is_n_set());
    }

    #[test]
    fn test_z_flag() {
        // Prep for the test
        let mut cpu: Cpu = get_test_cpu();
        let mut memory: Memory = get_test_memory(1);
        cpu.accumulator = 0x01;
        cpu.x_index = 0x00;

        // Execute instruction
        Opcode0x8a::execute(&mut cpu, &mut memory);

        // Assert results
        assert_eq!(cpu.accumulator, 0x00);
        assert!(cpu.is_z_set());
        assert!(!cpu.is_n_set());
    }
}
