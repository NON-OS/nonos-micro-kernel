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

//! Reading and writing the ring region as the part does, by offset.

use nonos_devmodel::FakeBar;

pub fn u16_at(bar: &FakeBar, off: usize) -> u16 {
    bar.wrote16(off)
}

pub fn u32_at(bar: &FakeBar, off: usize) -> u32 {
    bar.wrote32(off)
}

pub fn u64_at(bar: &FakeBar, off: usize) -> u64 {
    (bar.wrote32(off) as u64) | ((bar.wrote32(off + 4) as u64) << 32)
}

pub fn bytes(bar: &FakeBar, off: usize, len: usize) -> Vec<u8> {
    (0..len).map(|i| bar.wrote8(off + i)).collect()
}

pub fn put(bar: &FakeBar, off: usize, data: &[u8]) {
    for (i, b) in data.iter().enumerate() {
        bar.present8(off + i, *b);
    }
}

/// A device address back to an offset in the region. The harness makes the
/// two address spaces the same, so anything outside the region is a driver
/// error and the test should see it as one.
pub fn offset_of(bar: &FakeBar, addr: u64) -> usize {
    let off = addr.wrapping_sub(bar.base()) as usize;
    assert!(off < bar.len(), "descriptor address outside the ring region");
    off
}

/// One descriptor: address, length, flags, next.
pub struct Desc {
    pub addr: u64,
    pub len: u32,
    pub flags: u16,
    pub next: u16,
}

pub fn desc(bar: &FakeBar, index: u16, desc_offset: usize) -> Desc {
    let at = desc_offset + index as usize * 16;
    Desc {
        addr: u64_at(bar, at),
        len: u32_at(bar, at + 8),
        flags: u16_at(bar, at + 12),
        next: u16_at(bar, at + 14),
    }
}
