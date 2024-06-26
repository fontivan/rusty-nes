////////////////////////////////////////////////////////////////////////////////////////////////////
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
////////////////////////////////////////////////////////////////////////////////////////////////////

use crate::models::mos6502::instructions::illegal::opcode0x02::Opcode0x02;
use crate::models::mos6502::instructions::illegal::opcode0x03::Opcode0x03;
use crate::models::mos6502::instructions::illegal::opcode0x04::Opcode0x04;
use crate::models::mos6502::instructions::illegal::opcode0x07::Opcode0x07;
use crate::models::mos6502::instructions::illegal::opcode0x0b::Opcode0x0b;
use crate::models::mos6502::instructions::illegal::opcode0x0c::Opcode0x0c;
use crate::models::mos6502::instructions::illegal::opcode0x0f::Opcode0x0f;
use crate::models::mos6502::instructions::illegal::opcode0x12::Opcode0x12;
use crate::models::mos6502::instructions::illegal::opcode0x13::Opcode0x13;
use crate::models::mos6502::instructions::illegal::opcode0x14::Opcode0x14;
use crate::models::mos6502::instructions::illegal::opcode0x17::Opcode0x17;
use crate::models::mos6502::instructions::illegal::opcode0x1a::Opcode0x1a;
use crate::models::mos6502::instructions::illegal::opcode0x1b::Opcode0x1b;
use crate::models::mos6502::instructions::illegal::opcode0x1c::Opcode0x1c;
use crate::models::mos6502::instructions::illegal::opcode0x1f::Opcode0x1f;
use crate::models::mos6502::instructions::illegal::opcode0x22::Opcode0x22;
use crate::models::mos6502::instructions::illegal::opcode0x23::Opcode0x23;
use crate::models::mos6502::instructions::illegal::opcode0x27::Opcode0x27;
use crate::models::mos6502::instructions::illegal::opcode0x2b::Opcode0x2b;
use crate::models::mos6502::instructions::illegal::opcode0x2f::Opcode0x2f;
use crate::models::mos6502::instructions::illegal::opcode0x32::Opcode0x32;
use crate::models::mos6502::instructions::illegal::opcode0x33::Opcode0x33;
use crate::models::mos6502::instructions::illegal::opcode0x34::Opcode0x34;
use crate::models::mos6502::instructions::illegal::opcode0x37::Opcode0x37;
use crate::models::mos6502::instructions::illegal::opcode0x3a::Opcode0x3a;
use crate::models::mos6502::instructions::illegal::opcode0x3b::Opcode0x3b;
use crate::models::mos6502::instructions::illegal::opcode0x3c::Opcode0x3c;
use crate::models::mos6502::instructions::illegal::opcode0x3f::Opcode0x3f;
use crate::models::mos6502::instructions::illegal::opcode0x42::Opcode0x42;
use crate::models::mos6502::instructions::illegal::opcode0x43::Opcode0x43;
use crate::models::mos6502::instructions::illegal::opcode0x44::Opcode0x44;
use crate::models::mos6502::instructions::illegal::opcode0x47::Opcode0x47;
use crate::models::mos6502::instructions::illegal::opcode0x4b::Opcode0x4b;
use crate::models::mos6502::instructions::illegal::opcode0x4f::Opcode0x4f;
use crate::models::mos6502::instructions::illegal::opcode0x52::Opcode0x52;
use crate::models::mos6502::instructions::illegal::opcode0x53::Opcode0x53;
use crate::models::mos6502::instructions::illegal::opcode0x54::Opcode0x54;
use crate::models::mos6502::instructions::illegal::opcode0x57::Opcode0x57;
use crate::models::mos6502::instructions::illegal::opcode0x5a::Opcode0x5a;
use crate::models::mos6502::instructions::illegal::opcode0x5b::Opcode0x5b;
use crate::models::mos6502::instructions::illegal::opcode0x5c::Opcode0x5c;
use crate::models::mos6502::instructions::illegal::opcode0x5f::Opcode0x5f;
use crate::models::mos6502::instructions::illegal::opcode0x62::Opcode0x62;
use crate::models::mos6502::instructions::illegal::opcode0x63::Opcode0x63;
use crate::models::mos6502::instructions::illegal::opcode0x64::Opcode0x64;
use crate::models::mos6502::instructions::illegal::opcode0x67::Opcode0x67;
use crate::models::mos6502::instructions::illegal::opcode0x6b::Opcode0x6b;
use crate::models::mos6502::instructions::illegal::opcode0x6f::Opcode0x6f;
use crate::models::mos6502::instructions::illegal::opcode0x72::Opcode0x72;
use crate::models::mos6502::instructions::illegal::opcode0x73::Opcode0x73;
use crate::models::mos6502::instructions::illegal::opcode0x74::Opcode0x74;
use crate::models::mos6502::instructions::illegal::opcode0x77::Opcode0x77;
use crate::models::mos6502::instructions::illegal::opcode0x7a::Opcode0x7a;
use crate::models::mos6502::instructions::illegal::opcode0x7b::Opcode0x7b;
use crate::models::mos6502::instructions::illegal::opcode0x7c::Opcode0x7c;
use crate::models::mos6502::instructions::illegal::opcode0x7f::Opcode0x7f;
use crate::models::mos6502::instructions::illegal::opcode0x80::Opcode0x80;
use crate::models::mos6502::instructions::illegal::opcode0x82::Opcode0x82;
use crate::models::mos6502::instructions::illegal::opcode0x83::Opcode0x83;
use crate::models::mos6502::instructions::illegal::opcode0x87::Opcode0x87;
use crate::models::mos6502::instructions::illegal::opcode0x89::Opcode0x89;
use crate::models::mos6502::instructions::illegal::opcode0x8b::Opcode0x8b;
use crate::models::mos6502::instructions::illegal::opcode0x8f::Opcode0x8f;
use crate::models::mos6502::instructions::illegal::opcode0x92::Opcode0x92;
use crate::models::mos6502::instructions::illegal::opcode0x93::Opcode0x93;
use crate::models::mos6502::instructions::illegal::opcode0x97::Opcode0x97;
use crate::models::mos6502::instructions::illegal::opcode0x9b::Opcode0x9b;
use crate::models::mos6502::instructions::illegal::opcode0x9c::Opcode0x9c;
use crate::models::mos6502::instructions::illegal::opcode0x9e::Opcode0x9e;
use crate::models::mos6502::instructions::illegal::opcode0x9f::Opcode0x9f;
use crate::models::mos6502::instructions::illegal::opcode0xa3::Opcode0xa3;
use crate::models::mos6502::instructions::illegal::opcode0xa7::Opcode0xa7;
use crate::models::mos6502::instructions::illegal::opcode0xab::Opcode0xab;
use crate::models::mos6502::instructions::illegal::opcode0xaf::Opcode0xaf;
use crate::models::mos6502::instructions::illegal::opcode0xb2::Opcode0xb2;
use crate::models::mos6502::instructions::illegal::opcode0xb3::Opcode0xb3;
use crate::models::mos6502::instructions::illegal::opcode0xb7::Opcode0xb7;
use crate::models::mos6502::instructions::illegal::opcode0xbb::Opcode0xbb;
use crate::models::mos6502::instructions::illegal::opcode0xbf::Opcode0xbf;
use crate::models::mos6502::instructions::illegal::opcode0xc2::Opcode0xc2;
use crate::models::mos6502::instructions::illegal::opcode0xc3::Opcode0xc3;
use crate::models::mos6502::instructions::illegal::opcode0xc7::Opcode0xc7;
use crate::models::mos6502::instructions::illegal::opcode0xcb::Opcode0xcb;
use crate::models::mos6502::instructions::illegal::opcode0xcf::Opcode0xcf;
use crate::models::mos6502::instructions::illegal::opcode0xd2::Opcode0xd2;
use crate::models::mos6502::instructions::illegal::opcode0xd3::Opcode0xd3;
use crate::models::mos6502::instructions::illegal::opcode0xd4::Opcode0xd4;
use crate::models::mos6502::instructions::illegal::opcode0xd7::Opcode0xd7;
use crate::models::mos6502::instructions::illegal::opcode0xda::Opcode0xda;
use crate::models::mos6502::instructions::illegal::opcode0xdb::Opcode0xdb;
use crate::models::mos6502::instructions::illegal::opcode0xdc::Opcode0xdc;
use crate::models::mos6502::instructions::illegal::opcode0xdf::Opcode0xdf;
use crate::models::mos6502::instructions::illegal::opcode0xe2::Opcode0xe2;
use crate::models::mos6502::instructions::illegal::opcode0xe3::Opcode0xe3;
use crate::models::mos6502::instructions::illegal::opcode0xe7::Opcode0xe7;
use crate::models::mos6502::instructions::illegal::opcode0xeb::Opcode0xeb;
use crate::models::mos6502::instructions::illegal::opcode0xef::Opcode0xef;
use crate::models::mos6502::instructions::illegal::opcode0xf2::Opcode0xf2;
use crate::models::mos6502::instructions::illegal::opcode0xf3::Opcode0xf3;
use crate::models::mos6502::instructions::illegal::opcode0xf4::Opcode0xf4;
use crate::models::mos6502::instructions::illegal::opcode0xf7::Opcode0xf7;
use crate::models::mos6502::instructions::illegal::opcode0xfa::Opcode0xfa;
use crate::models::mos6502::instructions::illegal::opcode0xfb::Opcode0xfb;
use crate::models::mos6502::instructions::illegal::opcode0xfc::Opcode0xfc;
use crate::models::mos6502::instructions::illegal::opcode0xff::Opcode0xff;
use crate::models::mos6502::instructions::legal::opcode0x00::Opcode0x00;
use crate::models::mos6502::instructions::legal::opcode0x01::Opcode0x01;
use crate::models::mos6502::instructions::legal::opcode0x05::Opcode0x05;
use crate::models::mos6502::instructions::legal::opcode0x06::Opcode0x06;
use crate::models::mos6502::instructions::legal::opcode0x08::Opcode0x08;
use crate::models::mos6502::instructions::legal::opcode0x09::Opcode0x09;
use crate::models::mos6502::instructions::legal::opcode0x0a::Opcode0x0a;
use crate::models::mos6502::instructions::legal::opcode0x0d::Opcode0x0d;
use crate::models::mos6502::instructions::legal::opcode0x0e::Opcode0x0e;
use crate::models::mos6502::instructions::legal::opcode0x10::Opcode0x10;
use crate::models::mos6502::instructions::legal::opcode0x11::Opcode0x11;
use crate::models::mos6502::instructions::legal::opcode0x15::Opcode0x15;
use crate::models::mos6502::instructions::legal::opcode0x16::Opcode0x16;
use crate::models::mos6502::instructions::legal::opcode0x18::Opcode0x18;
use crate::models::mos6502::instructions::legal::opcode0x19::Opcode0x19;
use crate::models::mos6502::instructions::legal::opcode0x1d::Opcode0x1d;
use crate::models::mos6502::instructions::legal::opcode0x1e::Opcode0x1e;
use crate::models::mos6502::instructions::legal::opcode0x20::Opcode0x20;
use crate::models::mos6502::instructions::legal::opcode0x21::Opcode0x21;
use crate::models::mos6502::instructions::legal::opcode0x24::Opcode0x24;
use crate::models::mos6502::instructions::legal::opcode0x25::Opcode0x25;
use crate::models::mos6502::instructions::legal::opcode0x26::Opcode0x26;
use crate::models::mos6502::instructions::legal::opcode0x28::Opcode0x28;
use crate::models::mos6502::instructions::legal::opcode0x29::Opcode0x29;
use crate::models::mos6502::instructions::legal::opcode0x2a::Opcode0x2a;
use crate::models::mos6502::instructions::legal::opcode0x2c::Opcode0x2c;
use crate::models::mos6502::instructions::legal::opcode0x2d::Opcode0x2d;
use crate::models::mos6502::instructions::legal::opcode0x2e::Opcode0x2e;
use crate::models::mos6502::instructions::legal::opcode0x30::Opcode0x30;
use crate::models::mos6502::instructions::legal::opcode0x31::Opcode0x31;
use crate::models::mos6502::instructions::legal::opcode0x35::Opcode0x35;
use crate::models::mos6502::instructions::legal::opcode0x36::Opcode0x36;
use crate::models::mos6502::instructions::legal::opcode0x38::Opcode0x38;
use crate::models::mos6502::instructions::legal::opcode0x39::Opcode0x39;
use crate::models::mos6502::instructions::legal::opcode0x3d::Opcode0x3d;
use crate::models::mos6502::instructions::legal::opcode0x3e::Opcode0x3e;
use crate::models::mos6502::instructions::legal::opcode0x40::Opcode0x40;
use crate::models::mos6502::instructions::legal::opcode0x41::Opcode0x41;
use crate::models::mos6502::instructions::legal::opcode0x45::Opcode0x45;
use crate::models::mos6502::instructions::legal::opcode0x46::Opcode0x46;
use crate::models::mos6502::instructions::legal::opcode0x48::Opcode0x48;
use crate::models::mos6502::instructions::legal::opcode0x49::Opcode0x49;
use crate::models::mos6502::instructions::legal::opcode0x4a::Opcode0x4a;
use crate::models::mos6502::instructions::legal::opcode0x4c::Opcode0x4c;
use crate::models::mos6502::instructions::legal::opcode0x4d::Opcode0x4d;
use crate::models::mos6502::instructions::legal::opcode0x4e::Opcode0x4e;
use crate::models::mos6502::instructions::legal::opcode0x50::Opcode0x50;
use crate::models::mos6502::instructions::legal::opcode0x51::Opcode0x51;
use crate::models::mos6502::instructions::legal::opcode0x55::Opcode0x55;
use crate::models::mos6502::instructions::legal::opcode0x56::Opcode0x56;
use crate::models::mos6502::instructions::legal::opcode0x58::Opcode0x58;
use crate::models::mos6502::instructions::legal::opcode0x59::Opcode0x59;
use crate::models::mos6502::instructions::legal::opcode0x5d::Opcode0x5d;
use crate::models::mos6502::instructions::legal::opcode0x5e::Opcode0x5e;
use crate::models::mos6502::instructions::legal::opcode0x60::Opcode0x60;
use crate::models::mos6502::instructions::legal::opcode0x61::Opcode0x61;
use crate::models::mos6502::instructions::legal::opcode0x65::Opcode0x65;
use crate::models::mos6502::instructions::legal::opcode0x66::Opcode0x66;
use crate::models::mos6502::instructions::legal::opcode0x68::Opcode0x68;
use crate::models::mos6502::instructions::legal::opcode0x69::Opcode0x69;
use crate::models::mos6502::instructions::legal::opcode0x6a::Opcode0x6a;
use crate::models::mos6502::instructions::legal::opcode0x6c::Opcode0x6c;
use crate::models::mos6502::instructions::legal::opcode0x6d::Opcode0x6d;
use crate::models::mos6502::instructions::legal::opcode0x6e::Opcode0x6e;
use crate::models::mos6502::instructions::legal::opcode0x70::Opcode0x70;
use crate::models::mos6502::instructions::legal::opcode0x71::Opcode0x71;
use crate::models::mos6502::instructions::legal::opcode0x75::Opcode0x75;
use crate::models::mos6502::instructions::legal::opcode0x76::Opcode0x76;
use crate::models::mos6502::instructions::legal::opcode0x78::Opcode0x78;
use crate::models::mos6502::instructions::legal::opcode0x79::Opcode0x79;
use crate::models::mos6502::instructions::legal::opcode0x7d::Opcode0x7d;
use crate::models::mos6502::instructions::legal::opcode0x7e::Opcode0x7e;
use crate::models::mos6502::instructions::legal::opcode0x81::Opcode0x81;
use crate::models::mos6502::instructions::legal::opcode0x84::Opcode0x84;
use crate::models::mos6502::instructions::legal::opcode0x85::Opcode0x85;
use crate::models::mos6502::instructions::legal::opcode0x86::Opcode0x86;
use crate::models::mos6502::instructions::legal::opcode0x88::Opcode0x88;
use crate::models::mos6502::instructions::legal::opcode0x8a::Opcode0x8a;
use crate::models::mos6502::instructions::legal::opcode0x8c::Opcode0x8c;
use crate::models::mos6502::instructions::legal::opcode0x8d::Opcode0x8d;
use crate::models::mos6502::instructions::legal::opcode0x8e::Opcode0x8e;
use crate::models::mos6502::instructions::legal::opcode0x90::Opcode0x90;
use crate::models::mos6502::instructions::legal::opcode0x91::Opcode0x91;
use crate::models::mos6502::instructions::legal::opcode0x94::Opcode0x94;
use crate::models::mos6502::instructions::legal::opcode0x95::Opcode0x95;
use crate::models::mos6502::instructions::legal::opcode0x96::Opcode0x96;
use crate::models::mos6502::instructions::legal::opcode0x98::Opcode0x98;
use crate::models::mos6502::instructions::legal::opcode0x99::Opcode0x99;
use crate::models::mos6502::instructions::legal::opcode0x9a::Opcode0x9a;
use crate::models::mos6502::instructions::legal::opcode0x9d::Opcode0x9d;
use crate::models::mos6502::instructions::legal::opcode0xa0::Opcode0xa0;
use crate::models::mos6502::instructions::legal::opcode0xa1::Opcode0xa1;
use crate::models::mos6502::instructions::legal::opcode0xa2::Opcode0xa2;
use crate::models::mos6502::instructions::legal::opcode0xa4::Opcode0xa4;
use crate::models::mos6502::instructions::legal::opcode0xa5::Opcode0xa5;
use crate::models::mos6502::instructions::legal::opcode0xa6::Opcode0xa6;
use crate::models::mos6502::instructions::legal::opcode0xa8::Opcode0xa8;
use crate::models::mos6502::instructions::legal::opcode0xa9::Opcode0xa9;
use crate::models::mos6502::instructions::legal::opcode0xaa::Opcode0xaa;
use crate::models::mos6502::instructions::legal::opcode0xac::Opcode0xac;
use crate::models::mos6502::instructions::legal::opcode0xad::Opcode0xad;
use crate::models::mos6502::instructions::legal::opcode0xae::Opcode0xae;
use crate::models::mos6502::instructions::legal::opcode0xb0::Opcode0xb0;
use crate::models::mos6502::instructions::legal::opcode0xb1::Opcode0xb1;
use crate::models::mos6502::instructions::legal::opcode0xb4::Opcode0xb4;
use crate::models::mos6502::instructions::legal::opcode0xb5::Opcode0xb5;
use crate::models::mos6502::instructions::legal::opcode0xb6::Opcode0xb6;
use crate::models::mos6502::instructions::legal::opcode0xb8::Opcode0xb8;
use crate::models::mos6502::instructions::legal::opcode0xb9::Opcode0xb9;
use crate::models::mos6502::instructions::legal::opcode0xba::Opcode0xba;
use crate::models::mos6502::instructions::legal::opcode0xbc::Opcode0xbc;
use crate::models::mos6502::instructions::legal::opcode0xbd::Opcode0xbd;
use crate::models::mos6502::instructions::legal::opcode0xbe::Opcode0xbe;
use crate::models::mos6502::instructions::legal::opcode0xc0::Opcode0xc0;
use crate::models::mos6502::instructions::legal::opcode0xc1::Opcode0xc1;
use crate::models::mos6502::instructions::legal::opcode0xc4::Opcode0xc4;
use crate::models::mos6502::instructions::legal::opcode0xc5::Opcode0xc5;
use crate::models::mos6502::instructions::legal::opcode0xc6::Opcode0xc6;
use crate::models::mos6502::instructions::legal::opcode0xc8::Opcode0xc8;
use crate::models::mos6502::instructions::legal::opcode0xc9::Opcode0xc9;
use crate::models::mos6502::instructions::legal::opcode0xca::Opcode0xca;
use crate::models::mos6502::instructions::legal::opcode0xcc::Opcode0xcc;
use crate::models::mos6502::instructions::legal::opcode0xcd::Opcode0xcd;
use crate::models::mos6502::instructions::legal::opcode0xce::Opcode0xce;
use crate::models::mos6502::instructions::legal::opcode0xd0::Opcode0xd0;
use crate::models::mos6502::instructions::legal::opcode0xd1::Opcode0xd1;
use crate::models::mos6502::instructions::legal::opcode0xd5::Opcode0xd5;
use crate::models::mos6502::instructions::legal::opcode0xd6::Opcode0xd6;
use crate::models::mos6502::instructions::legal::opcode0xd8::Opcode0xd8;
use crate::models::mos6502::instructions::legal::opcode0xd9::Opcode0xd9;
use crate::models::mos6502::instructions::legal::opcode0xdd::Opcode0xdd;
use crate::models::mos6502::instructions::legal::opcode0xde::Opcode0xde;
use crate::models::mos6502::instructions::legal::opcode0xe0::Opcode0xe0;
use crate::models::mos6502::instructions::legal::opcode0xe1::Opcode0xe1;
use crate::models::mos6502::instructions::legal::opcode0xe4::Opcode0xe4;
use crate::models::mos6502::instructions::legal::opcode0xe5::Opcode0xe5;
use crate::models::mos6502::instructions::legal::opcode0xe6::Opcode0xe6;
use crate::models::mos6502::instructions::legal::opcode0xe8::Opcode0xe8;
use crate::models::mos6502::instructions::legal::opcode0xe9::Opcode0xe9;
use crate::models::mos6502::instructions::legal::opcode0xea::Opcode0xea;
use crate::models::mos6502::instructions::legal::opcode0xec::Opcode0xec;
use crate::models::mos6502::instructions::legal::opcode0xed::Opcode0xed;
use crate::models::mos6502::instructions::legal::opcode0xee::Opcode0xee;
use crate::models::mos6502::instructions::legal::opcode0xf0::Opcode0xf0;
use crate::models::mos6502::instructions::legal::opcode0xf1::Opcode0xf1;
use crate::models::mos6502::instructions::legal::opcode0xf5::Opcode0xf5;
use crate::models::mos6502::instructions::legal::opcode0xf6::Opcode0xf6;
use crate::models::mos6502::instructions::legal::opcode0xf8::Opcode0xf8;
use crate::models::mos6502::instructions::legal::opcode0xf9::Opcode0xf9;
use crate::models::mos6502::instructions::legal::opcode0xfd::Opcode0xfd;
use crate::models::mos6502::instructions::legal::opcode0xfe::Opcode0xfe;
use crate::models::mos6502::instructions::Opcode;
use crate::models::mos6502::Mos6502;

pub struct Decoder {}

// This is mostly boilerplate to connect the cpu and instructions.
impl Decoder {
    pub fn execute(cpu: &mut Mos6502, opcode: u8) {
        // Opcodes range from 0x00 to 0xFF, also know as a u8
        match opcode {
            0x00 => {
                Opcode0x00::execute(cpu);
            }
            0x01 => {
                Opcode0x01::execute(cpu);
            }
            0x02 => {
                Opcode0x02::execute(cpu);
            }
            0x03 => {
                Opcode0x03::execute(cpu);
            }
            0x04 => {
                Opcode0x04::execute(cpu);
            }
            0x05 => {
                Opcode0x05::execute(cpu);
            }
            0x06 => {
                Opcode0x06::execute(cpu);
            }
            0x07 => {
                Opcode0x07::execute(cpu);
            }
            0x08 => {
                Opcode0x08::execute(cpu);
            }
            0x09 => {
                Opcode0x09::execute(cpu);
            }
            0x0a => {
                Opcode0x0a::execute(cpu);
            }
            0x0b => {
                Opcode0x0b::execute(cpu);
            }
            0x0c => {
                Opcode0x0c::execute(cpu);
            }
            0x0d => {
                Opcode0x0d::execute(cpu);
            }
            0x0e => {
                Opcode0x0e::execute(cpu);
            }
            0x0f => {
                Opcode0x0f::execute(cpu);
            }
            0x10 => {
                Opcode0x10::execute(cpu);
            }
            0x11 => {
                Opcode0x11::execute(cpu);
            }
            0x12 => {
                Opcode0x12::execute(cpu);
            }
            0x13 => {
                Opcode0x13::execute(cpu);
            }
            0x14 => {
                Opcode0x14::execute(cpu);
            }
            0x15 => {
                Opcode0x15::execute(cpu);
            }
            0x16 => {
                Opcode0x16::execute(cpu);
            }
            0x17 => {
                Opcode0x17::execute(cpu);
            }
            0x18 => {
                Opcode0x18::execute(cpu);
            }
            0x19 => {
                Opcode0x19::execute(cpu);
            }
            0x1a => {
                Opcode0x1a::execute(cpu);
            }
            0x1b => {
                Opcode0x1b::execute(cpu);
            }
            0x1c => {
                Opcode0x1c::execute(cpu);
            }
            0x1d => {
                Opcode0x1d::execute(cpu);
            }
            0x1e => {
                Opcode0x1e::execute(cpu);
            }
            0x1f => {
                Opcode0x1f::execute(cpu);
            }
            0x20 => {
                Opcode0x20::execute(cpu);
            }
            0x21 => {
                Opcode0x21::execute(cpu);
            }
            0x22 => {
                Opcode0x22::execute(cpu);
            }
            0x23 => {
                Opcode0x23::execute(cpu);
            }
            0x24 => {
                Opcode0x24::execute(cpu);
            }
            0x25 => {
                Opcode0x25::execute(cpu);
            }
            0x26 => {
                Opcode0x26::execute(cpu);
            }
            0x27 => {
                Opcode0x27::execute(cpu);
            }
            0x28 => {
                Opcode0x28::execute(cpu);
            }
            0x29 => {
                Opcode0x29::execute(cpu);
            }
            0x2a => {
                Opcode0x2a::execute(cpu);
            }
            0x2b => {
                Opcode0x2b::execute(cpu);
            }
            0x2c => {
                Opcode0x2c::execute(cpu);
            }
            0x2d => {
                Opcode0x2d::execute(cpu);
            }
            0x2e => {
                Opcode0x2e::execute(cpu);
            }
            0x2f => {
                Opcode0x2f::execute(cpu);
            }
            0x30 => {
                Opcode0x30::execute(cpu);
            }
            0x31 => {
                Opcode0x31::execute(cpu);
            }
            0x32 => {
                Opcode0x32::execute(cpu);
            }
            0x33 => {
                Opcode0x33::execute(cpu);
            }
            0x34 => {
                Opcode0x34::execute(cpu);
            }
            0x35 => {
                Opcode0x35::execute(cpu);
            }
            0x36 => {
                Opcode0x36::execute(cpu);
            }
            0x37 => {
                Opcode0x37::execute(cpu);
            }
            0x38 => {
                Opcode0x38::execute(cpu);
            }
            0x39 => {
                Opcode0x39::execute(cpu);
            }
            0x3a => {
                Opcode0x3a::execute(cpu);
            }
            0x3b => {
                Opcode0x3b::execute(cpu);
            }
            0x3c => {
                Opcode0x3c::execute(cpu);
            }
            0x3d => {
                Opcode0x3d::execute(cpu);
            }
            0x3e => {
                Opcode0x3e::execute(cpu);
            }
            0x3f => {
                Opcode0x3f::execute(cpu);
            }
            0x40 => {
                Opcode0x40::execute(cpu);
            }
            0x41 => {
                Opcode0x41::execute(cpu);
            }
            0x42 => {
                Opcode0x42::execute(cpu);
            }
            0x43 => {
                Opcode0x43::execute(cpu);
            }
            0x44 => {
                Opcode0x44::execute(cpu);
            }
            0x45 => {
                Opcode0x45::execute(cpu);
            }
            0x46 => {
                Opcode0x46::execute(cpu);
            }
            0x47 => {
                Opcode0x47::execute(cpu);
            }
            0x48 => {
                Opcode0x48::execute(cpu);
            }
            0x49 => {
                Opcode0x49::execute(cpu);
            }
            0x4a => {
                Opcode0x4a::execute(cpu);
            }
            0x4b => {
                Opcode0x4b::execute(cpu);
            }
            0x4c => {
                Opcode0x4c::execute(cpu);
            }
            0x4d => {
                Opcode0x4d::execute(cpu);
            }
            0x4e => {
                Opcode0x4e::execute(cpu);
            }
            0x4f => {
                Opcode0x4f::execute(cpu);
            }
            0x50 => {
                Opcode0x50::execute(cpu);
            }
            0x51 => {
                Opcode0x51::execute(cpu);
            }
            0x52 => {
                Opcode0x52::execute(cpu);
            }
            0x53 => {
                Opcode0x53::execute(cpu);
            }
            0x54 => {
                Opcode0x54::execute(cpu);
            }
            0x55 => {
                Opcode0x55::execute(cpu);
            }
            0x56 => {
                Opcode0x56::execute(cpu);
            }
            0x57 => {
                Opcode0x57::execute(cpu);
            }
            0x58 => {
                Opcode0x58::execute(cpu);
            }
            0x59 => {
                Opcode0x59::execute(cpu);
            }
            0x5a => {
                Opcode0x5a::execute(cpu);
            }
            0x5b => {
                Opcode0x5b::execute(cpu);
            }
            0x5c => {
                Opcode0x5c::execute(cpu);
            }
            0x5d => {
                Opcode0x5d::execute(cpu);
            }
            0x5e => {
                Opcode0x5e::execute(cpu);
            }
            0x5f => {
                Opcode0x5f::execute(cpu);
            }
            0x60 => {
                Opcode0x60::execute(cpu);
            }
            0x61 => {
                Opcode0x61::execute(cpu);
            }
            0x62 => {
                Opcode0x62::execute(cpu);
            }
            0x63 => {
                Opcode0x63::execute(cpu);
            }
            0x64 => {
                Opcode0x64::execute(cpu);
            }
            0x65 => {
                Opcode0x65::execute(cpu);
            }
            0x66 => {
                Opcode0x66::execute(cpu);
            }
            0x67 => {
                Opcode0x67::execute(cpu);
            }
            0x68 => {
                Opcode0x68::execute(cpu);
            }
            0x69 => {
                Opcode0x69::execute(cpu);
            }
            0x6a => {
                Opcode0x6a::execute(cpu);
            }
            0x6b => {
                Opcode0x6b::execute(cpu);
            }
            0x6c => {
                Opcode0x6c::execute(cpu);
            }
            0x6d => {
                Opcode0x6d::execute(cpu);
            }
            0x6e => {
                Opcode0x6e::execute(cpu);
            }
            0x6f => {
                Opcode0x6f::execute(cpu);
            }
            0x70 => {
                Opcode0x70::execute(cpu);
            }
            0x71 => {
                Opcode0x71::execute(cpu);
            }
            0x72 => {
                Opcode0x72::execute(cpu);
            }
            0x73 => {
                Opcode0x73::execute(cpu);
            }
            0x74 => {
                Opcode0x74::execute(cpu);
            }
            0x75 => {
                Opcode0x75::execute(cpu);
            }
            0x76 => {
                Opcode0x76::execute(cpu);
            }
            0x77 => {
                Opcode0x77::execute(cpu);
            }
            0x78 => {
                Opcode0x78::execute(cpu);
            }
            0x79 => {
                Opcode0x79::execute(cpu);
            }
            0x7a => {
                Opcode0x7a::execute(cpu);
            }
            0x7b => {
                Opcode0x7b::execute(cpu);
            }
            0x7c => {
                Opcode0x7c::execute(cpu);
            }
            0x7d => {
                Opcode0x7d::execute(cpu);
            }
            0x7e => {
                Opcode0x7e::execute(cpu);
            }
            0x7f => {
                Opcode0x7f::execute(cpu);
            }
            0x80 => {
                Opcode0x80::execute(cpu);
            }
            0x81 => {
                Opcode0x81::execute(cpu);
            }
            0x82 => {
                Opcode0x82::execute(cpu);
            }
            0x83 => {
                Opcode0x83::execute(cpu);
            }
            0x84 => {
                Opcode0x84::execute(cpu);
            }
            0x85 => {
                Opcode0x85::execute(cpu);
            }
            0x86 => {
                Opcode0x86::execute(cpu);
            }
            0x87 => {
                Opcode0x87::execute(cpu);
            }
            0x88 => {
                Opcode0x88::execute(cpu);
            }
            0x89 => {
                Opcode0x89::execute(cpu);
            }
            0x8a => {
                Opcode0x8a::execute(cpu);
            }
            0x8b => {
                Opcode0x8b::execute(cpu);
            }
            0x8c => {
                Opcode0x8c::execute(cpu);
            }
            0x8d => {
                Opcode0x8d::execute(cpu);
            }
            0x8e => {
                Opcode0x8e::execute(cpu);
            }
            0x8f => {
                Opcode0x8f::execute(cpu);
            }
            0x90 => {
                Opcode0x90::execute(cpu);
            }
            0x91 => {
                Opcode0x91::execute(cpu);
            }
            0x92 => {
                Opcode0x92::execute(cpu);
            }
            0x93 => {
                Opcode0x93::execute(cpu);
            }
            0x94 => {
                Opcode0x94::execute(cpu);
            }
            0x95 => {
                Opcode0x95::execute(cpu);
            }
            0x96 => {
                Opcode0x96::execute(cpu);
            }
            0x97 => {
                Opcode0x97::execute(cpu);
            }
            0x98 => {
                Opcode0x98::execute(cpu);
            }
            0x99 => {
                Opcode0x99::execute(cpu);
            }
            0x9a => {
                Opcode0x9a::execute(cpu);
            }
            0x9b => {
                Opcode0x9b::execute(cpu);
            }
            0x9c => {
                Opcode0x9c::execute(cpu);
            }
            0x9d => {
                Opcode0x9d::execute(cpu);
            }
            0x9e => {
                Opcode0x9e::execute(cpu);
            }
            0x9f => {
                Opcode0x9f::execute(cpu);
            }
            0xa0 => {
                Opcode0xa0::execute(cpu);
            }
            0xa1 => {
                Opcode0xa1::execute(cpu);
            }
            0xa2 => {
                Opcode0xa2::execute(cpu);
            }
            0xa3 => {
                Opcode0xa3::execute(cpu);
            }
            0xa4 => {
                Opcode0xa4::execute(cpu);
            }
            0xa5 => {
                Opcode0xa5::execute(cpu);
            }
            0xa6 => {
                Opcode0xa6::execute(cpu);
            }
            0xa7 => {
                Opcode0xa7::execute(cpu);
            }
            0xa8 => {
                Opcode0xa8::execute(cpu);
            }
            0xa9 => {
                Opcode0xa9::execute(cpu);
            }
            0xaa => {
                Opcode0xaa::execute(cpu);
            }
            0xab => {
                Opcode0xab::execute(cpu);
            }
            0xac => {
                Opcode0xac::execute(cpu);
            }
            0xad => {
                Opcode0xad::execute(cpu);
            }
            0xae => {
                Opcode0xae::execute(cpu);
            }
            0xaf => {
                Opcode0xaf::execute(cpu);
            }
            0xb0 => {
                Opcode0xb0::execute(cpu);
            }
            0xb1 => {
                Opcode0xb1::execute(cpu);
            }
            0xb2 => {
                Opcode0xb2::execute(cpu);
            }
            0xb3 => {
                Opcode0xb3::execute(cpu);
            }
            0xb4 => {
                Opcode0xb4::execute(cpu);
            }
            0xb5 => {
                Opcode0xb5::execute(cpu);
            }
            0xb6 => {
                Opcode0xb6::execute(cpu);
            }
            0xb7 => {
                Opcode0xb7::execute(cpu);
            }
            0xb8 => {
                Opcode0xb8::execute(cpu);
            }
            0xb9 => {
                Opcode0xb9::execute(cpu);
            }
            0xba => {
                Opcode0xba::execute(cpu);
            }
            0xbb => {
                Opcode0xbb::execute(cpu);
            }
            0xbc => {
                Opcode0xbc::execute(cpu);
            }
            0xbd => {
                Opcode0xbd::execute(cpu);
            }
            0xbe => {
                Opcode0xbe::execute(cpu);
            }
            0xbf => {
                Opcode0xbf::execute(cpu);
            }
            0xc0 => {
                Opcode0xc0::execute(cpu);
            }
            0xc1 => {
                Opcode0xc1::execute(cpu);
            }
            0xc2 => {
                Opcode0xc2::execute(cpu);
            }
            0xc3 => {
                Opcode0xc3::execute(cpu);
            }
            0xc4 => {
                Opcode0xc4::execute(cpu);
            }
            0xc5 => {
                Opcode0xc5::execute(cpu);
            }
            0xc6 => {
                Opcode0xc6::execute(cpu);
            }
            0xc7 => {
                Opcode0xc7::execute(cpu);
            }
            0xc8 => {
                Opcode0xc8::execute(cpu);
            }
            0xc9 => {
                Opcode0xc9::execute(cpu);
            }
            0xca => {
                Opcode0xca::execute(cpu);
            }
            0xcb => {
                Opcode0xcb::execute(cpu);
            }
            0xcc => {
                Opcode0xcc::execute(cpu);
            }
            0xcd => {
                Opcode0xcd::execute(cpu);
            }
            0xce => {
                Opcode0xce::execute(cpu);
            }
            0xcf => {
                Opcode0xcf::execute(cpu);
            }
            0xd0 => {
                Opcode0xd0::execute(cpu);
            }
            0xd1 => {
                Opcode0xd1::execute(cpu);
            }
            0xd2 => {
                Opcode0xd2::execute(cpu);
            }
            0xd3 => {
                Opcode0xd3::execute(cpu);
            }
            0xd4 => {
                Opcode0xd4::execute(cpu);
            }
            0xd5 => {
                Opcode0xd5::execute(cpu);
            }
            0xd6 => {
                Opcode0xd6::execute(cpu);
            }
            0xd7 => {
                Opcode0xd7::execute(cpu);
            }
            0xd8 => {
                Opcode0xd8::execute(cpu);
            }
            0xd9 => {
                Opcode0xd9::execute(cpu);
            }
            0xda => {
                Opcode0xda::execute(cpu);
            }
            0xdb => {
                Opcode0xdb::execute(cpu);
            }
            0xdc => {
                Opcode0xdc::execute(cpu);
            }
            0xdd => {
                Opcode0xdd::execute(cpu);
            }
            0xde => {
                Opcode0xde::execute(cpu);
            }
            0xdf => {
                Opcode0xdf::execute(cpu);
            }
            0xe0 => {
                Opcode0xe0::execute(cpu);
            }
            0xe1 => {
                Opcode0xe1::execute(cpu);
            }
            0xe2 => {
                Opcode0xe2::execute(cpu);
            }
            0xe3 => {
                Opcode0xe3::execute(cpu);
            }
            0xe4 => {
                Opcode0xe4::execute(cpu);
            }
            0xe5 => {
                Opcode0xe5::execute(cpu);
            }
            0xe6 => {
                Opcode0xe6::execute(cpu);
            }
            0xe7 => {
                Opcode0xe7::execute(cpu);
            }
            0xe8 => {
                Opcode0xe8::execute(cpu);
            }
            0xe9 => {
                Opcode0xe9::execute(cpu);
            }
            0xea => {
                Opcode0xea::execute(cpu);
            }
            0xeb => {
                Opcode0xeb::execute(cpu);
            }
            0xec => {
                Opcode0xec::execute(cpu);
            }
            0xed => {
                Opcode0xed::execute(cpu);
            }
            0xee => {
                Opcode0xee::execute(cpu);
            }
            0xef => {
                Opcode0xef::execute(cpu);
            }
            0xf0 => {
                Opcode0xf0::execute(cpu);
            }
            0xf1 => {
                Opcode0xf1::execute(cpu);
            }
            0xf2 => {
                Opcode0xf2::execute(cpu);
            }
            0xf3 => {
                Opcode0xf3::execute(cpu);
            }
            0xf4 => {
                Opcode0xf4::execute(cpu);
            }
            0xf5 => {
                Opcode0xf5::execute(cpu);
            }
            0xf6 => {
                Opcode0xf6::execute(cpu);
            }
            0xf7 => {
                Opcode0xf7::execute(cpu);
            }
            0xf8 => {
                Opcode0xf8::execute(cpu);
            }
            0xf9 => {
                Opcode0xf9::execute(cpu);
            }
            0xfa => {
                Opcode0xfa::execute(cpu);
            }
            0xfb => {
                Opcode0xfb::execute(cpu);
            }
            0xfc => {
                Opcode0xfc::execute(cpu);
            }
            0xfd => {
                Opcode0xfd::execute(cpu);
            }
            0xfe => {
                Opcode0xfe::execute(cpu);
            }
            0xff => {
                Opcode0xff::execute(cpu);
            }
            _ => {
                panic!("Decoder not aware of instruction opcode '{0}'.", opcode);
            }
        }
    }
}
