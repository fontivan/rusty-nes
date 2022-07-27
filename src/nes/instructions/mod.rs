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

// Instruction set opcodes and legality retrieved from source #1 below.

// Instruction set usage is derived three sources:
//      1. https://www.masswerk.at/6502/6502_instruction_set.html
//      2. http://www.6502.org/tutorials/6502opcodes.html
//      3. https://sites.google.com/site/6502asembly/6502-instruction-set/

use crate::nes::architecture::cpu::Cpu;
use crate::nes::architecture::memory::Memory;

// All legal instructions will need to be implemented.
pub mod legal;

// Illegal instructions will be implemented on an as-needed basis.
// A partial list of games requiring illegal opcodes is available on the nes wiki
// https://wiki.nesdev.com/w/index.php/CPU_unofficial_opcodes#Games_using_unofficial_opcodes
pub mod illegal;

// Test utilities for testing the instructions

pub trait Opcode {
    fn get_name() -> String {
        panic!("Opcode::get_name() must be overwritten.")
    }

    fn execute(mut _cpu: &mut Cpu, mut _memory: &mut Memory) {
        panic!("Opcode::execute() must be overwritten.");
    }
}
