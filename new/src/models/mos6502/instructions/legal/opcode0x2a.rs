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

use crate::models::mos6502::Mos6502;
use crate::models::mos6502::instructions::Opcode;


pub struct Opcode0x2a {}

impl Opcode for Opcode0x2a {
    fn get_name() -> String {
        "0x2a".to_string()
    }

    fn execute(mut _system: &mut Mos6502) {
        // Rotate left on the accumulator register

        // Get the value from the accumulator
        let mut data = _system.accumulator;

        // Store the highest bit
        let high_bit = data & 0b1000_0000;

        // Shift the value
        data <<= 1;

        // Check for the carry flag and set the lowest bit of the data value if so
        if _system.is_c_set() {
            data |= 0b0000_0001;
        }

        // Save the value back in the accumulator
        _system.accumulator = data;

        // Save the high bit into the carry
        if high_bit != 0 {
            _system.set_c_flag();
        } else {
            _system.clear_c_flag();
        }
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
        Opcode0x2a::execute(&mut system);

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
        Opcode0x2a::execute(&mut system);

        // Assert results
        assert_eq!(system.accumulator, 0b1010_1010);
        assert!(system.is_c_set());
    }
}
