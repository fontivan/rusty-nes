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

pub struct Opcode0x4a {}

impl Opcode for Opcode0x4a {
    fn get_name() -> String {
        "0x4a".to_string()
    }

    fn execute(mut _system: &mut Mos6502) {
        // Fetch the rightmost bit
        let carry: bool = _system.accumulator & 0b0000_0001 == 0b0000_0001;

        // Rotate the bits in the accumlator to the right by 1 bit
        _system.accumulator >>= 1;

        // If data is now zero, then set the zero flag high
        if _system.accumulator == 0 {
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

        // Shift right inserts a 0 into bit 7, so N will always be cleared
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
        let mut system: Mos6502 = get_test_mos6502(1024, 1000000.0);

        system.accumulator = 0b1100_0110;
        system.clear_c_flag();

        // Execute instruction
        Opcode0x4a::execute(&mut system);

        // Assert results
        assert_eq!(system.accumulator, 0b0110_0011);
        assert!(!system.is_c_set());
    }

    #[test]
    fn test_with_carry() {
        // Prep for the test
        let mut system: Mos6502 = get_test_mos6502(1024, 1000000.0);

        system.accumulator = 0b1100_0011;
        system.clear_c_flag();

        // Execute instruction
        Opcode0x4a::execute(&mut system);

        // Assert results
        assert_eq!(system.accumulator, 0b0110_0001);
        assert!(system.is_c_set());
    }
}
