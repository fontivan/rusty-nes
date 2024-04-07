////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// MIT License
//
// Copyright (c) 2021-2024 fontivan
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

mod common {
    pub mod clock;
    pub mod memory;
    pub mod utils;
}

mod models {
    pub mod mos6502;
}

use crate::models::mos6502::Mos6502;
use std::fs;

fn main() {
    nes();
}

fn nes() {
    let mut mos6502 = Mos6502::new(
        // Memory size
        1024 * 1000 * 2 + 1024 * 1000 * 1000,
        // Clock speed
        236250000.0 / 11.0,
    );

    // Load nestest rom
    let nestest_rom: Vec<u8> = fs::read("target/nestest.nes").unwrap();
    write_nes_rom_to_memory(&mut mos6502, nestest_rom);

    // Automation mode is defined on github
    // https://github.com/christopherpow/nes-test-roms/blob/master/other/nestest.txt#L67
    let mut pc_data: Vec<u8> = Vec::new();
    // Set program counter to 0xc000
    mos6502.program_counter = 0xc000;

    // Start the system
    mos6502.run();
}

fn write_nes_rom_to_memory(system: &mut Mos6502, rom_content: Vec<u8>) {
    // This should be compliant with the iNES and NES2.0 file format specifications
    // iNES: https://wiki.nesdev.com/w/index.php/INES
    // NES2.0: https://wiki.nesdev.com/w/index.php/NES_2.0

    // The header is the first 16 bytes of the rom content

    // The first three bytes should be 'N' (0x4E), 'E' (0x45), and 'S' (0x53), followed by EOF (0x1A)
    // This is derived from https://wiki.nesdev.com/w/index.php/NES_2.0#Identification
    assert!(rom_content[0] == 0x4E);
    assert!(rom_content[1] == 0x45);
    assert!(rom_content[2] == 0x53);
    assert!(rom_content[3] == 0x1A);

    let mut nes2: bool = false;
    // The nes 2.0 specification is that from the 7th byte of the header, that bit 2 is clear and bit 3 is set
    let id_byte: u8 = rom_content[7];
    if id_byte & 0b0000_0100 == 0b0000_0000 && id_byte & 0b0000_1000 == 0b0000_1000 {
        nes2 = true;
    };

    //TODO: Load the rom properly. This is a temporary hack sourced from Stack Overflow
    // https://stackoverflow.com/questions/46998060/how-do-i-load-nestest-rom/47036424#47036424
    // Skip copying the header as it is not expected to be in memory
    let mut data: Vec<u8> = rom_content.clone();
    data.drain(0..16);
    system.memory.write(0x8000, data.clone());
    system.memory.write(0xC000, data.clone());

    if nes2 {
        println!("NES2.0 format detected.")
    } else {
        println!("iNES format detected.")
    }
}
