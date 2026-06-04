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
use super::{avail, desc, layout::QueueLayout, used, wait};
use crate::constants::CTRLQ_INDEX;
use crate::regs::Regs;
use core::ptr::{read_volatile, write_volatile};
pub struct SubmitOutput {
    pub used_len: u32,
}
pub fn submit_sync(
    layout: QueueLayout,
    regs: Regs,
    request: &[u8],
    resp_len: u32,
) -> Result<SubmitOutput, &'static str> {
    let req_len = request.len();
    if req_len == 0 || resp_len == 0 {
        return Err("virtio-gpu: empty request/response");
    }
    let total = req_len.checked_add(resp_len as usize).ok_or("virtio-gpu: overflow")?;
    if total > layout.staging_len() {
        return Err("virtio-gpu: staging too small");
    }
    let staging = layout.staging_va();
    unsafe {
        for (i, b) in request.iter().enumerate() {
            write_volatile(staging.add(i), *b);
        }
        for i in req_len..total {
            write_volatile(staging.add(i), 0);
        }
    }
    let req_addr = layout.staging_device_addr();
    let resp_addr = req_addr + req_len as u64;
    let pre_used_idx = used::read_idx(layout);
    let head = avail::read_idx(layout) % layout.queue_size;
    desc::write_request_chain(layout, head, req_addr, req_len as u32, resp_addr, resp_len);
    avail::publish(layout, head);
    unsafe {
        regs.notify(CTRLQ_INDEX);
    }
    wait::used_changed(layout, pre_used_idx)?;
    let entry = used::read_entry(layout, pre_used_idx);
    if entry.id as u16 != head {
        return Err("virtio-gpu: used id mismatch");
    }
    Ok(SubmitOutput { used_len: entry.len })
}
pub fn read_response_byte(layout: QueueLayout, req_len: usize, offset: usize) -> u8 {
    let staging = layout.staging_va();
    unsafe { read_volatile(staging.add(req_len + offset)) }
}
