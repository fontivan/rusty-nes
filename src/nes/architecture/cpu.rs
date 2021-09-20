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

pub struct Cpu {
    pub accumulator: u8,
    pub flags: u8,
    pub program_counter: u16,
    pub stack: u8,
    pub x_index: u8,
    pub y_index: u8
}

impl Cpu{

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
            y_index: 0
        }
    }

    // Reset the cpu to the starting conditions
    // This is done in the "After reset" state as described by the nesdev wiki
    // https://wiki.nesdev.com/w/index.php/CPU_power_up_state#After_reset
    pub fn reset(&mut self){

        // Set I flag high
        self.flags = self.flags & 0b00000100;

        // Decrement stack by 3
        self.stack = self.stack - 3;
    }

    // Execute a clock cycle on the cpu
    pub fn execute_clock_cycle(&mut self){
        self.reset();
    }

}
