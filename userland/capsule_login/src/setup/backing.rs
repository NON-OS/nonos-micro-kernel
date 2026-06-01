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

use nonos_libc::mk_mmap;

pub(super) fn alloc(width: u32, height: u32) -> Result<(u64, u32, u64), &'static str> {
    let stride = width.checked_mul(4).ok_or("stride overflow")?;
    let byte_len = (stride as u64).checked_mul(height as u64).ok_or("surface size overflow")?;
    let base = mk_mmap(
        core::ptr::null_mut(),
        byte_len as usize,
        super::constants::PROT_READ_WRITE,
        super::constants::MAP_PRIVATE_ANON,
        -1,
        0,
    );
    if (base as isize) <= 0 {
        return Err("backing mmap failed");
    }
    Ok((base as u64, stride, byte_len))
}
