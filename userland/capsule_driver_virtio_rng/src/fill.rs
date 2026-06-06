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

use nonos_libc::{mk_irq_ack, mk_irq_poll, mk_yield, IrqPollOut};

use super::constants::LEG_QUEUE_NOTIFY;
use super::queue::Queue;
use super::regs::Regs;

const MAX_YIELDS: u32 = 100_000;

pub fn fill(regs: Regs, queue: &mut Queue, irq_grant: u64) -> Result<u32, &'static str> {
    queue.post_request();
    unsafe {
        regs.w16(LEG_QUEUE_NOTIFY, 0);
    }

    let prev_seq = read_seq(irq_grant);
    let target = queue.last_used.wrapping_add(1);

    let mut tries = 0u32;
    loop {
        if queue.used_idx() == target {
            break;
        }
        if read_seq(irq_grant) != prev_seq {
            break;
        }
        if tries >= MAX_YIELDS {
            return Err("virtio-rng: device did not respond");
        }
        let _ = mk_yield();
        tries = tries.wrapping_add(1);
    }
    queue.last_used = target;
    let len = queue.used_len();
    let _ = mk_irq_ack(irq_grant);
    Ok(len)
}

fn read_seq(grant: u64) -> u64 {
    let mut out = IrqPollOut { seq: 0, overflow: 0 };
    let _ = mk_irq_poll(grant, &mut out as *mut _);
    out.seq
}
