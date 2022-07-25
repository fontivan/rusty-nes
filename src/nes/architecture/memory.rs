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

use crate::nes::architecture::utils::Utils;

// Structure for Memory
pub struct Memory {
    // The size of the memory pool in bytes
    size: usize,

    // The raw memory stored as a vector of bytes (u8)
    raw_memory: Vec<u8>,
}

// Implementation for Memory
impl Memory {
    // Constructor for Memory
    pub fn new(size: usize) -> Result<Self, usize> {
        // Create a zero'd out array of bytes to act as the raw memory
        let raw_memory: Vec<u8> = vec![0; size];

        // Create the Memory object and return it
        Ok(Memory { size, raw_memory })
    }

    // Helper method used to validate that the inputs to the read/write functions are valid
    fn assert_valid_inputs(&mut self, offset: usize, data_length: usize) {
        // If the offset is greater then the size of the memory we have a problem
        // This must be strictly less then, as if the offset was equal to the size then no matter
        // the length it would be a problem
        assert!(offset < self.size);

        // If the sum of the offset and the length is greater then the size of the memory we have a problem
        // This must be lesser then or equal to, as we could potentially be reading right up to the last byte
        assert!(offset + data_length <= self.size);
    }

    // Get the size of the memory
    pub fn get_size(&mut self) -> usize {
        self.size
    }

    // Read a set number of bytes from memory at a provided offset
    pub fn read(&mut self, offset: usize, data_length: usize) -> Vec<u8> {
        // Assert that the input offset and length were valid
        self.assert_valid_inputs(offset, data_length);

        // Each element in the array is one byte
        // Therefore we want to return a number of elements, where the number is the number of bytes
        self.raw_memory[offset..(offset + data_length)].to_vec()
    }

    // Write a set number of bytes from memory at a provided offset
    pub fn write(&mut self, offset: usize, data: Vec<u8>) {
        // Couunt the number of bytes we need to write
        let data_length: usize = data.len();

        // Assert that the input offset and length were valid
        self.assert_valid_inputs(offset, data_length);

        // For each input byte, overwrite the corresponding byte in the memory pool
        self.raw_memory[offset..(data_length + offset)].copy_from_slice(&data[..data_length]);
    }

    pub fn load_rom_from_cartridge(&mut self, rom_content: String) {
        // This should be compliant with the iNES and NES2.0 file format specifications
        // iNES: https://wiki.nesdev.com/w/index.php/INES
        // NES2.0: https://wiki.nesdev.com/w/index.php/NES_2.0

        // The header is the first 16 bytes of the rom content
        let header: String = rom_content[0..16].to_string();
        let mut raw_header: Vec<u8> = [].to_vec();

        for char in header.chars() {
            raw_header.push(char as u8);
        }

        // The first three bytes should be 'N' (0x4E), 'E' (0x45), and 'S' (0x53), followed by EOF (0x1A)
        // This is derived from https://wiki.nesdev.com/w/index.php/NES_2.0#Identification
        assert!(raw_header[0] == 0x4E);
        assert!(raw_header[1] == 0x45);
        assert!(raw_header[2] == 0x53);
        assert!(raw_header[3] == 0x1A);

        let mut nes2: bool = false;
        // The nes 2.0 specification is that from the 7th byte of the header, that bit 2 is clear and bit 3 is set
        let id_byte: u8 = raw_header[7];
        if id_byte & 0b0000_0100 == 0b0000_0000 && id_byte & 0b0000_1000 == 0b0000_1000 {
            nes2 = true;
        };

        //TODO: Load the rom properly. This is a temporary hack sourced from Stack Overflow
        // https://stackoverflow.com/questions/46998060/how-do-i-load-nestest-rom/47036424#47036424
        let rom_data: String = (&rom_content[16..]).to_string();
        let mut raw_rom_data: Vec<u8> = [].to_vec();
        for char in rom_data.chars() {
            raw_rom_data.push(char as u8);
        }
        self.write(0x8000, raw_rom_data.clone());
        self.write(0xC000, raw_rom_data.clone());

        if nes2 {
            print!("NES2.0 format detected.")
        } else {
            print!("iNES format detected.")
        }
    }

    pub fn get_instruction_argument(&mut self, offset: u16, size: usize) -> u16 {
        // We expect this to be between 1 and 4 bytes
        assert!(size >= 1);
        assert!(size <= 4);

        // First get the program counter and read the data stored after the instruction
        let data: Vec<u8> = self.read(offset.into(), size);

        match size {
            1 => {
                let mut result: u16 = data[0].into();
                result >>= 4;
                result &= 0b0000_1111;
                return result.into();
            }
            2 => {
                let result: u16 = data[0].into();
                return result;
            }
            3 => {
                let mut low: u16 = data[1].into();
                low >>= 4;
                low &= 0b0000_1111;
                let mut high: u16 = data[0].into();
                high <<= 4;
                return high | low;
            }
            4 => {
                let low: u16 = data[1].into();
                let mut high: u16 = data[0].into();
                high <<= 8;
                return high | low;
            }
            _ => {
                panic!("This shouldn't be possible!");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // The number of bytes to be used by the test memory
    fn get_test_memory_size() -> usize {
        8
    }

    // Helper function for the tests to let them grab a Memory instance for testing
    fn get_test_memory() -> Memory {
        // The number of bytes of memory to use for the test
        let memory_result: Result<Memory, usize> = Memory::new(get_test_memory_size());
        match memory_result {
            Ok(result) => result,
            Err(_) => {
                panic!("Unable to initialize memory");
            }
        }
    }

    #[test]
    fn can_create_memory() {
        // Fetch a test instance of memory
        let mut memory: Memory = get_test_memory();

        // Assert that the reported size of the memory is the size we were expecting
        assert_eq!(memory.get_size(), get_test_memory_size());
    }

    #[test]
    fn memory_is_initially_zeroed() {
        // Fetch a test instance of memory
        let mut memory: Memory = get_test_memory();

        // Assert that all the memory we initialized was zeroed
        let actual_memory: Vec<u8> = memory.read(0, get_test_memory_size());
        for i in 0..get_test_memory_size() {
            assert_eq!(actual_memory[i], 0);
        }
    }

    #[test]
    fn can_write_to_memory() {
        // This test requires at least 5 bytes in memory
        assert!(get_test_memory_size() > 5);

        // Fetch a test instance of memory
        let mut memory: Memory = get_test_memory();

        // Set up our expected data
        let expected_data: Vec<u8> = [1, 2, 3, 4, 5].to_vec();
        let expected_length: usize = expected_data.len();

        // We need to clone the original data to prevent a use after move compiler error
        let clone: Vec<u8> = expected_data.clone();

        // Write the data to memory
        memory.write(0, expected_data);

        // Set up our actual data
        let actual_data: Vec<u8> = memory.read(0, 5);
        let actual_length: usize = actual_data.len();

        // Assert that the length of the actual vs expected data is the same
        assert_eq!(expected_length, actual_length);

        // Assert that the data we read from the memory was the same as the data we wrote
        assert_eq!(clone, actual_data);
    }

    #[test]
    fn write_sparsely_populated_data() {
        // This test requires at least 3 bytes in memory
        assert!(get_test_memory_size() > 3);

        // Fetch a test instance of memory
        let mut memory: Memory = get_test_memory();

        // We will write two separate bytes sparsely to the memory
        let data1: Vec<u8> = [1].to_vec();
        let data2: Vec<u8> = [2].to_vec();

        // We will write to offset 0 and 2 below, so we expect our data to sandwich an empty at index 1.
        let expected_data: Vec<u8> = [1, 0, 2].to_vec();

        // Write the two bytes to memory
        memory.write(0, data1);
        memory.write(2, data2);

        // Read the 3 bytes sequence
        let actual_data: Vec<u8> = memory.read(0, 3);

        // Assert that the expected and actual data are the same
        assert_eq!(expected_data, actual_data);
    }

    #[test]
    #[should_panic]
    fn read_memory_out_of_bounds() {
        // Fetch a test instance of memory
        let mut memory: Memory = get_test_memory();

        // Intentionally read from memory out of bounds. This should cause a panic.
        let _panic: Vec<u8> = memory.read(get_test_memory_size() + 1, 1);
    }

    #[test]
    #[should_panic]
    fn write_memory_out_of_bounds() {
        // This test requires at least 3 bytes in memory
        assert!(get_test_memory_size() > 5);

        // Fetch a test instance of memory
        let mut memory: Memory = get_test_memory();

        // Prepare some test data
        let expected_data: Vec<u8> = [1, 2, 3].to_vec();

        // Intentionally write to memory out of bounds. This should cause a panic.
        memory.write(get_test_memory_size() + 1, expected_data);
    }

    #[test]
    #[should_panic]
    fn get_instruct_argument_invalid_size_too_small() {
        // Fetch a test instance of memory
        let mut memory: Memory = get_test_memory();

        // Test data is for an arithmetic shift left operator using absolute addressing mode
        let data: Vec<u8> = [0x0, 0xE, 0x4, 0x4, 0x0, 0x0].to_vec();

        // Write test data to memory
        memory.write(0, data);

        // Attempt to fetch an argument of size 0
        let result: u16 = memory.get_instruction_argument(2, 0);
    }

    #[test]
    #[should_panic]
    fn get_instruct_argument_invalid_size_too_large() {
        // Fetch a test instance of memory
        let mut memory: Memory = get_test_memory();

        // Test data is for an arithmetic shift left operator using absolute addressing mode
        let data: Vec<u8> = [0x0, 0xE, 0x4, 0x4, 0x0, 0x0].to_vec();

        // Write test data to memory
        memory.write(0, data);

        // Attempt to fetch an argument of size 5
        let result: u16 = memory.get_instruction_argument(2, 5);
    }

    #[test]
    fn get_one_byte_instruction_argument() {
        // Fetch a test instance of memory
        let mut memory: Memory = get_test_memory();

        // Test data is for an arithmetic shift left operator using absolute addressing mode
        let data: Vec<u8> = [0x0E, 0x44, 0x00].to_vec();

        // Write test data to memory
        memory.write(0, data);

        // Fetch the instruction argument
        let result: u16 = memory.get_instruction_argument(1, 1);

        // Assert result is as expected
        assert_eq!(result, 0x4);
    }

    #[test]
    fn get_two_byte_instruction_argument() {
        // Fetch a test instance of memory
        let mut memory: Memory = get_test_memory();

        // Test data is for an arithmetic shift left operator using absolute addressing mode
        let data: Vec<u8> = [0x0E, 0x44, 0x00].to_vec();

        // Write test data to memory
        memory.write(0, data);

        // Fetch the instruction argument
        let result: u16 = memory.get_instruction_argument(1, 2);

        // Assert result is as expected
        assert_eq!(result, 0x44);
    }

    #[test]
    fn get_three_byte_instruction_argument() {
        // Fetch a test instance of memory
        let mut memory: Memory = get_test_memory();

        // Test data is for an arithmetic shift left operator using absolute addressing mode
        let data: Vec<u8> = [0x0E, 0x44, 0x00].to_vec();

        // Write test data to memory
        memory.write(0, data);

        // Fetch the instruction argument
        let result: u16 = memory.get_instruction_argument(1, 3);

        // Assert result is as expected
        assert_eq!(result, 0x440);
    }

    #[test]
    fn get_four_byte_instruction_argument() {
        // Fetch a test instance of memory
        let mut memory: Memory = get_test_memory();

        // Test data is for an arithmetic shift left operator using absolute addressing mode
        let data: Vec<u8> = [0x0E, 0x44, 0x00].to_vec();

        // Write test data to memory
        memory.write(0, data);

        // Fetch the instruction argument
        let result: u16 = memory.get_instruction_argument(1, 4);

        // Assert result is as expected
        assert_eq!(result, 0x4400);
    }
}
