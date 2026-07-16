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

use super::hex::hex_u64;

pub(super) fn log_handoff_fb(ptr: u64, stride: u32, format: u32) {
    let mut buf = [0u8; 16];
    crate::sys::serial::print(b"[FB] handoff ptr=0x");
    crate::sys::serial::print(hex_u64(ptr, &mut buf));
    crate::sys::serial::print(b" stride=0x");
    let mut b2 = [0u8; 16];
    crate::sys::serial::print(hex_u64(stride as u64, &mut b2));
    crate::sys::serial::print(b" fmt=0x");
    let mut b3 = [0u8; 16];
    crate::sys::serial::println(hex_u64(format as u64, &mut b3));
}
