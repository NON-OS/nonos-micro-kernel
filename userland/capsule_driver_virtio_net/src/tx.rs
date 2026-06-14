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

use nonos_libc::{mk_irq_ack, mk_yield};

use crate::constants::{LEG_QUEUE_NOTIFY, MIN_ETHERNET_FRAME, Q_TX, VIRTIO_NET_HDR_LEN};
use crate::queue::TxQueue;
use crate::regs::Regs;

const MAX_YIELDS: u32 = 200_000;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TxError {
    IrqAck,
    Timeout,
}

pub fn send(regs: Regs, tx: &mut TxQueue, irq_grant: u64, frame: &[u8]) -> Result<(), TxError> {
    let eth_len = if frame.len() < MIN_ETHERNET_FRAME {
        MIN_ETHERNET_FRAME
    } else {
        frame.len()
    };
    let total = (VIRTIO_NET_HDR_LEN + eth_len) as u32;
    let mut tries = 0u32;
    while tx.next_avail.wrapping_sub(tx.used_idx()) >= tx.buf_count {
        if tries >= MAX_YIELDS {
            return Err(TxError::Timeout);
        }
        mk_yield();
        tries = tries.wrapping_add(1);
    }
    let slot = tx.next_avail % tx.buf_count;
    unsafe {
        let buf = tx.buffer_mut(slot, total);
        for b in buf.iter_mut() {
            *b = 0;
        }
        buf[VIRTIO_NET_HDR_LEN..VIRTIO_NET_HDR_LEN + frame.len()].copy_from_slice(frame);
    }
    tx.post_packet(slot, total);
    unsafe {
        regs.w16(LEG_QUEUE_NOTIFY, Q_TX);
    }
    let target = tx.next_avail.wrapping_add(1);
    tx.next_avail = target;
    tries = 0;
    loop {
        if tx.used_idx() == target {
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
