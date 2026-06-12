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

#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct Quote {
    pub magic: u32,
    pub pcr_mask: u32,
    pub pcr_digest: [u8; 32],
    pub signature: [u8; 256],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct PcrBank {
    pub algorithm: u16,
    pub digest_size: u16,
    pub pcr_count: u32,
}

impl PcrBank {
    pub fn sha256() -> Self {
        Self { algorithm: 0x000B, digest_size: 32, pcr_count: 24 }
    }
    pub fn sha1() -> Self {
        Self { algorithm: 0x0004, digest_size: 20, pcr_count: 24 }
    }
    pub fn sha384() -> Self {
        Self { algorithm: 0x000C, digest_size: 48, pcr_count: 24 }
    }
}
