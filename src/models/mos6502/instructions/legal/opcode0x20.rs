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

pub struct Opcode0x20 {}

impl Opcode for Opcode0x20 {
    fn get_name() -> String {
        "0x20".to_string()
    }

    fn execute(mut _system: &mut Mos6502) {
        // Jump to a subroutine

        // Get the target address
        let jump_address: u16 = _system.get_instruction_argument(_system.program_counter, 2);

        // The instruction specifies that we are to set the return address minus one to the stack
        // Since we would otherwise add two here, just add one instead of doing redundant math
        let return_address: u16 = _system.program_counter + 1;
        let address_bytes = Utils::get_u8_pair_from_u16(return_address);

        // These are to be pushed into the stack bytewise, highest byte first
        _system.stack_push(address_bytes.0);
        _system.stack_push(address_bytes.1);

        // Set the program counter
        _system.program_counter = jump_address;
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

        system.stack = 0xFE;
        system.program_counter = 0xF5;
        system.memory.write(0xF4, [0x20, 0x11, 0x44].to_vec());

        // Execute instruction
        Opcode0x20::execute(&mut system);

        // Assert results
        let stack_dump: Vec<u8> = system.memory.read(0x0100, 255);
        assert_eq!(system.program_counter, 0x4411);
        assert_eq!(stack_dump[253], 0xF6);
        assert_eq!(stack_dump[254], 0x00);
    }

    #[test]
    fn test_stack_overflow() {
        // Prep for the test
        let mut system: Mos6502 = get_test_mos6502(1024, 1000000.0);

        system.stack = 0x00;
        // Write garbage to the stack memory

        let mut i: u8 = 0;
        loop {
            let mut address: usize = i.into();
            address += 0x0100;

            let data: Vec<u8> = [141].to_vec();
            system.memory.write(address, data);

            if i < 255 {
                i += 1;
            } else {
                break;
            }
        }

        system.program_counter = 0xF5;
        system.memory.write(0xF4, [0x20, 0x11, 0x44].to_vec());

        // Execute instruction
        Opcode0x20::execute(&mut system);

        // Assert results
        let stack_dump: Vec<u8> = system.memory.read(0x0100, 255);
        assert_eq!(system.program_counter, 0x4411);
        assert_eq!(system.stack, 0xFD);
        assert_eq!(stack_dump[0], 0x00);
        assert_eq!(stack_dump[254], 0xF6);
    }
}
