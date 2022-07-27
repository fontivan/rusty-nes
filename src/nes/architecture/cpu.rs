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

use crate::nes::architecture::decoder::Decoder;
use crate::nes::architecture::memory::Memory;
use std::convert::TryFrom;

pub enum Register {
    Accumulator,
    Flags,
    ProgramCounter,
    Stack,
    XIndex,
    YIndex,
}

pub struct Cpu {
    pub accumulator: u8,
    // Flag usage is derived from the nesdev wiki
    // https://wiki.nesdev.com/w/index.php/Status_flags
    pub flags: u8,
    pub program_counter: u16,
    pub stack: u8,
    pub x_index: u8,
    pub y_index: u8,
}

impl Cpu {
    // Constructor for Cpu
    // This is done in the "Power Up" state as described by the nesdev wiki
    // https://wiki.nesdev.com/w/index.php/CPU_power_up_state#At_power-up
    pub fn new() -> Cpu {
        Cpu {
            accumulator: 0,
            flags: 0,
            program_counter: 0x34,
            stack: 0xFD,
            x_index: 0,
            y_index: 0,
        }
    }

    // Flag bit 0 - Carry
    // Set when the accumulator rolls over from 0xFF to 0x00, or as part of some operations
    pub fn set_c_flag(&mut self) {
        self.flags |= 0b0000_0001;
    }

    pub fn clear_c_flag(&mut self) {
        self.flags &= 0b1111_1110;
    }

    pub fn is_c_set(&mut self) -> bool {
        self.flags & 0b0000_0001 == 0b0000_0001
    }

    // Flag bit 1 - Zero
    // Set when the result of most instructions is 0x00
    pub fn set_z_flag(&mut self) {
        self.flags |= 0b0000_0010;
    }

    pub fn clear_z_flag(&mut self) {
        self.flags &= 0b1111_1101;
    }

    pub fn is_z_set(&mut self) -> bool {
        self.flags & 0b0000_0010 == 0b0000_0010
    }

    // Flag bit 2 - Interrupt
    // Set when various interrupt methods are called
    pub fn set_i_flag(&mut self) {
        self.flags |= 0b0000_0100;
    }

    pub fn clear_i_flag(&mut self) {
        self.flags &= 0b1111_1011;
    }

    pub fn is_i_set(&mut self) -> bool {
        self.flags & 0b0000_0100 == 0b0000_0100
    }

    // Flag bit 3 - Decimal
    pub fn set_d_flag(&mut self) {
        self.flags |= 0b0000_1000;
    }

    pub fn clear_d_flag(&mut self) {
        self.flags &= 0b1111_0111;
    }

    pub fn is_d_set(&mut self) -> bool {
        self.flags & 0b0000_1000 == 0b0000_1000
    }

    // Flag bit 4 - Break
    pub fn set_b_flag(&mut self) {
        self.flags |= 0b0001_0000;
    }

    pub fn clear_b_flag(&mut self) {
        self.flags &= 0b1110_1111;
    }

    pub fn is_b_set(&mut self) -> bool {
        self.flags & 0b0001_0000 == 0b0001_0000
    }

    // Flag bit 5 - Unused

    // Flag bit 6 - Overflow
    pub fn set_v_flag(&mut self) {
        self.flags |= 0b0100_0000;
    }

    pub fn clear_v_flag(&mut self) {
        self.flags &= 0b1011_1111;
    }

    pub fn is_v_set(&mut self) -> bool {
        self.flags & 0b0100_0000 == 0b0100_0000
    }

    // Flag bit 7 - Negative
    // Set when the highest bit of the result is also set
    pub fn set_n_flag(&mut self) {
        self.flags |= 0b1000_0000;
    }

    pub fn clear_n_flag(&mut self) {
        self.flags &= 0b0111_1111;
    }

    pub fn is_n_set(&mut self) -> bool {
        self.flags & 0b1000_0000 == 0b1000_0000
    }

    pub fn register_add(&mut self, register: Register, operand: isize) {
        match register {
            Register::Accumulator => {
                let mut result: isize = self.accumulator.into();
                result = result + operand;
                if result > std::u8::MAX.into() {
                    self.set_v_flag();
                    result &= 0b0000_0000_1111_1111;
                }
                if result < 0 {
                    result += 0xFF
                }
                match u8::try_from(result) {
                    Ok(result) => {
                        self.accumulator = result;
                    }
                    Err(error) => {
                        std::panic::panic_any(error);
                    }
                }
            }
            Register::Flags => {
                panic!("Should not be adding integers to flag register")
            }
            Register::ProgramCounter => {
                let mut result: isize = isize::try_from(self.program_counter).unwrap();
                result = result + operand;
                if result > isize::try_from(std::u16::MAX).unwrap() {
                    self.set_v_flag();
                    result &= 0x00FF;
                }
                if result < 0 {
                    result += 0xFF
                }
                match u16::try_from(result) {
                    Ok(result) => {
                        self.program_counter = result;
                    }
                    Err(error) => {
                        std::panic::panic_any(error);
                    }
                }
            }
            Register::Stack => {
                let mut result: isize = self.stack.into();
                result = result + operand;
                if result > std::u8::MAX.into() {
                    self.set_v_flag();
                    result &= 0b0000_0000_1111_1111;
                }
                if result < 0 {
                    result += 0xFF
                }
                match u8::try_from(result) {
                    Ok(result) => {
                        self.stack = result;
                    }
                    Err(error) => {
                        std::panic::panic_any(error);
                    }
                }
            }
            Register::XIndex => {
                let mut result: isize = self.x_index.into();
                result = result + operand;
                if result > std::u8::MAX.into() {
                    self.set_v_flag();
                    result &= 0b0000_0000_1111_1111;
                }
                if result < 0 {
                    result += 0xFF
                }
                match u8::try_from(result) {
                    Ok(result) => {
                        self.x_index = result;
                    }
                    Err(error) => {
                        std::panic::panic_any(error);
                    }
                }
            }
            Register::YIndex => {
                let mut result: isize = self.y_index.into();
                result = result + operand;
                if result > std::u8::MAX.into() {
                    self.set_v_flag();
                    result &= 0b0000_0000_1111_1111;
                }
                if result < 0 {
                    result += 0xFF
                }
                match u8::try_from(result) {
                    Ok(result) => {
                        self.y_index = result;
                    }
                    Err(error) => {
                        std::panic::panic_any(error);
                    }
                }
            }
        }
    }

    // This function will be called by a large number of instructions to check if the z and n flags should be set
    pub fn check_result_for_zero_and_negative_flags(&mut self, result: u8) {
        // If the last result was 0 then the zero flag must be set
        if result == 0 {
            self.set_z_flag()
        } else {
            self.clear_z_flag()
        }

        // If the highest bit of the last result was 1 then the negative flag must be set
        if result & 0b1000_0000 == 0b1000_0000 {
            self.set_n_flag()
        } else {
            self.clear_n_flag()
        }
    }

    // Reset the cpu to the starting conditions
    // This is done in the "After reset" state as described by the nesdev wiki
    // https://wiki.nesdev.com/w/index.php/CPU_power_up_state#After_reset
    pub fn reset(&mut self) {
        // Set I flag high
        self.set_i_flag();

        // Decrement stack by 3
        self.register_add(Register::Stack, -3);
    }

    pub fn set_nestest_automation(&mut self) {
        // Automation mode is defined on github
        // https://github.com/christopherpow/nes-test-roms/blob/master/other/nestest.txt#L67
        self.program_counter = 0x0c000;
    }

    // Execute a clock cycle on the cpu
    pub fn execute_clock_cycle(cpu: &mut Cpu, memory: &mut Memory) {
        // Fetch
        let data: Vec<u8> = memory.read(cpu.program_counter.into(), 1);
        // let debug: Vec<u8> = memory.read(cpu.program_counter.into(), 1024);
        // print!("{:?}", debug);

        // Increment program counter
        cpu.register_add(Register::ProgramCounter, 1);

        // Decode and execute
        Decoder::execute(cpu, memory, data[0]);
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::nes::architecture::memory::tests::get_test_memory;

    pub fn get_test_cpu() -> Cpu {
        // Get a cpu
        let mut cpu: Cpu = Cpu::new();
        cpu.program_counter = 0x00;
        return cpu;
    }

    #[test]
    fn accumulator_add() {
        let mut cpu: Cpu = get_test_cpu();
        let mut memory: Memory = get_test_memory(1024);

        let data: Vec<u8> = [0xA9, 0x01, 0x69, 0x03].to_vec();

        memory.write(0, data);
        cpu.program_counter = 0x00;

        // Execute two cycles
        assert_eq!(cpu.accumulator, 0x00);
        Cpu::execute_clock_cycle(&mut cpu, &mut memory);
        assert_eq!(cpu.accumulator, 0x01);
        Cpu::execute_clock_cycle(&mut cpu, &mut memory);
        assert_eq!(cpu.accumulator, 0x04);
    }

    #[test]
    fn x_increment() {
        let mut cpu: Cpu = get_test_cpu();
        let mut memory: Memory = get_test_memory(1024);

        let data: Vec<u8> = [0xA2, 0xF0, 0xE8, 0xE8, 0xE8].to_vec();

        memory.write(0, data);
        cpu.program_counter = 0x00;

        // Execute four cycles in total - 1 load, plus 3 increments
        assert_eq!(cpu.x_index, 0x00);
        Cpu::execute_clock_cycle(&mut cpu, &mut memory);
        assert_eq!(cpu.x_index, 0xF0);
        Cpu::execute_clock_cycle(&mut cpu, &mut memory);
        assert_eq!(cpu.x_index, 0xF1);
        Cpu::execute_clock_cycle(&mut cpu, &mut memory);
        assert_eq!(cpu.x_index, 0xF2);
        Cpu::execute_clock_cycle(&mut cpu, &mut memory);
        assert_eq!(cpu.x_index, 0xF3);
    }

    #[test]
    fn y_decrement() {
        let mut cpu: Cpu = get_test_cpu();
        let mut memory: Memory = get_test_memory(1024);

        let data: Vec<u8> = [0xA0, 0xFF, 0x88, 0x88, 0x88, 0x88].to_vec();

        memory.write(0, data);
        cpu.program_counter = 0x00;

        // Execute five cycles in total - one load, plus 4 decrements
        assert_eq!(cpu.y_index, 0x00);
        Cpu::execute_clock_cycle(&mut cpu, &mut memory);
        assert_eq!(cpu.y_index, 0xFF);
        Cpu::execute_clock_cycle(&mut cpu, &mut memory);
        assert_eq!(cpu.y_index, 0xFE);
        Cpu::execute_clock_cycle(&mut cpu, &mut memory);
        assert_eq!(cpu.y_index, 0xFD);
        Cpu::execute_clock_cycle(&mut cpu, &mut memory);
        assert_eq!(cpu.y_index, 0xFC);
        Cpu::execute_clock_cycle(&mut cpu, &mut memory);
        assert_eq!(cpu.y_index, 0xFB);
    }
}
