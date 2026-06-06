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

pub fn fill(base: u64, width: u32, height: u32, stride: u32, argb: u32) {
    for y in 0..height {
        for x in 0..width {
            let cell = (base + (y as u64 * stride as u64 + x as u64 * 4)) as *mut u32;
            unsafe { core::ptr::write_volatile(cell, argb) };
        }
    }
}
