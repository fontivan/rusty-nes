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

pub struct Opcode0xb6 {}

impl Opcode for Opcode0xb6 {
    fn get_name() -> String {
        return "0xb6".to_string();
    }

    fn execute(mut _cpu: &mut Cpu, mut _memory: &mut Memory, mut _data: Vec<u8>) {
        // Add the contents of index y to the provided operand and load that address from memory
        _cpu.x_index = _memory.read((_data[0] + _cpu.y_index).into(), 1)[0];

        // If the MSB is high then we will need to set N
        if _cpu.x_index & 0b1000_0000 == 0 {
            _cpu.clear_n_flag();
        } else {
            _cpu.set_n_flag();
        }

        // If the value is zero then set Z
        if _cpu.x_index == 0 {
            _cpu.set_z_flag();
        } else {
            _cpu.clear_z_flag();
        }
    }
}
