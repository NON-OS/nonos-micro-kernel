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

//! Issue a host command to the firmware: place its header and payload in the
//! command buffer for the current ring slot, fill that slot's TFD to point at
//! it, advance the write pointer, and ring the doorbell. This ties the header,
//! ring, TFD and doorbell pieces into the one operation every post-alive step
//! (NVM read, scan, auth, assoc) uses. Written to the legacy spec; the exact
//! transfer still needs hardware validation, like the firmware-load path.

use crate::constants::{CMD_SLOT_SIZE, TFD_QUEUE_SIZE};
use crate::hcmd::doorbell;
use crate::hcmd::header::{cmd_header, make_sequence, CMD_HEADER_LEN};
use crate::hcmd::ring::advance;
use crate::hcmd::tfd::{fill_single, TFD_SIZE};
use crate::regs::Mmio;

/// The DMA-visible layout of a command queue: the CPU and device views of the
/// buffer, the byte offsets of the TFD ring and the per-slot command area, and
/// the queue id the doorbell targets.
#[derive(Clone, Copy)]
pub struct CmdQueue {
    pub dma_user_va: u64,
    pub dma_device_addr: u64,
    pub ring_off: usize,
    pub cmd_off: usize,
    pub queue: u8,
}

/// Compose and issue a host command over `q` at `write_ptr`: place the header
/// and payload in the slot's command buffer, fill the slot's TFD to point at
/// it, advance the write pointer and ring the doorbell. Returns the new write
/// pointer, or `None` if the command does not fit a slot.
///
/// # Safety
/// `q.dma_user_va` must point at a buffer of at least
/// `q.cmd_off + TFD_QUEUE_SIZE * CMD_SLOT_SIZE` bytes, mapped for the device at
/// `q.dma_device_addr`.
pub unsafe fn send_host_command<M: Mmio>(
    mmio: &M,
    q: &CmdQueue,
    write_ptr: usize,
    cmd: u8,
    group: u8,
    payload: &[u8],
) -> Option<usize> {
    let idx = write_ptr & (TFD_QUEUE_SIZE - 1);
    let total = CMD_HEADER_LEN.checked_add(payload.len())?;
    if total > CMD_SLOT_SIZE || total > crate::hcmd::tfd::TFD_MAX_TB_LEN as usize {
        return None;
    }
    let hdr = cmd_header(cmd, group, make_sequence(q.queue, idx as u8));
    let slot = q.cmd_off + idx * CMD_SLOT_SIZE;
    unsafe {
        let base = (q.dma_user_va as *mut u8).add(slot);
        core::ptr::copy_nonoverlapping(hdr.as_ptr(), base, hdr.len());
        if !payload.is_empty() {
            core::ptr::copy_nonoverlapping(payload.as_ptr(), base.add(hdr.len()), payload.len());
        }
        let tfd_ptr = (q.dma_user_va as *mut u8).add(q.ring_off + idx * TFD_SIZE);
        let tfd = core::slice::from_raw_parts_mut(tfd_ptr, TFD_SIZE);
        if !fill_single(tfd, q.dma_device_addr + slot as u64, total as u16) {
            return None;
        }
    }
    let next = advance(write_ptr);
    doorbell::ring(mmio, q.queue, next as u8);
    Some(next)
}
