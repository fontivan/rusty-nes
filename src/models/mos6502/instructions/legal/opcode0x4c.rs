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

pub struct Opcode0x4c {}

impl Opcode for Opcode0x4c {
    fn get_name() -> String {
        "0x4c".to_string()
    }

    fn execute(mut _system: &mut Mos6502) {
        // Jump directly to operand address
        let address = _system.get_instruction_argument(_system.program_counter, 2);

        // Write data to program counter
        _system.program_counter = address;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::models::mos6502::tests::get_test_mos6502;

    #[test]
    fn test_execute() {
        // Prep for the test
        let mut system: Mos6502 = get_test_mos6502(1024, 1000000.0);
        system.memory.write(0, [0x4c, 0x12, 0x34].to_vec());

        // Execute instruction
        system.program_counter = 0x01;
        Opcode0x4c::execute(&mut system);

        // Assert results
        assert_eq!(system.program_counter, 0x3412);
    }
}
