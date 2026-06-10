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

//! IRQ binding and notification. Cap requirement: `Irq`. Two flag
//! values are accepted today:
//!
//!   * `0` — legacy INTx. `irq_source` is the device's GSI as
//!     reported by `mk_device_list`; `vector_count` must be 0.
//!   * `MK_IRQ_BIND_MSIX` — MSI-X. `irq_source` must be 0;
//!     `vector_count` is the run length, 1..=64. The kernel
//!     allocates the run, programs the device's MSI-X table, and
//!     returns the BASE grant id and BASE vector. Per-vector grant
//!     ids are derived as `grant_id + i` for `i` in
//!     `0..vector_count`.

use super::types::{IrqBindOut, IrqPollOut};
use crate::syscall::{
    call_raw, N_MK_IRQ_ACK, N_MK_IRQ_BIND, N_MK_IRQ_POLL, N_MK_IRQ_UNBIND, N_MK_IRQ_WAIT,
};

pub const MK_IRQ_BIND_MSIX: u32 = 1 << 0;

#[no_mangle]
pub extern "C" fn mk_irq_bind(
    device_id: u64,
    claim_epoch: u64,
    irq_source: u32,
    flags: u32,
    vector_count: u32,
    out: *mut IrqBindOut,
) -> i64 {
    call_raw(
        N_MK_IRQ_BIND,
        [device_id, claim_epoch, irq_source as u64, flags as u64, vector_count as u64, out as u64],
    )
}

#[no_mangle]
pub extern "C" fn mk_irq_unbind(grant_id: u64) -> i64 {
    call_raw(N_MK_IRQ_UNBIND, [grant_id, 0, 0, 0, 0, 0])
}

#[no_mangle]
pub extern "C" fn mk_irq_ack(grant_id: u64) -> i64 {
    call_raw(N_MK_IRQ_ACK, [grant_id, 0, 0, 0, 0, 0])
}

#[no_mangle]
pub extern "C" fn mk_irq_poll(grant_id: u64, out: *mut IrqPollOut) -> i64 {
    call_raw(N_MK_IRQ_POLL, [grant_id, out as u64, 0, 0, 0, 0])
}

// Blocks until any IRQ delivery moves the seq past `last_seq`, the
// timeout lapses (0 = kernel default), or an unrelated wake lands —
// spurious returns are expected, callers loop. `grant_id` 0 waits
// on every grant this capsule owns; `out_seq` receives the value to
// pass back as the next `last_seq`.
#[no_mangle]
pub extern "C" fn mk_irq_wait(
    grant_id: u64,
    last_seq: u64,
    timeout_ms: u64,
    out_seq: *mut u64,
) -> i64 {
    call_raw(N_MK_IRQ_WAIT, [grant_id, last_seq, timeout_ms, out_seq as u64, 0, 0])
}
