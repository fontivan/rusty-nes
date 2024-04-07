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

pub struct Opcode0xaa {}

impl Opcode for Opcode0xaa {
    fn get_name() -> String {
        "0xaa".to_string()
    }

    fn execute(mut _system: &mut Mos6502) {
        // Transfer from the accumulator to x register
        _system.x_index = _system.accumulator;

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

        system.accumulator = 0x0F;

        // Execute instruction
        Opcode0xaa::execute(&mut system);

        // Assert results
        assert_eq!(system.x_index, 0x0F);
        assert!(!system.is_z_set());
        assert!(!system.is_n_set());
    }

    #[test]
    fn test_n_flag() {
        // Prep for the test
        let mut system: Mos6502 = get_test_mos6502(1024, 1000000.0);

        system.accumulator = 0xF0;

        // Execute instruction
        Opcode0xaa::execute(&mut system);

        // Assert results
        assert_eq!(system.x_index, 0xF0);
        assert!(!system.is_z_set());
        assert!(system.is_n_set());
    }

    #[test]
    fn test_z_flag() {
        // Prep for the test
        let mut system: Mos6502 = get_test_mos6502(1024, 1000000.0);

        system.x_index = 0x01;
        system.accumulator = 0x00;

        // Execute instruction
        Opcode0xaa::execute(&mut system);

        // Assert results
        assert_eq!(system.x_index, 0x00);
        assert!(system.is_z_set());
        assert!(!system.is_n_set());
    }
}
