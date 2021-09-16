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
        return Ok(
            Memory {
                size,
                raw_memory,
            }
        );
    }

    // Helper method used to validate that the inputs to the read/write functions are valid
    fn assert_valid_inputs(&mut self, offset: usize, data_length: usize) {

        // If the offset is greater then the size of the memory we have a problem
        // This must be strictly less then, as if the offset was equal to the size then no matter
        // the length it would be a problem
        assert_eq!(offset < self.size, true);

        // If the sum of the offset and the length is greater then the size of the memory we have a problem
        // This must be lesser then or equal to, as we could potentially be reading right up to the last byte
        assert_eq!(offset + data_length <= self.size, true);
    }

    // Get the size of the memory
    pub fn get_size(&mut self) -> usize {
        return self.size;
    }

    // Read a set number of bytes from memory at a provided offset
    pub fn read(&mut self, offset: usize, data_length: usize) -> Vec<u8> {

        // Assert that the input offset and length were valid
        self.assert_valid_inputs(offset, data_length);

        // Each element in the array is one byte
        // Therefore we want to return a number of elements, where the number is the number of bytes
        return self.raw_memory[offset..(offset + data_length)].to_vec();
    }

    // Write a set number of bytes from memory at a provided offset
    pub fn write(&mut self, offset: usize, data: Vec<u8>) {

        // Couunt the number of bytes we need to write
        let data_length: usize = data.len();

        // Assert that the input offset and length were valid
        self.assert_valid_inputs(offset, data_length);

        // For each input byte, overwrite the corresponding byte in the memory pool
        for i in 0..data_length {
            self.raw_memory[offset + i] = data[i];
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    // The number of bytes to be used by the test memory
    fn get_test_memory_size() -> usize {
        return 32;
    }

    // Helper function for the tests to let them grab a Memory instance for testing
    fn get_test_memory() -> Memory {

        // The number of bytes of memory to use for the test
        let memory_result: Result<Memory, usize> = Memory::new(get_test_memory_size());
        match memory_result {
            Ok(result) => {
                return result;
            }
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
        assert!(memory.get_size() == get_test_memory_size());
    }

    #[test]
    fn memory_is_initially_zeroed() {

        // Fetch a test instance of memory
        let mut memory: Memory = get_test_memory();

        // Assert that all the memory we initialized was zeroed
        let actual_memory: Vec<u8> = memory.read(0, get_test_memory_size());
        for i in 0..get_test_memory_size() {
            assert!(actual_memory[i] == 0);
        }
    }

    #[test]
    fn can_write_to_memory() {

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
        assert!(expected_length == actual_length);

        // Assert that the data we read from the memory was the same as the data we wrote
        assert!(clone == actual_data);
    }
}
