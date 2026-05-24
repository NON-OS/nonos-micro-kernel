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

use crate::constants::{LEG_QUEUE_NOTIFY, Q_TX, VIRTIO_NET_HDR_LEN};
use crate::queue::TxQueue;
use crate::regs::Regs;

const MAX_YIELDS: u32 = 200_000;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TxError {
    IrqAck,
    IrqPoll,
    Timeout,
}

pub fn send(regs: Regs, tx: &mut TxQueue, irq_grant: u64, frame: &[u8]) -> Result<(), TxError> {
    let total = (VIRTIO_NET_HDR_LEN + frame.len()) as u32;
    unsafe {
        let buf = tx.buffer_mut(total);
        for b in buf[..VIRTIO_NET_HDR_LEN].iter_mut() {
            *b = 0;
        }
        buf[VIRTIO_NET_HDR_LEN..].copy_from_slice(frame);
    }
    tx.post_packet(total);
    unsafe {
        regs.w16(LEG_QUEUE_NOTIFY, Q_TX);
    }

    let prev_seq = read_seq(irq_grant)?;
    let target = tx.last_used.wrapping_add(1);
    let mut tries = 0u32;
    loop {
        if tx.used_idx() == target {
            break;
        }
        if read_seq(irq_grant)? != prev_seq {
            break;
        }
        if tries >= MAX_YIELDS {
            return Err(TxError::Timeout);
        }
        mk_yield();
        tries = tries.wrapping_add(1);
    }
    tx.last_used = target;
    if mk_irq_ack(irq_grant) < 0 {
        return Err(TxError::IrqAck);
    }
    Ok(())
}

fn read_seq(grant: u64) -> Result<u64, TxError> {
    let mut out = IrqPollOut { seq: 0, overflow: 0 };
    if mk_irq_poll(grant, &mut out as *mut _) < 0 {
        return Err(TxError::IrqPoll);
    }
    Ok(out.seq)
}
