// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

//! Parse an `iwl_rx_packet`: the framing the firmware puts every response and
//! notification in. A 4-byte `len_n_flags` gives the byte count that follows,
//! then a 4-byte command header (command, group, sequence), then the payload.
//! The sequence matches the response back to the command that asked for it.
//! Pure parsing over untrusted DRAM, checked by `iwlwifi_proofs`.

/// len_n_flags carries the following byte count in its low 14 bits.
pub const FRAME_SIZE_MASK: u32 = 0x3FFF;
/// The command header (command, group, sequence) after len_n_flags.
pub const CMD_HEADER_LEN: usize = 4;
/// len_n_flags plus the command header.
pub const RX_HEADER_LEN: usize = 4 + CMD_HEADER_LEN;

/// A parsed received packet. `payload` borrows the receive buffer.
pub struct RxPacket<'a> {
    pub cmd: u8,
    pub group: u8,
    pub sequence: u16,
    pub payload: &'a [u8],
}

/// Parse a received buffer. Returns `None` if it is shorter than the header or
/// the declared length runs past it.
pub fn parse(buf: &[u8]) -> Option<RxPacket<'_>> {
    if buf.len() < RX_HEADER_LEN {
        return None;
    }
    let len_n_flags = u32::from_le_bytes([buf[0], buf[1], buf[2], buf[3]]);
    let after = (len_n_flags & FRAME_SIZE_MASK) as usize;
    if after < CMD_HEADER_LEN {
        return None;
    }
    let end = 4usize.checked_add(after)?;
    if end > buf.len() {
        return None;
    }
    Some(RxPacket {
        cmd: buf[4],
        group: buf[5],
        sequence: u16::from_le_bytes([buf[6], buf[7]]),
        payload: &buf[RX_HEADER_LEN..end],
    })
}
