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

pub struct Opcode0xe8 {}

impl Opcode for Opcode0xe8 {
    fn get_name() -> String {
        "0xe8".to_string()
    }

    fn execute(mut _system: &mut Mos6502) {
        // Increment the x register
        _system.register_add(Register::XIndex, 1);

        _system.check_result_for_zero_and_negative_flags(_system.x_index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::models::mos6502::tests::get_test_mos6502;

    #[test]
    fn test_no_flags() {
        // Prep for the test
        let mut system: Mos6502 = get_test_mos6502(1024, 1000000.0);

        system.x_index = 0x0F;

        // Execute instruction
        Opcode0xe8::execute(&mut system);

        // Assert results
        assert_eq!(system.x_index, 0x10);
        assert!(!system.is_z_set());
        assert!(!system.is_n_set());
        assert!(!system.is_v_set());
    }

    #[test]
    fn test_n_flag() {
        // Prep for the test
        let mut system: Mos6502 = get_test_mos6502(1024, 1000000.0);

        system.x_index = 0xF1;

        // Execute instruction
        Opcode0xe8::execute(&mut system);

        // Assert results
        assert_eq!(system.x_index, 0xF2);
        assert!(!system.is_z_set());
        assert!(system.is_n_set());
        assert!(!system.is_v_set());
    }

    #[test]
    fn test_z_flag() {
        // Prep for the test
        let mut system: Mos6502 = get_test_mos6502(1024, 1000000.0);

        system.x_index = 0xFF;

        // Execute instruction
        Opcode0xe8::execute(&mut system);

        // Assert results
        assert_eq!(system.x_index, 0x00);
        assert!(system.is_z_set());
        assert!(!system.is_n_set());
        assert!(system.is_v_set());
    }
}
