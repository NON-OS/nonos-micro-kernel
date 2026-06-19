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

use nonos_libc::mk_ipc_call;

use super::icmp::build_echo;
use super::{HDR_LEN, IP_MAGIC, OP_SEND_PACKET, PING_ID, PING_SEQ, PROTO_ICMP};

const PAYLOAD: [u8; 16] = *b"NONOS-ping-echo!";

pub fn send_echo(port: u32, dst: [u8; 4]) -> u16 {
    let mut icmp = [0u8; 24];
    let Some(n) = build_echo(PING_ID, PING_SEQ, &PAYLOAD, &mut icmp) else {
        return 0xffff;
    };
    let mut req = [0u8; HDR_LEN + 5 + 24];
    req[0..4].copy_from_slice(&IP_MAGIC.to_le_bytes());
    req[4..6].copy_from_slice(&1u16.to_le_bytes());
    req[6..8].copy_from_slice(&OP_SEND_PACKET.to_le_bytes());
    req[16..20].copy_from_slice(&((5 + n) as u32).to_le_bytes());
    req[20..24].copy_from_slice(&dst);
    req[24] = PROTO_ICMP;
    req[25..25 + n].copy_from_slice(&icmp[..n]);
    let mut rx = [0u8; HDR_LEN];
    let r = mk_ipc_call(port as u64, req.as_ptr(), HDR_LEN + 5 + n, rx.as_mut_ptr(), rx.len());
    if r < HDR_LEN as i64 {
        return 0xffff;
    }
    u16::from_le_bytes([rx[8], rx[9]])
}
