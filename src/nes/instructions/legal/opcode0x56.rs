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

use crate::nes::architecture::cpu::Cpu;
use crate::nes::architecture::memory::Memory;
use crate::nes::architecture::utils::Utils;
use crate::nes::instructions::Opcode;

pub struct Opcode0x56 {}

impl Opcode for Opcode0x56 {
    fn get_name() -> String {
        return "0x56".to_string();
    }

    fn execute(mut _cpu: &mut Cpu, mut _memory: &mut Memory) {
        // Get the operand data from the memory
        let low_byte: u8 = _memory.read((_cpu.program_counter + 1).into(), 1)[0];

        // Get the address
        let address: usize = Utils::get_zero_paged_address(_cpu.x_index, low_byte).into();

        // Fetch the data from memory using the x index as an offset
        let mut data: u8 = _memory.read(address, 1)[0];

        // Fetch the rightmost bit
        let carry: u8 = data & 0b0000_0001;

        // Rotate the bits right by 1 bit
        data = data >> 1;

        // Set leftmost bit to 0
        data = data & 0b0111_1111;

        // Write the data back to memory
        _memory.write(address, [data].to_vec());

        // If data is now zero, then set the zero flag high
        if data == 0 {
            _cpu.set_z_flag();
        } else {
            _cpu.clear_z_flag();
        }

        // Set carry flag to the value of the rightmost bit
        if carry == 0 {
            _cpu.clear_c_flag();
        } else {
            _cpu.set_c_flag();
        }

        // Shift right inserts 1 into bit 7, so N will always be cleared
        _cpu.clear_n_flag();
    }
}
