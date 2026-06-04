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
use super::layout::QueueLayout;
use crate::constants::{VRING_DESC_F_NEXT, VRING_DESC_F_WRITE};
use core::ptr::write_volatile;
pub const DESC_LEN: usize = 16;
pub fn write_desc(layout: QueueLayout, index: u16, addr: u64, len: u32, flags: u16, next: u16) {
    let base = layout.desc_va() as usize + (index as usize) * DESC_LEN;
    unsafe {
        write_volatile(base as *mut u64, addr);
        write_volatile((base + 8) as *mut u32, len);
        write_volatile((base + 12) as *mut u16, flags);
        write_volatile((base + 14) as *mut u16, next);
    }
}
pub fn write_request_chain(
    layout: QueueLayout,
    head: u16,
    req_addr: u64,
    req_len: u32,
    resp_addr: u64,
    resp_len: u32,
) -> u16 {
    let resp_idx = head.wrapping_add(1) % layout.queue_size;
    write_desc(layout, head, req_addr, req_len, VRING_DESC_F_NEXT, resp_idx);
    write_desc(layout, resp_idx, resp_addr, resp_len, VRING_DESC_F_WRITE, 0);
    head
}
