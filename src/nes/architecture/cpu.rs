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

use crate::nes::architecture::decoder::Decoder;
use crate::nes::architecture::memory::Memory;

pub struct Cpu {
    pub accumulator: u8,
    // Flag usage is derived from the nesdev wiki
    // https://wiki.nesdev.com/w/index.php/Status_flags
    pub flags: u8,
    pub program_counter: u16,
    pub stack: u8,
    pub x_index: u8,
    pub y_index: u8,
}

impl Cpu {
    // Constructor for Cpu
    // This is done in the "Power Up" state as described by the nesdev wiki
    // https://wiki.nesdev.com/w/index.php/CPU_power_up_state#At_power-up
    pub fn new() -> Cpu {
        return Cpu {
            accumulator: 0,
            flags: 0,
            program_counter: 0x34,
            stack: 0xFD,
            x_index: 0,
            y_index: 0,
        };
    }

    // Flag bit 0 - Carry
    // Set when the accumulator rolls over from 0xFF to 0x00, or as part of some operations
    pub fn set_c_flag(&mut self) {
        self.flags = self.flags | 0b0000_0001;
    }

    pub fn clear_c_flag(&mut self) {
        self.flags = self.flags & 0b1111_1110;
    }

    pub fn is_c_set(&mut self) -> bool {
        return self.flags & 0b0000_0001 == 0b0000_0001
    }

    // Flag bit 1 - Zero
    // Set when the result of most instructions is 0x00
    pub fn set_z_flag(&mut self) {
        self.flags = self.flags | 0b0000_0010;
    }

    pub fn clear_z_flag(&mut self) {
        self.flags = self.flags & 0b1111_1101;
    }

    pub fn is_z_set(&mut self) -> bool {
        return self.flags & 0b0000_0010 == 0b0000_0010
    }

    // Flag bit 2 - Interrupt
    // Set when various interrupt methods are called
    pub fn set_i_flag(&mut self) {
        self.flags = self.flags | 0b0000_0100;
    }

    pub fn clear_i_flag(&mut self) {
        self.flags = self.flags & 0b1111_1011;
    }

    pub fn is_i_set(&mut self) -> bool {
        return self.flags & 0b0000_0100 == 0b0000_0100
    }

    // Flag bit 3 - Decimal
    pub fn set_d_flag(&mut self) {
        self.flags = self.flags | 0b0000_1000;
    }

    pub fn clear_d_flag(&mut self) {
        self.flags = self.flags & 0b1111_0111;
    }

    pub fn is_d_set(&mut self) -> bool {
        return self.flags & 0b0000_0000 == 0b0000_1000
    }

    // Flag bit 4 - Break
    pub fn set_b_flag(&mut self) {
        self.flags = self.flags | 0b0001_0000;
    }

    pub fn clear_b_flag(&mut self) {
        self.flags = self.flags & 0b1110_1111;
    }

    pub fn is_b_set(&mut self) -> bool {
        return self.flags & 0b0001_0000 == 0b0001_0000
    }

    // Flag bit 5 - Unused

    // Flag bit 6 - Overflow
    pub fn set_v_flag(&mut self) {
        self.flags = self.flags | 0b0100_0000;
    }

    pub fn clear_v_flag(&mut self) {
        self.flags = self.flags & 0b1011_1111;
    }

    pub fn is_v_set(&mut self) -> bool {
        return self.flags & 0b0100_0000 == 0b0100_0000
    }

    // Flag bit 7 - Negative
    // Set when the highest bit of the result is also set
    pub fn set_n_flag(&mut self) {
        self.flags = self.flags | 0b1000_0000;
    }

    pub fn clear_n_flag(&mut self) {
        self.flags = self.flags & 0b0111_1111;
    }

    pub fn is_n_set(&mut self) -> bool {
        return self.flags & 0b1000_0000 == 0b1000_0000
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

    pub fn get_u16_from_u8_pair(&self, low_byte: u8, high_byte: u8) -> u16 {

        // Load the high byte into the address
        let mut high: u16 = high_byte.into();

        // Shift the bytes to the left by 8 bits to make room
        high = high << 8;

        // Load the low byte into the address
        let low: u16 = low_byte.into();

        // Combine the high and low bytes bitwise
        let address: u16 = high | low;

        // Return the result
        return address;
    }

    pub fn get_zero_paged_address(&self, index: u8, operand: u8) -> u16 {

        // First get the absolute address
        let mut address: u16 = self.get_absolute_address(index, operand.into());

        // Zero out the high byte
        address = address & 0b0000_0000_1111_1111;

        // Return the address
        return address;
    }

    pub fn get_absolute_address(&self, index: u8, operand: u16) -> u16 {

        // Turn the u8 into a u16
        let mut address: u16 = index.into();

        // Add the operand to address
        address = address + operand;

        // Return the address
        return address;
    }

    // Reset the cpu to the starting conditions
    // This is done in the "After reset" state as described by the nesdev wiki
    // https://wiki.nesdev.com/w/index.php/CPU_power_up_state#After_reset
    pub fn reset(&mut self) {

        // Set I flag high
        self.set_i_flag();

        // Decrement stack by 3
        self.stack = self.stack - 3;
    }


    pub fn set_nestest_automation(&mut self) {
        // Automation mode is defined on github
        // https://github.com/christopherpow/nes-test-roms/blob/master/other/nestest.txt#L67
        self.program_counter = 0x0000_c000;
    }

    // Execute a clock cycle on the cpu
    pub fn execute_clock_cycle(mut cpu: &mut Cpu, mut memory: &mut Memory) {

        // Fetch
        let opcode: u8 = memory.read(cpu.program_counter.into(), 1)[0];

        // Decode and execute
        Decoder::execute(cpu, memory, opcode);

    }
}
