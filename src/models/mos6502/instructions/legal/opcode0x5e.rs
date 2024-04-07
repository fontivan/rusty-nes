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

use crate::common::utils::Utils;
use crate::models::mos6502::instructions::Opcode;
use crate::models::mos6502::Mos6502;
use crate::models::mos6502::Register;

pub struct Opcode0x5e {}

impl Opcode for Opcode0x5e {
    fn get_name() -> String {
        "0x5e".to_string()
    }

    fn execute(mut _system: &mut Mos6502) {
        // Get the operand data from the memory
        let instruction_arg: u16 = _system.get_instruction_argument(_system.program_counter, 2);

        // Increase PC by amount of bytes read
        _system.register_add(Register::ProgramCounter, 2);

        // Get the address
        let address: usize = Utils::get_absolute_address(_system.x_index, instruction_arg).into();

        // Fetch the data from memory using the x index as an offset
        let mut data: u8 = _system.memory.read(address, 1)[0];

        // Fetch the rightmost bit
        let carry: bool = data & 0b0000_0001 == 0b0000_0001;

        // Rotate the bits right by 1 bit
        data >>= 1;

        // Write the data back to memory
        _system.memory.write(address, [data].to_vec());

        // If data is now zero, then set the zero flag high
        if data == 0 {
            _system.set_z_flag();
        } else {
            _system.clear_z_flag();
        }

        // Set carry flag to the value of the rightmost bit
        if carry {
            _system.set_c_flag();
        } else {
            _system.clear_c_flag();
        }

        // Shift right inserts 1 into bit 7, so N will always be cleared
        _system.clear_n_flag();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::models::mos6502::tests::get_test_mos6502;

    #[test]
    fn test_without_carry() {
        // Prep for the test
        let mut system: Mos6502 = get_test_mos6502(18000, 1000000.0);

        system.program_counter = 0x01;
        system.memory.write(0, [0x5e, 0x00, 0x44, 0x58].to_vec());
        system.memory.write(0x4400 + 0xF0, [0b1010_1010].to_vec());

        system.x_index = 0xF0;
        system.clear_c_flag();

        // Execute instruction
        Opcode0x5e::execute(&mut system);

        // Assert results
        let result: u8 = system.memory.read(0x4400 + 0xF0, 1)[0];
        assert_eq!(result, 0b0101_0101);
        assert!(!system.is_c_set());
        assert_eq!(system.program_counter, 0x03);
    }

    #[test]
    fn test_with_carry() {
        // Prep for the test
        let mut system: Mos6502 = get_test_mos6502(18000, 1000000.0);

        system.program_counter = 0x01;
        system.memory.write(0, [0x5e, 0x00, 0x44, 0x58].to_vec());
        system.memory.write(0x4400 + 0xF0, [0b1010_0101].to_vec());

        system.x_index = 0xF0;
        system.clear_c_flag();

        // Execute instruction
        Opcode0x5e::execute(&mut system);

        // Assert results
        let result: u8 = system.memory.read(0x4400 + 0xF0, 1)[0];
        assert_eq!(result, 0b0101_0010);
        assert!(system.is_c_set());
        assert_eq!(system.program_counter, 0x03);
    }
}
