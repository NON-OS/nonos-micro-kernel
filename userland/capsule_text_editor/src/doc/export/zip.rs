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

mod central;

use alloc::vec::Vec;

pub fn crc32(data: &[u8]) -> u32 {
    let mut c = 0xFFFF_FFFFu32;
    for &b in data {
        c ^= b as u32;
        for _ in 0..8 {
            c = if c & 1 != 0 { (c >> 1) ^ 0xEDB8_8320 } else { c >> 1 };
        }
    }
    !c
}

struct Entry {
    name: Vec<u8>,
    crc: u32,
    size: u32,
    offset: u32,
}

pub struct Zip {
    out: Vec<u8>,
    entries: Vec<Entry>,
}

impl Zip {
    pub fn new() -> Self {
        Self { out: Vec::new(), entries: Vec::new() }
    }

    pub fn add(&mut self, name: &str, data: &[u8]) {
        let offset = self.out.len() as u32;
        let crc = crc32(data);
        let size = data.len() as u32;
        self.out.extend_from_slice(b"PK\x03\x04");
        self.out.extend_from_slice(&[20, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        self.out.extend_from_slice(&crc.to_le_bytes());
        self.out.extend_from_slice(&size.to_le_bytes());
        self.out.extend_from_slice(&size.to_le_bytes());
        self.out.extend_from_slice(&(name.len() as u16).to_le_bytes());
        self.out.extend_from_slice(&0u16.to_le_bytes());
        self.out.extend_from_slice(name.as_bytes());
        self.out.extend_from_slice(data);
        self.entries.push(Entry { name: name.as_bytes().to_vec(), crc, size, offset });
    }
}

impl Default for Zip {
    fn default() -> Self {
        Self::new()
    }
}
