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

use std::fs;
// use std::fs::File;
// use std::io::Read;

pub struct CartridgeSlot {
    file_path: String,
    pub rom_contents: Vec<u8>,
}

impl CartridgeSlot {
    // Constructor for CartidgeSlot
    pub fn new(file_path: String) -> CartridgeSlot {
        // Initialize an empty slot
        let mut cartridge_slot: CartridgeSlot = CartridgeSlot {
            file_path: "".to_string(),
            rom_contents: [].to_vec(),
        };

        // Load the file
        cartridge_slot.load_cartridge(file_path);

        // Return the cartridge slot
        cartridge_slot
    }

    pub fn load_cartridge(&mut self, file_path: String) {
        // If the path is empty then remove the current cartridge
        if file_path.is_empty() {
            self.file_path = "".to_string();
            self.rom_contents = [].to_vec();
            return;
        }

        // Set the path for the cartridge slot
        self.file_path = file_path;

        let contents: Vec<u8> = fs::read(self.file_path.clone()).unwrap();
        self.rom_contents = contents;
    }
}
