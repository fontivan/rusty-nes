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

mod instructions;

use crate::common::clock::Clock;
use crate::common::memory::Memory;
use crate::common::utils::Utils;
use crate::models::mos6502::instructions::decoder::Decoder;
use std::convert::TryFrom;

pub enum Register {
    Accumulator,
    Flags,
    ProgramCounter,
    Stack,
    XIndex,
    YIndex,
}

pub struct Mos6502 {
    pub accumulator: u8,
    pub clock: Clock,
    pub flags: u8,
    pub memory: Memory,
    pub program_counter: u16,
    pub stack: u8,
    pub x_index: u8,
    pub y_index: u8,
}

impl Mos6502 {
    pub fn new(memory_size: usize, clock_speed_hz: f64) -> Mos6502 {
        Mos6502 {
            accumulator: 0,
            clock: Clock::new(clock_speed_hz),
            flags: 0,
            memory: Memory::new(memory_size).unwrap(),
            program_counter: 0x34,
            stack: 0xFD,
            x_index: 0,
            y_index: 0,
        }
    }

    pub fn run(&mut self) {
        loop {
            // Wait for clock cycle
            self.clock.tick();

            // Fetch the address from memory
            let instruction_data: Vec<u8> = self.memory.read(self.program_counter.into(), 1);

            // Increment program counter
            self.program_counter = self.program_counter + 1;

            // Decode and execute
            Decoder::execute(self, instruction_data[0]);
        }
    }

    // Flag bit 0 - Carry
    // Set when the accumulator rolls over from 0xFF to 0x00, or as part of some operations
    pub fn set_c_flag(&mut self) {
        self.flags |= 0b0000_0001;
    }

    pub fn clear_c_flag(&mut self) {
        self.flags &= 0b1111_1110;
    }

    pub fn is_c_set(&mut self) -> bool {
        self.flags & 0b0000_0001 == 0b0000_0001
    }

    // Flag bit 1 - Zero
    // Set when the result of most instructions is 0x00
    pub fn set_z_flag(&mut self) {
        self.flags |= 0b0000_0010;
    }

    pub fn clear_z_flag(&mut self) {
        self.flags &= 0b1111_1101;
    }

    pub fn is_z_set(&mut self) -> bool {
        self.flags & 0b0000_0010 == 0b0000_0010
    }

    // Flag bit 2 - Interrupt
    // Set when various interrupt methods are called
    pub fn set_i_flag(&mut self) {
        self.flags |= 0b0000_0100;
    }

    pub fn clear_i_flag(&mut self) {
        self.flags &= 0b1111_1011;
    }

    pub fn is_i_set(&mut self) -> bool {
        self.flags & 0b0000_0100 == 0b0000_0100
    }

    // Flag bit 3 - Decimal
    pub fn set_d_flag(&mut self) {
        self.flags |= 0b0000_1000;
    }

    pub fn clear_d_flag(&mut self) {
        self.flags &= 0b1111_0111;
    }

    pub fn is_d_set(&mut self) -> bool {
        self.flags & 0b0000_1000 == 0b0000_1000
    }

    // Flag bit 4 - Break
    pub fn set_b_flag(&mut self) {
        self.flags |= 0b0001_0000;
    }

    pub fn clear_b_flag(&mut self) {
        self.flags &= 0b1110_1111;
    }

    pub fn is_b_set(&mut self) -> bool {
        self.flags & 0b0001_0000 == 0b0001_0000
    }

    // Flag bit 5 - Unused

    // Flag bit 6 - Overflow
    pub fn set_v_flag(&mut self) {
        self.flags |= 0b0100_0000;
    }

    pub fn clear_v_flag(&mut self) {
        self.flags &= 0b1011_1111;
    }

    pub fn is_v_set(&mut self) -> bool {
        self.flags & 0b0100_0000 == 0b0100_0000
    }

    // Flag bit 7 - Negative
    // Set when the highest bit of the result is also set
    pub fn set_n_flag(&mut self) {
        self.flags |= 0b1000_0000;
    }

    pub fn clear_n_flag(&mut self) {
        self.flags &= 0b0111_1111;
    }

    pub fn is_n_set(&mut self) -> bool {
        self.flags & 0b1000_0000 == 0b1000_0000
    }

    pub fn register_add(&mut self, register: Register, operand: isize) {
        match register {
            Register::Accumulator => {
                let mut result: isize = self.accumulator.into();
                result = result + operand;
                if result > std::u8::MAX.into() {
                    self.set_v_flag();
                    result &= 0b0000_0000_1111_1111;
                }
                if result < 0 {
                    result += 0xFF
                }
                match u8::try_from(result) {
                    Ok(result) => {
                        self.accumulator = result;
                    }
                    Err(error) => {
                        std::panic::panic_any(error);
                    }
                }
            }
            Register::Flags => {
                panic!("Should not be adding integers to flag register")
            }
            Register::ProgramCounter => {
                let mut result: isize = isize::try_from(self.program_counter).unwrap();
                result = result + operand;
                if result > isize::try_from(std::u16::MAX).unwrap() {
                    self.set_v_flag();
                    result &= 0x00FF;
                }
                if result < 0 {
                    result += 0xFF
                }
                match u16::try_from(result) {
                    Ok(result) => {
                        self.program_counter = result;
                    }
                    Err(error) => {
                        std::panic::panic_any(error);
                    }
                }
            }
            Register::Stack => {
                let mut result: isize = self.stack.into();
                result = result + operand;
                if result > std::u8::MAX.into() {
                    self.set_v_flag();
                    result &= 0b0000_0000_1111_1111;
                }
                if result < 0 {
                    result += 0xFF
                }
                match u8::try_from(result) {
                    Ok(result) => {
                        self.stack = result;
                    }
                    Err(error) => {
                        std::panic::panic_any(error);
                    }
                }
            }
            Register::XIndex => {
                let mut result: isize = self.x_index.into();
                result = result + operand;
                if result > std::u8::MAX.into() {
                    self.set_v_flag();
                    result &= 0b0000_0000_1111_1111;
                }
                if result < 0 {
                    result += 0xFF
                }
                match u8::try_from(result) {
                    Ok(result) => {
                        self.x_index = result;
                    }
                    Err(error) => {
                        std::panic::panic_any(error);
                    }
                }
            }
            Register::YIndex => {
                let mut result: isize = self.y_index.into();
                result = result + operand;
                if result > std::u8::MAX.into() {
                    self.set_v_flag();
                    result &= 0b0000_0000_1111_1111;
                }
                if result < 0 {
                    result += 0xFF
                }
                match u8::try_from(result) {
                    Ok(result) => {
                        self.y_index = result;
                    }
                    Err(error) => {
                        std::panic::panic_any(error);
                    }
                }
            }
        }
    }

    pub fn get_stack_pointer(&mut self) -> u16 {
        // Build the stack pointer from the register and page
        let mut stack_pointer: u16 = 0x0100;
        let stack_register_value: u16 = self.stack.into();
        return stack_pointer | stack_register_value;
    }

    pub fn get_branch_relative_jump(&mut self, offset: u16) -> isize {
        // If carry is set then we need to figure out where we are branching to
        let offset: u16 = self.get_instruction_argument(offset, 1);

        // This may be a 2s complement negative number
        if offset & 0b1000_0000 == 0b1000_0000 {
            // 2s complement
            let magnitude: usize = Utils::get_twos_complement_magnitude(offset.into(), 8);
            let mut value: isize = magnitude.try_into().unwrap();
            value = value * -1;
            return value;
        }

        return offset.try_into().unwrap();
    }

    pub fn get_instruction_argument(&mut self, offset: u16, size: usize) -> u16 {
        // We expect this to be between 1 and 4 bytes
        assert!(size >= 1);
        assert!(size <= 2);

        // First get the program counter and read the data stored after the instruction
        let data: Vec<u8> = self.memory.read(offset.into(), size);

        match size {
            1 => {
                let mut result: u16 = data[0].into();
                return result.into();
            }
            2 => {
                let mut result: u16 = data[1].into();
                result <<= 8;
                let data16: u16 = data[0].into();
                result |= data16;
                return result;
            }
            _ => {
                panic!("This shouldn't be possible!");
            }
        }
    }

    // This function will be called by a large number of instructions to check if the z and n flags should be set
    pub fn check_result_for_zero_and_negative_flags(&mut self, result: u8) {
        // If the last result was 0 then the zero flag must be set
        if result == 0 {
            self.set_z_flag()
        } else {
            self.clear_z_flag()
        }

        // If the highest bit of the last result was 1 then the negative flag must be set
        if result & 0b1000_0000 == 0b1000_0000 {
            self.set_n_flag()
        } else {
            self.clear_n_flag()
        }
    }

    pub fn stack_pop(&mut self) -> u8 {
        let stack_pointer: u16 = self.get_stack_pointer();

        // Read the value from memory
        let value: u8 = self.memory.read(stack_pointer.into(), 1)[0];

        // Clear the value from the stack
        self.memory.write(stack_pointer.into(), [0x00].to_vec());

        // Increment the stack pointer
        self.register_add(Register::Stack, 1);

        return value;
    }

    pub fn stack_push(&mut self, value: u8) {
        let stack_pointer: u16 = self.get_stack_pointer();

        // Write the value into memory
        self.memory.write(stack_pointer.into(), [value].to_vec());

        // Decrement the stack register
        self.register_add(Register::Stack, -1);
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    pub fn get_test_mos6502(memory_size: usize, clock_speed_hz: f64) -> Mos6502 {
        let mut system: Mos6502 = Mos6502::new(memory_size, clock_speed_hz);
        system.program_counter = 0x0000;
        return system;
    }

    #[test]
    pub fn test_c_flag() {
        // Get a system
        let mut system: Mos6502 = get_test_mos6502(1024, 1000000.0);

        // Verify flag low
        assert!(!system.is_c_set());

        // Set flag high
        system.set_c_flag();

        // Verify flag high
        assert!(system.is_c_set());

        // Set flag low
        system.clear_c_flag();

        // Verify flag low
        assert!(!system.is_c_set());
    }

    #[test]
    pub fn test_z_flag() {
        // Get a system
        let mut system: Mos6502 = get_test_mos6502(1024, 1000000.0);

        // Verify flag low
        assert!(!system.is_z_set());

        // Set flag high
        system.set_z_flag();

        // Verify flag high
        assert!(system.is_z_set());

        // Set flag low
        system.clear_z_flag();

        // Verify flag low
        assert!(!system.is_z_set());
    }

    #[test]
    pub fn test_i_flag() {
        // Get a system
        let mut system: Mos6502 = get_test_mos6502(1024, 1000000.0);

        // Verify flag low
        assert!(!system.is_i_set());

        // Set flag high
        system.set_i_flag();

        // Verify flag high
        assert!(system.is_i_set());

        // Set flag low
        system.clear_i_flag();

        // Verify flag low
        assert!(!system.is_i_set());
    }

    #[test]
    pub fn test_d_flag() {
        // Get a system
        let mut system: Mos6502 = get_test_mos6502(1024, 1000000.0);

        // Verify flag low
        assert!(!system.is_d_set());

        // Set flag high
        system.set_d_flag();

        // Verify flag high
        assert!(system.is_d_set());

        // Set flag low
        system.clear_d_flag();

        // Verify flag low
        assert!(!system.is_d_set());
    }

    #[test]
    pub fn test_b_flag() {
        // Get a system
        let mut system: Mos6502 = get_test_mos6502(1024, 1000000.0);

        // Verify flag low
        assert!(!system.is_b_set());

        // Set flag high
        system.set_b_flag();

        // Verify flag high
        assert!(system.is_b_set());

        // Set flag low
        system.clear_b_flag();

        // Verify flag low
        assert!(!system.is_b_set());
    }

    #[test]
    pub fn test_v_flag() {
        // Get a system
        let mut system: Mos6502 = get_test_mos6502(1024, 1000000.0);

        // Verify flag low
        assert!(!system.is_v_set());

        // Set flag high
        system.set_v_flag();

        // Verify flag high
        assert!(system.is_v_set());

        // Set flag low
        system.clear_v_flag();

        // Verify flag low
        assert!(!system.is_v_set());
    }

    #[test]
    pub fn test_n_flag() {
        // Get a system
        let mut system: Mos6502 = get_test_mos6502(1024, 1000000.0);

        // Verify flag low
        assert!(!system.is_n_set());

        // Set flag high
        system.set_n_flag();

        // Verify flag high
        assert!(system.is_n_set());

        // Set flag low
        system.clear_n_flag();

        // Verify flag low
        assert!(!system.is_n_set());
    }
}
