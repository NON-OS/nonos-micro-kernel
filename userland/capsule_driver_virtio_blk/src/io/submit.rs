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
use super::error::BlkError;
use super::read_seq::read_seq;
use crate::constants::{
    LEG_QUEUE_NOTIFY, VIRTIO_BLK_S_IOERR, VIRTIO_BLK_S_OK, VIRTIO_BLK_S_UNSUPP,
};
use crate::queue::{Direction, Queue};
use crate::regs::Regs;
use nonos_libc::{mk_irq_ack, mk_irq_wait};

/// Per-wait slice and the whole-request budget. The budget is counted in
/// slices actually spent, wait or not: a shared interrupt line that keeps
/// advancing the sequence must consume budget on every pass, or a request the
/// device never completes pins the server in this loop forever.
const WAIT_SLICE_MS: u64 = 100;
const MAX_SLICES: u32 = 50;
const MAX_PASSES: u32 = 5000;

pub fn submit(
    regs: Regs,
    queue: &mut Queue,
    irq_grant: u64,
    dir: Direction,
    lba: u64,
    nsectors: u32,
) -> Result<(), BlkError> {
    queue.post_request(dir, lba, nsectors);
    unsafe { regs.w16(LEG_QUEUE_NOTIFY, 0) }
    let mut seq = read_seq(irq_grant)?;
    // Block on the interrupt instead of yield-polling. The old loop spun up
    // to 200k yields per request; every disk read then cycled the whole run
    // queue for the request's full latency, and on one CPU the rest of the
    // system paid for each sector. Sliced waits keep the same total budget
    // and the used-ring check on every wake covers a completion whose
    // interrupt was suppressed or already consumed.
    // Two bounds, each safe alone. Timed-out waits count toward the time
    // budget, so an idle device gets the full five seconds. Every pass counts
    // toward the iteration guard, so a shared line whose sequence keeps
    // advancing cannot hold the loop open forever, and a healthy request
    // finishes thousands of iterations under it.
    let mut timed_out_slices = 0u32;
    let mut passes = 0u32;
    loop {
        let observed = queue.used_idx();
        if observed.wrapping_sub(queue.last_used) != 0 {
            queue.last_used = observed;
            break;
        }
        passes = passes.wrapping_add(1);
        if passes > MAX_PASSES {
            queue.last_used = queue.used_idx();
            return Err(BlkError::Timeout);
        }
        let mut out_seq: u64 = 0;
        if mk_irq_wait(irq_grant, seq, WAIT_SLICE_MS, &mut out_seq) >= 0 {
            seq = out_seq;
        } else {
            timed_out_slices = timed_out_slices.wrapping_add(1);
            if timed_out_slices > MAX_SLICES {
                queue.last_used = queue.used_idx();
                return Err(BlkError::Timeout);
            }
        }
    }
    let status = queue.status_byte();
    if mk_irq_ack(irq_grant) < 0 {
        return Err(BlkError::Io);
    }
    match status {
        VIRTIO_BLK_S_OK => Ok(()),
        VIRTIO_BLK_S_IOERR => Err(BlkError::Io),
        VIRTIO_BLK_S_UNSUPP => Err(BlkError::Unsupported),
        _ => Err(BlkError::Io),
    }
}
