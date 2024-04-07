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

pub struct Opcode0x0a {}

impl Opcode for Opcode0x0a {
    fn get_name() -> String {
        "0x0a".to_string()
    }

    fn execute(mut _system: &mut Mos6502) {
        // Arithmetic shift left on the accumulator register

        // Get the value from the accumulator
        let mut value = _system.accumulator;

        // Get the high bit from the value
        let high_bit = value & 0b1000_0000;

        // Shift the value
        value <<= 1;

        // Save the value back to the accumulator
        _system.accumulator = value;

        // Save the high bit into the carry
        if high_bit != 0 {
            _system.set_c_flag();
        } else {
            _system.clear_c_flag();
        }

        _system.check_result_for_zero_and_negative_flags(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::models::mos6502::tests::get_test_mos6502;

    #[test]
    fn test_without_carry() {
        // Prep for the test
        let mut system: Mos6502 = get_test_mos6502(1024, 1000000.0);

        system.accumulator = 0b0101_0101;
        system.clear_c_flag();

        // Execute instruction
        Opcode0x0a::execute(&mut system);

        // Assert results
        assert_eq!(system.accumulator, 0b1010_1010);
        assert!(!system.is_c_set());
    }

    #[test]
    fn test_with_carry() {
        // Prep for the test
        let mut system: Mos6502 = get_test_mos6502(1024, 1000000.0);

        system.accumulator = 0b1101_0101;
        system.clear_c_flag();

        // Execute instruction
        Opcode0x0a::execute(&mut system);

        // Assert results
        assert_eq!(system.accumulator, 0b1010_1010);
        assert!(system.is_c_set());
    }
}
