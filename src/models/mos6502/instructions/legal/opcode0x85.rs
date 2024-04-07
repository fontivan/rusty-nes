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

pub struct Opcode0x85 {}

impl Opcode for Opcode0x85 {
    fn get_name() -> String {
        "0x85".to_string()
    }

    fn execute(mut _system: &mut Mos6502) {
        // Store value from zero paged address into the accumulator
        let instruction_arg: u16 = _system.get_instruction_argument(_system.program_counter, 1);

        // Increase the program counter
        _system.register_add(Register::ProgramCounter, 1);

        // Get the zero page address
        let address: u16 = Utils::get_zero_paged_address(instruction_arg.try_into().unwrap(), 0);

        // Get the value from memory
        let value: u8 = _system.memory.read(address.into(), 1)[0];

        // Save the value in the accumulator
        _system.accumulator = value;
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

        system.program_counter = 0x01;
        system.memory.write(0, [0x85, 0x44].to_vec());
        system.memory.write(0x44, [0xfa].to_vec());

        // Execute instruction
        Opcode0x85::execute(&mut system);

        // Assert results
        assert_eq!(system.accumulator, 0xfa);
    }
}
