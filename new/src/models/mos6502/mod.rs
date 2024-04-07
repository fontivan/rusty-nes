////////////////////////////////////////////////////////////////////////////////////////////////////
// MIT License
//
// Copyright (c) 2024 fontivan
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
use crate::models::mos6502::instructions::decoder::Decoder;

pub struct Mos6502 {
    pub accumulator: Memory,
    pub clock: Clock,
    pub flags: Memory,
    pub memory: Memory,
    pub program_counter: Memory,
    pub stack: Memory,
    pub x_register: Memory,
    pub y_register: Memory,
}

impl Mos6502 {
    pub fn new(memory_size: usize, clock_speed_hz: f64) -> Mos6502 {
        Mos6502 {
            accumulator: Memory::new(1).unwrap(),
            clock: Clock::new(clock_speed_hz),
            flags: Memory::new(1).unwrap(),
            memory: Memory::new(memory_size).unwrap(),
            program_counter: Memory::new(2).unwrap(),
            stack: Memory::new(1).unwrap(),
            x_register: Memory::new(1).unwrap(),
            y_register: Memory::new(1).unwrap()
        }
    }

    pub fn run(&mut self) {
        loop {
            // Wait for clock cycle
            self.clock.tick();

            // Get address
            let target_address = self.get_address_from_program_counter();

            // Fetch the address from memory
            let instruction_data: Vec<u8> = self.memory.read(target_address.into(), 1);

            // Increment program counter
            self.program_counter.increment_data_at_address(0);

            // Decode and execute
            Decoder::execute(self, instruction_data[0]);
        }
    }

    // Flag bit 0 - Carry
    // Set when the accumulator rolls over from 0xFF to 0x00, or as part of some operations
    pub fn set_c_flag(&mut self) {
        self.flags.set_bit(0, 0b0000_0001);
    }

    pub fn clear_c_flag(&mut self) {
        self.flags.clear_bit(0, 0b0000_0001);
    }

    pub fn is_c_set(&mut self) -> bool {
        return self.flags.get_bit(0, 0b0000_0001)
    }

    // Flag bit 1 - Zero
    // Set when the result of most instructions is 0x00
    pub fn set_z_flag(&mut self) {
        self.flags.set_bit(0, 0b0000_0010);
    }

    pub fn clear_z_flag(&mut self) {
        self.flags.clear_bit(0, 0b0000_0010);
    }

    pub fn is_z_set(&mut self) -> bool {
        return self.flags.get_bit(0, 0b0000_0010)
    }

    // Flag bit 2 - Interrupt
    // Set when various interrupt methods are called
    pub fn set_i_flag(&mut self) {
        self.flags.set_bit(0, 0b0000_0100);
    }

    pub fn clear_i_flag(&mut self) {
        self.flags.clear_bit(0, 0b0000_0100);
    }

    pub fn is_i_set(&mut self) -> bool {
        return self.flags.get_bit(0, 0b0000_0100)
    }

    // Flag bit 3 - Decimal
    pub fn set_d_flag(&mut self) {
        self.flags.set_bit(0, 0b0000_1000);
    }

    pub fn clear_d_flag(&mut self) {
        self.flags.clear_bit(0, 0b0000_1000);
    }

    pub fn is_d_set(&mut self) -> bool {
        return self.flags.get_bit(0, 0b0000_1000)
    }

    // Flag bit 4 - Break
    pub fn set_b_flag(&mut self) {
        self.flags.set_bit(0, 0b0001_0000);
    }

    pub fn clear_b_flag(&mut self) {
        self.flags.clear_bit(0, 0b0001_0000);
    }

    pub fn is_b_set(&mut self) -> bool {
        return self.flags.get_bit(0, 0b0001_0000)
    }

    // Flag bit 5 - Unused

    // Flag bit 6 - Overflow
    pub fn set_v_flag(&mut self) {
        self.flags.set_bit(0, 0b0100_0000);
    }

    pub fn clear_v_flag(&mut self) {
        self.flags.clear_bit(0, 0b0100_0000);
    }

    pub fn is_v_set(&mut self) -> bool {
        return self.flags.get_bit(0, 0b0100_0000)
    }

    // Flag bit 7 - Negative
    // Set when the highest bit of the result is also set
    pub fn set_n_flag(&mut self) {
        self.flags.set_bit(0, 0b1000_0000);
    }

    pub fn clear_n_flag(&mut self) {
        self.flags.clear_bit(0, 0b1000_0000);
    }

    pub fn is_n_set(&mut self) -> bool {
        return self.flags.get_bit(0, 0b1000_0000)
    }

    pub fn get_address_from_program_counter(&mut self) -> usize {
        // Convert the PC data into a memory address
        let pc_data = self.program_counter.read(0, 2);

        // Convert pc data to an address
        let target_address = pc_data[0] as usize + ((pc_data[1] as u16) << 8) as usize;

        // Return address as usize
        return target_address;
    }

    pub fn get_instruction_argument(&mut self, argument_size: usize) -> Vec<u8> {
        // Instruction arguments can be 1 to 4 bytes and at located at the pc value
        let target_address = self.get_address_from_program_counter();
        let result = self.memory.read(target_address, argument_size);
        return result;
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
}

#[cfg(test)]
pub mod tests {
    use super::*;
    
    pub fn get_test_mos6502(memory_size: usize, clock_speed_hz: f64) -> Mos6502 {
        let mut system: Mos6502 = Mos6502::new(memory_size, clock_speed_hz);
        system.program_counter.write(0, [0x00, 0x00].to_vec());
        return system;
    }

    #[test]
    pub fn test_c_flag(){
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
    pub fn test_z_flag(){
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
    pub fn test_i_flag(){
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
    pub fn test_d_flag(){
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
    pub fn test_b_flag(){
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
    pub fn test_v_flag(){
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
    pub fn test_n_flag(){
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
