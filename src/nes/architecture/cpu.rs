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
    accumulator: u8,
    flag: u8,
    program_counter: u16,
    stack: u8,
    x_index: u8,
    y_index: u8
}

impl Cpu{

    // Constructor for Cpu
    pub fn new() -> Cpu {
        return Cpu {
            accumulator: 0,
            flag: 0,
            program_counter: 0,
            stack: 0,
            x_index: 0,
            y_index: 0
        }
    }

    // Reset the cpu to the starting conditions
    pub fn reset(&mut self){
        self.accumulator = 0;
        self.flag = 0;
        self.program_counter = 0;
        self.stack = 0;
        self.x_index = 0;
        self.y_index = 0;
    }

    // Execute a clock cycle on the cpu
    pub fn execute_clock_cycle(&mut self){
        self.reset();
    }

}