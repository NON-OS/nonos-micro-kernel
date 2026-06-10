// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

// Copy `len` payload bytes from `payload[src_off..src_off+len]` to
// the bootloader-allocated phys frame at `dst_phys`. The phys frame
// is identity-mapped while UEFI Boot Services are still active, so
// the raw cast is sound.
//
// SAFETY:
//   - `dst_phys` must be a 4 KiB-aligned phys page allocated by the
//     loader and not freed.
//   - `[dst_phys, dst_phys + len)` must lie within that allocation.
//   - `src_off + len <= payload.len()`.
pub unsafe fn copy_payload(payload: &[u8], src_off: usize, dst_phys: u64, len: usize) {
    core::ptr::copy_nonoverlapping(payload.as_ptr().add(src_off), dst_phys as *mut u8, len);
}
