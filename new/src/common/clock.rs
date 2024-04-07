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

use crate::common::utils::Utils;
use std::{thread, time};
use std::convert::TryInto;

pub struct Clock {
    // The clock rate in Hz
    cycle_period_nanoseconds: u128,
    // The last tick unix epoch in nanoseconds
    last_tick_nanoseconds: u128
}

impl Clock {
    pub fn new(clock_speed_hz: f64) -> Clock {
        let tick_time = 1000000000.0 / clock_speed_hz;
        Clock {
            cycle_period_nanoseconds: tick_time as u128,
            last_tick_nanoseconds: Utils::get_current_time_in_nanoseconds()
        }
    }

    pub fn tick(&mut self) {
        // Check the current time
        let current_time = Utils::get_current_time_in_nanoseconds();

        // Check if we are ahead or behind
        let target_time = self.last_tick_nanoseconds + self.cycle_period_nanoseconds;

        // If current time is ahead of target time then we are lagging behind, return immediately
        if current_time > target_time {
            return;
        }

        // Otherwise we want to sleep until the target time
        let sleep_time_nanoseconds = target_time - current_time;
        thread::sleep(time::Duration::from_nanos(sleep_time_nanoseconds.try_into().unwrap()));
        return
    }
}
