#!/usr/bin/env python3

import os
import shutil

copyright_header = """////////////////////////////////////////////////////////////////////////////////////////////////////
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
////////////////////////////////////////////////////////////////////////////////////////////////////"""

struct_prefix = "struct Opcode"
struct_suffix = " {}"
opcode_import = "use crate::nes::instructions::Opcode;"
impl_prefix = "impl Opcode for Opcode"
impl_suffix = "}"

temp_dir = "/tmp/opcodes/generated"

def clean():
    if os.path.exists(temp_dir):
        shutil.rmtree(temp_dir)

    os.makedirs(temp_dir)
    os.makedirs("{}/illegal".format(temp_dir))
    os.makedirs("{}/legal".format(temp_dir))


# https://www.masswerk.at/6502/6502_instruction_set.html
illegal = [
    # 0x0
    0x02,
    0x03,
    0x04,
    0x07,
    0x0B,
    0x0C,
    0x0F,
    # 0x1
    0x12,
    0x13,
    0x14,
    0x17,
    0x1A,
    0x1B,
    0x1C,
    0x1F,
    # 0x2
    0x22,
    0x23,
    0x27,
    0x2B,
    0x2F,
    # 0x3
    0x32,
    0x33,
    0x34,
    0x37,
    0x3A,
    0x3B,
    0x3C,
    0x3F,
    # 0x4
    0x42,
    0x43,
    0x44,
    0x47,
    0x4B,
    0x4F,
    # 0x5
    0x52,
    0x53,
    0x54,
    0x57,
    0x5A,
    0x5B,
    0x5C,
    0x5F,
    # 0x6
    0x62,
    0x63,
    0x64,
    0x67,
    0x6B,
    0x6F,
    # 0x7
    0x72,
    0x73,
    0x74,
    0x77,
    0x7A,
    0x7B,
    0x7C,
    0x7F,
    # 0x8
    0x80,
    0x82,
    0x83,
    0x87,
    0x89,
    0x8B,
    0x8F,
    # 0x9
    0x92,
    0x93,
    0x97,
    0x9B,
    0x9C,
    0x9E,
    0x9F,
    # 0xA
    0xA3,
    0xA7,
    0xAB,
    0xAF,
    # 0xB
    0xB2,
    0xB3,
    0xB7,
    0xBB,
    0xBF,
    # 0xC
    0xC2,
    0xC3,
    0xC7,
    0xCB,
    0xCF,
    # 0xD
    0xD2,
    0xD3,
    0xD4,
    0xD7,
    0xDA,
    0xDB,
    0xDC,
    0xDF,
    # 0xE
    0xE2,
    0xE3,
    0xE7,
    0xEB,
    0xEF,
    # 0xF
    0xF2,
    0xF3,
    0xF4,
    0xF7,
    0xFA,
    0xFB,
    0xFC,
    0xFF
]

def get_opcode(input):
    return "0x%0*x" % (2, input)

def generate_opcodes():

    for i in range(0x00, 0xFF+1):
        opcode = get_opcode(i)
        impl_content = """
    fn get_name(&mut self) -> &str {{
        return "{}"
    }}
    
    fn decode(&mut self) {{
        print!("TBD")
    }}
""".format(opcode)
        file_contents = """{}\n\n{}\n\n{}{}{}\n\n{}{}{}\n{}\n{}\n""".format(
                copyright_header,
                opcode_import,
                struct_prefix, opcode, struct_suffix,
                impl_prefix, opcode, " {", impl_content, impl_suffix)
        legality_path = "legal"
        if i in illegal:
            legality_path = "illegal"
        path = "{}/{}/opcode{}.rs".format(temp_dir, legality_path, opcode)
        with open(path, "w") as text_file:
            text_file.write(file_contents)

def generate_legal_mod_rs():
    path = "{}/legal/mod.rs".format(temp_dir)
    with open(path, "w") as mod_file:
        file_contents = """{}\n\n""".format(copyright_header)
        for i in range(0x00, 0xFF+1):
            if i not in illegal:
                file_contents += "pub mod opcode{};\n".format(get_opcode(i))

        mod_file.write(file_contents)

def generate_illegal_mod_rs():

    path = "{}/illegal/mod.rs".format(temp_dir)
    with open(path, "w") as mod_file:
        file_contents = """{}\n\n""".format(copyright_header)
        for i in range(0x00, 0xFF+1):
            if i in illegal:
                file_contents += "pub mod opcode{};\n".format(get_opcode(i))

        mod_file.write(file_contents)

clean()
generate_opcodes()
generate_legal_mod_rs()
generate_illegal_mod_rs()

# Sample usage from within project root directory
# $ rm -rf ./src/nes/instructions/illegal/ ./src/nes/instructions/legal/ && ./generate_opcode_files.py && cp -r /tmp/opcodes/generated/* ./src/nes/instructions/
