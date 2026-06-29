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







use crate::constants::{RING_SLOTS, VIRTIO_NET_HDR_LEN};
use crate::queue::RxQueue;

pub struct Frame<'a> {
    pub bytes: &'a [u8],
}








pub unsafe fn take_one(rx: &mut RxQueue) -> Option<Frame<'static>> {
    let used = rx.used_idx();
    if used == rx.last_used {
        return None;
    }
    let ring_pos = rx.last_used % RING_SLOTS;
    let (desc_id, used_len) = rx.used_elem_at(ring_pos);





    let (payload_ptr, payload_len) = if used_len as usize > VIRTIO_NET_HDR_LEN {
        let raw = (used_len as usize) - VIRTIO_NET_HDR_LEN;
        let cap = (rx.buf_len as usize).saturating_sub(VIRTIO_NET_HDR_LEN);
        let len = core::cmp::min(raw, cap);
        let slot = (desc_id as usize) % (rx.buf_count as usize);
        let base = rx.buf_va.add(rx.buf_len as usize * slot + VIRTIO_NET_HDR_LEN);
        (base as *const u8, len)
    } else {
        (core::ptr::null::<u8>(), 0usize)
    };

    rx.last_used = rx.last_used.wrapping_add(1);
    rx.pending_refill = Some(desc_id as u16);

    if payload_len == 0 {
        return Some(Frame { bytes: &[] });
    }
    let bytes: &'static [u8] = core::slice::from_raw_parts(payload_ptr, payload_len);
    Some(Frame { bytes })
}
