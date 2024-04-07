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

use std::time::SystemTime;
use std::convert::TryInto;

pub struct Utils;

impl Utils {

    pub fn get_current_time_in_nanoseconds() -> u128 {
        let duration_since_epoch = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
        let timestamp_nanos = duration_since_epoch.as_nanos(); // u128
        return timestamp_nanos
    }

    pub fn get_u32_from_u16_pair(high_bytes: u16, low_bytes: u16) -> u32 {
        // Load the high bytes into the address
        let mut high: u32 = high_bytes.into();

        // Shift the bytes to the left by 16 bytes to make room
        high <<= 16;

        // Load the low bytes into the address
        let low: u32 = low_bytes.into();

        // Combine the high and low bytes bitwise
        let address: u32 = high | low;

        // Return the result
        address
    }

    pub fn get_u16_from_u8_pair(high_byte: u8, low_byte: u8) -> u16 {
        // Load the high byte into the address
        let mut high: u16 = high_byte.into();

        // Shift the bytes to the left by 8 bits to make room
        high <<= 8;

        // Load the low byte into the address
        let low: u16 = low_byte.into();

        // Combine the high and low bytes bitwise
        let address: u16 = high | low;

        // Return the result
        address
    }

    pub fn get_u8_pair_from_u16(input: u16) -> (u8, u8) {
        let mut high: u16 = input;
        let mut low: u16 = input;
        low &= 0b0000_0000_1111_1111;
        high >>= 8;
        high &= 0b0000_0000_1111_1111;
        return (high.try_into().unwrap(), low.try_into().unwrap());
    }

    pub fn get_twos_complement_magnitude(input: usize, size: usize) -> usize {
        // We will need a mutable number for this
        let mut result: usize = input;

        // Subtract one
        result = result - 1;

        // Invert all the bits
        result = !result;

        // Create a mask to truncate the result
        let mut mask = 0;

        // Determine how many bits of the mask need to be set
        match size {
            8 => {
                //
                mask = u8::MAX.try_into().unwrap();
            }
            16 => {
                //
                mask = u16::MAX.try_into().unwrap();
            }
            32 => {
                //
                mask = u32::MAX.try_into().unwrap();
            }
            _ => {
                panic!("Invalid size, expected one of (8, 16, 32).")
            }
        }

        // Return bitwise and of result and mask
        return result & mask;
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_u32_pair() {
        let low: u16 = 0x00AB;
        let high: u16 = 0x00CD;
        let actual: u32 = Utils::get_u32_from_u16_pair(high, low);
        assert_eq!(actual, 0x00CD00AB);
    }

    #[test]
    fn test_u16_pair() {
        let low: u8 = 0x0A;
        let high: u8 = 0x0B;
        let actual: u16 = Utils::get_u16_from_u8_pair(high, low);
        assert_eq!(actual, 0x0B0A);
    }

    #[test]
    fn test_u8_pair() {
        let input: u16 = 0xABCD;
        let result = Utils::get_u8_pair_from_u16(input);
        assert_eq!(result.0, 0xAB);
        assert_eq!(result.1, 0xCD);
    }

    #[test]
    fn test_twos_complement() {
        let inputs: Vec<usize> = [
            0b1000_0000,
            0b1000_0001,
            0b1000_0010,
            0b1111_1110,
            0b1111_1111,
        ]
        .to_vec();
        let expected: Vec<usize> = [128, 127, 126, 2, 1].to_vec();
        for i in 0..5 {
            assert_eq!(
                Utils::get_twos_complement_magnitude(inputs[i], 8),
                expected[i]
            );
        }
    }
}
