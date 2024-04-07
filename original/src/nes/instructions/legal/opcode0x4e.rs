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
use crate::nes::architecture::utils::Utils;
use crate::nes::instructions::Opcode;

pub struct Opcode0x4e {}

impl Opcode for Opcode0x4e {
    fn get_name() -> String {
        "0x4e".to_string()
    }

    fn execute(mut _cpu: &mut Cpu, mut _memory: &mut Memory) {
        // Get the operand data from the memory
        let instruction_arg: u16 = _memory.get_instruction_argument(_cpu.program_counter, 2);

        // Increase PC by amount of bytes read
        _cpu.register_add(Register::ProgramCounter, 2);

        // Get the address
        let address: usize = Utils::get_absolute_address(0, instruction_arg).into();

        // Fetch the data from memory
        let mut data: u8 = _memory.read(address, 1)[0];

        // Fetch the rightmost bit
        let carry: u8 = data & 0b0000_0001;

        // Rotate the bits right by 1 bit
        data >>= 1;

        // Set leftmost bit to 0
        data &= 0b0111_1111;

        // Write the data back to memory
        _memory.write(address, [data].to_vec());

        // If data is now zero, then set the zero flag high
        if data == 0 {
            _cpu.set_z_flag();
        } else {
            _cpu.clear_z_flag();
        }

        // Set carry flag to the value of the rightmost bit
        if carry == 0 {
            _cpu.clear_c_flag();
        } else {
            _cpu.set_c_flag();
        }

        // Shift right inserts 1 into bit 7, so N will always be cleared
        _cpu.clear_n_flag();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::nes::architecture::cpu::tests::get_test_cpu;
    use crate::nes::architecture::memory::tests::get_test_memory;

    #[test]
    fn test_without_carry() {
        // Prep for the test
        let mut cpu: Cpu = get_test_cpu();
        let mut memory: Memory = get_test_memory(18000);
        cpu.program_counter = 0x01;
        memory.write(0, [0x06, 0x00, 0x44].to_vec());
        memory.write(0x4400, [0b0101_1010].to_vec());

        // Execute instruction
        Opcode0x4e::execute(&mut cpu, &mut memory);

        // Assert results
        let result: u8 = memory.read(0x4400, 1)[0];
        assert_eq!(result, 0b0010_1101);
        assert!(!cpu.is_c_set());
        assert!(!cpu.is_z_set());
        assert!(!cpu.is_n_set());
    }

    #[test]
    fn test_with_carry() {
        // Prep for the test
        let mut cpu: Cpu = get_test_cpu();
        let mut memory: Memory = get_test_memory(18000);
        cpu.program_counter = 0x01;
        memory.write(0, [0x06, 0x00, 0x44].to_vec());
        memory.write(0x4400, [0b1101_1011].to_vec());

        // Execute instruction
        Opcode0x4e::execute(&mut cpu, &mut memory);

        // Assert results
        let result: u8 = memory.read(0x4400, 1)[0];
        assert_eq!(result, 0b0110_1101);
        assert!(cpu.is_c_set());
        assert!(!cpu.is_z_set());
        assert!(!cpu.is_n_set());
    }
}
