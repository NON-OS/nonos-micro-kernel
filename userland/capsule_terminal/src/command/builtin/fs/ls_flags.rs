// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Argument parsing for `ls`: clustered or separate short flags, `--` to end
//! the flag run, and every remaining word as a path operand.

use alloc::vec::Vec;

#[derive(Default, Clone, Copy, PartialEq, Eq, Debug)]
pub struct LsFlags {
    pub long: bool,
    pub all: bool,
    pub human: bool,
    pub one_per_line: bool,
    pub recurse: bool,
    pub by_time: bool,
    pub by_size: bool,
}

impl LsFlags {
    pub fn needs_meta(&self) -> bool {
        self.long || self.by_time || self.by_size
    }
}

pub fn parse<'a>(argv: &[&'a [u8]]) -> Result<(LsFlags, Vec<&'a [u8]>), u8> {
    let mut flags = LsFlags::default();
    let mut operands: Vec<&'a [u8]> = Vec::new();
    let mut ended = false;
    for arg in argv.iter().skip(1) {
        if !ended && *arg == b"--" {
            ended = true;
            continue;
        }
        if ended || arg.len() < 2 || arg[0] != b'-' {
            operands.push(arg);
            continue;
        }
        for &c in &arg[1..] {
            match c {
                b'l' => flags.long = true,
                b'a' => flags.all = true,
                b'h' => flags.human = true,
                b'1' => flags.one_per_line = true,
                b'R' => flags.recurse = true,
                b't' => flags.by_time = true,
                b'S' => flags.by_size = true,
                _ => return Err(c),
            }
        }
    }
    Ok((flags, operands))
}
