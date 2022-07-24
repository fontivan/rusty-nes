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

use crate::nes::architecture::decoder::Decoder;
use crate::nes::architecture::memory::Memory;
use crate::nes::architecture::cpu::Cpu;

pub struct Utils;

impl Utils{

    pub fn get_instruction_argument(cpu: Cpu, mut memory: Memory, size: usize) -> u32 {

        // First get the program counter and read the data stored after the instruction
        let mut address: usize = cpu.program_counter.into();
        address = address + 2;
        let data: Vec<u8> = memory.read(address, size);

        // We will always return a u32 but it needs to be constructed based on the raw data
        if data.len() == 1{
            return Utils::get_u16_from_u8_pair(data[0], 0).into();
        } else if data.len() == 2 {
            return Utils::get_u16_from_u8_pair(data[1], data[0]).into();
        } else if data.len() == 3 {
            return Utils::get_u32_from_u16_pair( Utils::get_u16_from_u8_pair(data[1], data[0]),  Utils::get_u16_from_u8_pair(data[2], 0))
        } else {
            return Utils::get_u32_from_u16_pair( Utils::get_u16_from_u8_pair(data[1], data[0]),  Utils::get_u16_from_u8_pair(data[3], data[2]))
        }
    }

    pub fn get_u32_from_u16_pair(low_bytes: u16, high_bytes: u16) -> u32 {

        // Load the high bytes into the address
        let mut high: u32 = high_bytes.into();

        // Shift the bytes to the left by 16 bytes to make room
        high = high << 16;

        // Load the low bytes into the address
        let low: u32 = low_bytes.into();

        // Combine the high and low bytes bitwise
        let address: u32 = high | low;

        // Return the result
        return address;
    }

    pub fn get_u16_from_u8_pair(low_byte: u8, high_byte: u8) -> u16 {

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

    pub fn get_zero_paged_address(index: u8, operand: u8) -> u16 {

        // First get the absolute address
        let mut address: u16 = Utils::get_absolute_address(index, operand.into());

        // Zero out the high byte
        address = address & 0b0000_0000_1111_1111;

        // Return the address
        return address;
    }

    pub fn get_absolute_address(index: u8, operand: u16) -> u16 {

        // Turn the u8 into a u16
        let mut address: u16 = index.into();

        // Add the operand to address
        address = address + operand;

        // Return the address
        return address;
    }

}