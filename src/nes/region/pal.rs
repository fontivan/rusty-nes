////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// MIT License
//
// Copyright (c) 2022 fontivan
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

use crate::nes::region::Region;

pub struct PAL;

impl Region for PAL {
    fn get_name(&self) -> &str {
        return "NTSC";
    }

    fn get_master_clockrate(&self) -> f64 {
        return 26601712.5; //hertz
    }

    fn get_cpu_clockrate(&self) -> f64 {
        return self.get_master_clockrate() / 16.0; //hertz
    }

    fn get_ppu_clockrate(&self) -> f64 {
        return self.get_cpu_clockrate() / 3.0; //hertz
    }

    fn get_scanline_count(&self) -> isize {
        return 239;
    }

    fn get_video_framerate(&self) -> isize {
        return 50;
    }
}
