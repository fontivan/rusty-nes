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

use crate::models::mos6502::instructions::Opcode;
use crate::models::mos6502::Mos6502;
use crate::models::mos6502::Register;

pub struct Opcode0x0e {}

impl Opcode for Opcode0x0e {
    fn get_name() -> String {
        "0x0e".to_string()
    }

    fn execute(mut _system: &mut Mos6502) {
        // Arithmetic shift left on an absolute page address

        // Get the address offset
        let address: u16 = _system.get_instruction_argument(_system.program_counter, 2);

        // Increase PC by amount of bytes read
        _system.register_add(Register::ProgramCounter, 2);

        // Read the value
        let mut value = _system.memory.read(address.into(), 1)[0];

        // The carry would be lose by the bitwise shift below so we need to snag it first
        let carry = value & 0b1000_0000 == 0b1000_0000;

        // Shift left
        value <<= 1;

        // Check for one in bit 0
        if carry {
            // Set the carry bit
            _system.set_c_flag();
        }

        // Check for 0 or negative flags
        _system.check_result_for_zero_and_negative_flags(value);

        // Write the new data back into memory
        _system.memory.write(address.into(), [value].to_vec());
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
        system.memory.write(0, [0x06, 0x00, 0x44].to_vec());
        system.memory.write(0x4400, [0b0101_1010].to_vec());

        // Execute instruction
        Opcode0x0e::execute(&mut system);

        // Assert results
        let result: u8 = system.memory.read(0x4400, 1)[0];
        assert_eq!(result, 0b1011_0100);
        assert!(!system.is_c_set());
        assert!(!system.is_z_set());
        assert!(system.is_n_set());
    }

    #[test]
    fn test_with_carry() {
        // Prep for the test
        let mut system: Mos6502 = get_test_mos6502(18000, 1000000.0);

        system.program_counter = 0x01;
        system.memory.write(0, [0x06, 0x00, 0x44].to_vec());
        system.memory.write(0x4400, [0b1101_1010].to_vec());

        // Execute instruction
        Opcode0x0e::execute(&mut system);

        // Assert results
        let result: u8 = system.memory.read(0x4400, 1)[0];
        assert_eq!(result, 0b1011_0100);
        assert!(system.is_c_set());
        assert!(!system.is_z_set());
        assert!(system.is_n_set());
    }
}
