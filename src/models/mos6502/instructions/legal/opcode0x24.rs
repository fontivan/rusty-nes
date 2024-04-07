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

pub struct Opcode0x24 {}

impl Opcode for Opcode0x24 {
    fn get_name() -> String {
        "0x24".to_string()
    }

    fn execute(mut _system: &mut Mos6502) {
        // Perform some bit tests and set appropriate flags using a zero paged address

        // Store value from zero paged address into the accumulator
        let instruction_arg: u16 = _system.get_instruction_argument(_system.program_counter, 1);

        // Increase the program counter
        _system.register_add(Register::ProgramCounter, 1);

        // Get the zero page address
        let address: u16 = Utils::get_zero_paged_address(instruction_arg.try_into().unwrap(), 0);

        // Get the value from memory
        let value: u8 = _system.memory.read(address.into(), 1)[0];

        // Test #1 - If the result of the value AND'd with the accumulator is zero then set z
        if _system.accumulator & value == 0 {
            _system.set_z_flag();
        } else {
            _system.clear_z_flag();
        }

        // Test #2 - If bit 7 of the original value from memory was 1 then set n
        if value & 0b1000_0000 == 0b1000_0000 {
            _system.set_n_flag();
        } else {
            _system.clear_n_flag();
        }

        // Test #3 - If bit 6 of the original value from memory was 1 then set v
        if value & 0b0100_0000 == 0b0100_0000 {
            _system.set_v_flag();
        } else {
            _system.clear_v_flag();
        }
    }
}

#[cfg(test)]

mod tests {
    use super::*;
    use crate::models::mos6502::tests::get_test_mos6502;

    #[test]
    fn test_execute_with_nv() {
        // Prep for the test
        let mut system: Mos6502 = get_test_mos6502(1024, 1000000.0);

        system.program_counter = 0x01;
        system.accumulator = 0xf1;
        system.memory.write(0, [0x24, 0x44].to_vec());
        system.memory.write(0x44, [0xfa].to_vec());

        // Execute instruction
        Opcode0x24::execute(&mut system);

        // Assert results
        assert!(system.is_n_set());
        assert!(system.is_v_set());
        assert!(!system.is_z_set());
    }

    #[test]
    fn test_execute_with_no_flags() {
        // Prep for the test
        let mut system: Mos6502 = get_test_mos6502(1024, 1000000.0);

        system.program_counter = 0x01;
        system.accumulator = 0xf1;
        system.memory.write(0, [0x24, 0x44].to_vec());
        system.memory.write(0x44, [0x0f].to_vec());

        // Execute instruction
        Opcode0x24::execute(&mut system);

        // Assert results
        assert!(!system.is_n_set());
        assert!(!system.is_v_set());
        assert!(!system.is_z_set());
    }

    #[test]
    fn test_execute_with_nvz() {
        // Prep for the test
        let mut system: Mos6502 = get_test_mos6502(1024, 1000000.0);

        system.program_counter = 0x01;
        system.accumulator = 0x00;
        system.memory.write(0, [0x24, 0x44].to_vec());
        system.memory.write(0x44, [0xff].to_vec());

        // Execute instruction
        Opcode0x24::execute(&mut system);

        // Assert results
        assert!(system.is_n_set());
        assert!(system.is_v_set());
        assert!(system.is_z_set());
    }

    #[test]
    fn test_execute_with_n() {
        // Prep for the test
        let mut system: Mos6502 = get_test_mos6502(1024, 1000000.0);

        system.program_counter = 0x01;
        system.accumulator = 0xaa;
        system.memory.write(0, [0x24, 0x44].to_vec());
        system.memory.write(0x44, [0b1000_0000].to_vec());

        // Execute instruction
        Opcode0x24::execute(&mut system);

        // Assert results
        assert!(system.is_n_set());
        assert!(!system.is_v_set());
        assert!(!system.is_z_set());
    }

    #[test]
    fn test_execute_with_v() {
        // Prep for the test
        let mut system: Mos6502 = get_test_mos6502(1024, 1000000.0);

        system.program_counter = 0x01;
        system.accumulator = 0xff;
        system.memory.write(0, [0x24, 0x44].to_vec());
        system.memory.write(0x44, [0b0100_0000].to_vec());

        // Execute instruction
        Opcode0x24::execute(&mut system);

        // Assert results
        assert!(!system.is_n_set());
        assert!(system.is_v_set());
        assert!(!system.is_z_set());
    }
}
