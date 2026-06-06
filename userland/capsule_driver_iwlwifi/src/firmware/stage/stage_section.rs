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

pub(super) fn stage_section(
    data: &[u8],
    off: usize,
    len: usize,
    dma: u64,
    cap: usize,
    dst: &mut usize,
) -> Option<()> {
    let payload_len = len - 4;
    let total = 12usize.checked_add(payload_len)?;
    if dst.checked_add(total)? > cap {
        return None;
    }
    unsafe {
        let ptr = (dma as *mut u8).add(*dst);
        core::ptr::copy_nonoverlapping(data.as_ptr().add(off), ptr, 4);
        core::ptr::copy_nonoverlapping(
            &(payload_len as u32).to_le_bytes() as *const u8,
            ptr.add(4),
            4,
        );
        core::ptr::write_bytes(ptr.add(8), 0, 4);
        core::ptr::copy_nonoverlapping(data.as_ptr().add(off + 4), ptr.add(12), payload_len);
    }
    *dst += total;
    Some(())
}
