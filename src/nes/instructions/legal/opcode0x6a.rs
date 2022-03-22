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
use crate::nes::instructions::Opcode;

pub struct Opcode0x6a {}

impl Opcode for Opcode0x6a {
    fn get_name() -> String {
        return "0x6a".to_string();
    }

    fn execute(mut _cpu: &mut Cpu, mut _memory: &mut Memory) {
        // Rotate right on the accumulator register

        // Get the value from the accumulator
        let value = _cpu.accumulator;

        // Get the high bit from the value
        let low_bit = value & 0b0000_0001;

        // Shift the value
        value = value >> 1;

        // Set the highest bit to be the value from the carry flag
        if _cpu.carry == 1 {
            value = value | 0b1000_0000;
        }

        // Save the value back to the accumulator
        _cpu.accumulator = value;

        // Save the high bit into the carry
        _cpu.carry = low_bit;

        // Conditionally set the zero flag
        if value == 0 {
            _cpu.set_z_flag();
        } else {
            _cpu.clear_z_flag();
        }
        
        // Conditionally set the negative flag
        if value & 0b1000_0000 == 0b1000_0000 {
            _cpu.set_n_flag();
        } else {
            _cpu.clear_n_flag();
        }    
    }
}
