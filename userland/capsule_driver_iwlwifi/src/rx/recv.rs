// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

//! Receive the next packet the firmware posted. If the driver's read index has
//! not caught the firmware's write index, the receive buffer at the read index
//! holds a packet; parse it, copy its payload out, and advance the read index.
//! Ties the packet parse and the ring index together, the counterpart to the
//! host-command send. Written to the legacy spec; needs hardware validation.

use crate::constants::{RB_SIZE, RX_QUEUE_SIZE};
use crate::rx::packet::parse;
use crate::rx::ring::{advance, has_packet};

/// Receive the next packet. Returns `(cmd, group, sequence, payload_len, new
/// read index)` with the payload copied into `out`, or `None` if the ring is
/// empty or the buffer does not parse.
///
/// # Safety
/// `dma_user_va` must point at a buffer holding `RX_QUEUE_SIZE` receive buffers
/// of `RB_SIZE` bytes starting at `rb_off`.
pub unsafe fn receive(
    dma_user_va: u64,
    rb_off: usize,
    read_idx: usize,
    write_idx: usize,
    out: &mut [u8],
) -> Option<(u8, u8, u16, usize, usize)> {
    if !has_packet(read_idx, write_idx) {
        return None;
    }
    let r = read_idx & (RX_QUEUE_SIZE - 1);
    let rb = rb_off + r * RB_SIZE;
    let buf = unsafe { core::slice::from_raw_parts((dma_user_va as *const u8).add(rb), RB_SIZE) };
    let pkt = parse(buf)?;
    let n = core::cmp::min(pkt.payload.len(), out.len());
    out[..n].copy_from_slice(&pkt.payload[..n]);
    Some((pkt.cmd, pkt.group, pkt.sequence, n, advance(read_idx)))
}
