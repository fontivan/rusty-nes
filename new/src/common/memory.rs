////////////////////////////////////////////////////////////////////////////////////////////////////
// MIT License
//
// Copyright (c) 2024 fontivan
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

    // Set a single bit in memory
    pub fn set_bit(&mut self, address_offset: usize, bitmask: u8) {
        // Inputs will be validated in read fn
        let byte = self.read(address_offset, 1)[0];

        // Bitwise OR with the bitmask to force flag the bit
        let masked = byte | bitmask;

        // Create a new vec for the data
        let mut data = Vec::new();
        data.push(masked);

        // Write the data back
        self.write(address_offset, data);
    }

    // Clear a single bit in memory
    pub fn clear_bit(&mut self, address_offset: usize, bitmask: u8) {
        // Inputs will be validated in read fn
        let byte = self.read(address_offset, 1)[0];

        // Bitwise AND with the inverted bitmask to isolate the bit in question as a 0
        let masked = byte & !bitmask;

        // Create a new vec for the data
        let mut data = Vec::new();
        data.push(masked);

        // Write the data back
        self.write(address_offset, data);
    }

    // Get a single bit
    pub fn get_bit(&mut self, address_offset: usize, bitmask: u8) -> bool {
        // Inputs will be validated in read fn
        let byte = self.read(address_offset, 1)[0];

        // Bitwise AND with the bitmask to isolate the bit in question
        let masked = byte & bitmask;

        // If any bits are set then the result will be > 0
        return masked != 0;
    }

    // Increment byte at specific address
    pub fn increment_data_at_address(&mut self, address: usize) {
        let mut data = self.read(address, 1)[0];
        data += 1;

        let mut vec = Vec::new();
        vec.push(data);

        self.write(address, vec)
    }

    // Decrement byte at specific address
    pub fn decrement_data_at_address(&mut self, address: usize) {
        let mut data = self.read(address, 1)[0];
        data -= 1;

        let mut vec = Vec::new();
        vec.push(data);

        self.write(address, vec)
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    // Helper function for the tests to let them grab a Memory instance for testing
    pub fn get_test_memory(memory_size: usize) -> Memory {
        // The number of bytes of memory to use for the test
        let memory_result: Result<Memory, usize> = Memory::new(memory_size);
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
        let mut memory: Memory = get_test_memory(8);

        // Assert that the reported size of the memory is the size we were expecting
        assert_eq!(memory.get_size(), 8);
    }

    #[test]
    fn memory_is_initially_zeroed() {
        // Fetch a test instance of memory
        let mut memory: Memory = get_test_memory(8);

        // Assert that all the memory we initialized was zeroed
        let actual_memory: Vec<u8> = memory.read(0, 8);
        for i in 0..8 {
            assert_eq!(actual_memory[i], 0);
        }
    }

    #[test]
    fn can_write_to_memory() {
        // Fetch a test instance of memory
        let mut memory: Memory = get_test_memory(8);

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
        // Fetch a test instance of memory
        let mut memory: Memory = get_test_memory(8);

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
        let mut memory: Memory = get_test_memory(8);

        // Intentionally read from memory out of bounds. This should cause a panic.
        let _panic: Vec<u8> = memory.read(9, 1);
    }

    #[test]
    #[should_panic]
    fn write_memory_out_of_bounds() {
        // Fetch a test instance of memory
        let mut memory: Memory = get_test_memory(8);

        // Prepare some test data
        let expected_data: Vec<u8> = [1, 2, 3].to_vec();

        // Intentionally write to memory out of bounds. This should cause a panic.
        memory.write(9, expected_data);
    }

    #[test]
    fn get_bit_from_memory() {
        // Fetch a test instance of memory
        let mut memory: Memory = get_test_memory(8);

        // Prepare some test data
        let expected_data: Vec<u8> = [0b1010_0101].to_vec();

        // Write the data to memory
        memory.write(0, expected_data);

        // Verify each bit is as expected
        assert_eq!(memory.get_bit(0, 0b1000_0000), true);
        assert_eq!(memory.get_bit(0, 0b0100_0000), false);
        assert_eq!(memory.get_bit(0, 0b0010_0000), true);
        assert_eq!(memory.get_bit(0, 0b0001_0000), false);
        assert_eq!(memory.get_bit(0, 0b0000_1000), false);
        assert_eq!(memory.get_bit(0, 0b0000_0100), true);
        assert_eq!(memory.get_bit(0, 0b0000_0010), false);
        assert_eq!(memory.get_bit(0, 0b0000_0001), true);
    }

    #[test]
    fn set_bit_from_memory() {
        // Fetch a test instance of memory
        let mut memory: Memory = get_test_memory(8);

        // Prepare some test data
        let expected_data: Vec<u8> = [191].to_vec();

        // Prepare a bitmask
        let bitmask = 0b0100_0000;

        // Write the data to memory
        memory.write(0, expected_data);

        // Verify bit is currently low
        assert!(!memory.get_bit(0, bitmask));

        // Clear a single bit
        memory.set_bit(0, bitmask);

        // Verify the result is as expected
        assert_eq!(memory.read(0, 1)[0], 255);    
    }

    #[test]
    fn clear_bit_from_memory() {
        // Fetch a test instance of memory
        let mut memory: Memory = get_test_memory(8);

        // Prepare some test data
        let expected_data: Vec<u8> = [255].to_vec();

        // Prepare a bitmask
        let bitmask = 0b0100_0000;

        // Write the data to memory
        memory.write(0, expected_data);

        // Verify bit is currently high
        assert!(memory.get_bit(0, bitmask));

        // Clear a single bit
        memory.clear_bit(0, bitmask);

        // Verify the result is as expected
        assert_eq!(memory.read(0, 1)[0], 191);
    }

    #[test]
    fn increment_data_at_address() {
        // Fetch a test instance of memory
        let mut memory: Memory = get_test_memory(8);

        // Prepare some test data
        let expected_data: Vec<u8> = [128].to_vec();

        // Write the data to memory
        memory.write(0, expected_data);

        // Increment data at address
        memory.increment_data_at_address(0);
    
        // Verify result
        assert_eq!(memory.read(0, 1)[0], 129);
    }

    #[test]
    fn decrement_data_at_address() {
        // Fetch a test instance of memory
        let mut memory: Memory = get_test_memory(8);

        // Prepare some test data
        let expected_data: Vec<u8> = [128].to_vec();

        // Write the data to memory
        memory.write(0, expected_data);

        // Increment data at address
        memory.decrement_data_at_address(0);
    
        // Verify result
        assert_eq!(memory.read(0, 1)[0], 127);
    }
}
