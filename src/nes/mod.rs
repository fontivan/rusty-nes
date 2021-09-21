////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
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
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use crate::nes::architecture::cartridge_slot::CartridgeSlot;
use crate::nes::architecture::cpu::Cpu;
use crate::nes::architecture::memory::Memory;

pub mod architecture;
pub mod controllers;
pub mod instructions;
pub mod mappers;

pub struct Nes {
    cartridge_slot: CartridgeSlot,
    cpu: Cpu,
    // apu: Apu,
    memory: Memory,
    // ppu: Ppu,
    // controllers: Controllers
}

impl Nes {
    // Constructor for Nes
    pub fn new() -> Self {


        // Initialize the memory with 2KB (RAM) + 1MB (ROM)
        let memory_size: usize = 1024*1000*2 + 1024*1000*1000;
        let memory_result: Result<Memory, usize> = Memory::new(memory_size);
        let memory: Memory;
        match memory_result {
            Ok(result) => {
                memory = result;
            }
            Err(_) => {
                panic!("Unable to initialize memory");
            }
        }

        // Initialize the CPU
        let cpu: Cpu = Cpu::new();

        // Initialize the cartridge slot
        let cartridge_slot: CartridgeSlot = CartridgeSlot::new("".to_string());

        // Construct the Nes
        return Nes {
            cartridge_slot,
            cpu,
            memory,
        };
    }

    fn dump_memory(&mut self) {
        let memory_size = self.memory.get_size();
        for i in 0..memory_size {
            self.memory.write(i, [1].to_vec());
        }
        print!("{:?}", self.memory.read(0, memory_size));
    }

    pub fn run (&mut self){
        self.dump_memory();
        self.cpu.execute_clock_cycle();
    }
}
