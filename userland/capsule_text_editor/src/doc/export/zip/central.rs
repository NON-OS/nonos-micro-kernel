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

use alloc::vec::Vec;

use super::Zip;

impl Zip {
    pub fn finish(mut self) -> Vec<u8> {
        let start = self.out.len() as u32;
        for e in &self.entries {
            self.out.extend_from_slice(b"PK\x01\x02");
            self.out.extend_from_slice(&[20, 0, 20, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
            self.out.extend_from_slice(&e.crc.to_le_bytes());
            self.out.extend_from_slice(&e.size.to_le_bytes());
            self.out.extend_from_slice(&e.size.to_le_bytes());
            self.out.extend_from_slice(&(e.name.len() as u16).to_le_bytes());
            self.out.extend_from_slice(&[0u8; 12]);
            self.out.extend_from_slice(&e.offset.to_le_bytes());
            self.out.extend_from_slice(&e.name);
        }
        let cd_size = self.out.len() as u32 - start;
        let n = self.entries.len() as u16;
        self.out.extend_from_slice(b"PK\x05\x06\x00\x00\x00\x00");
        self.out.extend_from_slice(&n.to_le_bytes());
        self.out.extend_from_slice(&n.to_le_bytes());
        self.out.extend_from_slice(&cd_size.to_le_bytes());
        self.out.extend_from_slice(&start.to_le_bytes());
        self.out.extend_from_slice(&0u16.to_le_bytes());
        self.out
    }
}
