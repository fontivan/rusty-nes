////////////////////////////////////////////////////////////////////////////////////////////////////
// MIT License
//
// Copyright (c) 2022 fontivan
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

pub struct Utils;

use crate::nes::architecture::cpu::Cpu;
use crate::nes::architecture::cpu::Register;
use crate::nes::architecture::memory::Memory;
use std::convert::TryInto;

impl Utils {
    pub fn get_u32_from_u16_pair(high_bytes: u16, low_bytes: u16) -> u32 {
        // Load the high bytes into the address
        let mut high: u32 = high_bytes.into();

        // Shift the bytes to the left by 16 bytes to make room
        high <<= 16;

        // Load the low bytes into the address
        let low: u32 = low_bytes.into();

        // Combine the high and low bytes bitwise
        let address: u32 = high | low;

        // Return the result
        address
    }

    pub fn get_u16_from_u8_pair(high_byte: u8, low_byte: u8) -> u16 {
        // Load the high byte into the address
        let mut high: u16 = high_byte.into();

        // Shift the bytes to the left by 8 bits to make room
        high <<= 8;

        // Load the low byte into the address
        let low: u16 = low_byte.into();

        // Combine the high and low bytes bitwise
        let address: u16 = high | low;

        // Return the result
        address
    }

    pub fn get_u8_pair_from_u16(input: u16) -> (u8, u8) {
        let mut high: u16 = input;
        let mut low: u16 = input;
        low &= 0b0000_0000_1111_1111;
        high >>= 8;
        high &= 0b0000_0000_1111_1111;
        let result = [high, low];
        return (high.try_into().unwrap(), low.try_into().unwrap());
    }

    pub fn get_zero_paged_address(index: u8, operand: u8) -> u16 {
        // First get the absolute address
        let mut address: u16 = Utils::get_absolute_address(index, operand.into());

        // Zero out the high byte
        address &= 0b0000_0000_1111_1111;

        // Return the address
        address
    }

    pub fn get_absolute_address(index: u8, operand: u16) -> u16 {
        // Turn the u8 into a u16
        let mut address: u16 = index.into();

        // Add the operand to address
        address += operand;

        // Return the address
        address
    }

    pub fn stack_pop(mut cpu: &mut Cpu, mut memory: &mut Memory) -> u8 {
        let stack_pointer: u16 = cpu.get_stack_pointer();

        // Read the value from memory
        let value: u8 = memory.read(stack_pointer.into(), 1)[0];

        // Clear the value from the stack
        memory.write(stack_pointer.into(), [0x00].to_vec());

        // Increment the stack pointer
        cpu.register_add(Register::Stack, 1);

        return value;
    }

    pub fn stack_push(mut cpu: &mut Cpu, mut memory: &mut Memory, value: u8) {
        let stack_pointer: u16 = cpu.get_stack_pointer();

        // Write the value into memory
        memory.write(stack_pointer.into(), [value].to_vec());

        // Decrement the stack register
        cpu.register_add(Register::Stack, -1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_u32_pair() {
        let low: u16 = 0x00AB;
        let high: u16 = 0x00CD;
        let actual: u32 = Utils::get_u32_from_u16_pair(high, low);
        assert_eq!(actual, 0x00CD00AB);
    }

    #[test]
    fn test_u16_pair() {
        let low: u8 = 0x0A;
        let high: u8 = 0x0B;
        let actual: u16 = Utils::get_u16_from_u8_pair(high, low);
        assert_eq!(actual, 0x0B0A);
    }

    #[test]
    fn test_u8_pair() {
        let input: u16 = 0xABCD;
        let result = Utils::get_u8_pair_from_u16(input);
        assert_eq!(result.0, 0xAB);
        assert_eq!(result.1, 0xCD);
    }
}
