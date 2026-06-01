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

use nonos_runtime::mmap;

const PROT_RW: i32 = 0x1 | 0x2;
const MAP_PRIVATE_ANON: i32 = 0x02 | 0x20;

pub(super) fn alloc_backing(width: u32, height: u32) -> Option<(*mut u32, u32, u64)> {
    let stride = width * 4;
    let byte_len = width as u64 * height as u64 * 4;
    let base = mmap(core::ptr::null_mut(), byte_len as usize, PROT_RW, MAP_PRIVATE_ANON, -1, 0);
    if base.is_null() || (base as i64) < 0 {
        return None;
    }
    Some((base as *mut u32, stride, byte_len))
}
