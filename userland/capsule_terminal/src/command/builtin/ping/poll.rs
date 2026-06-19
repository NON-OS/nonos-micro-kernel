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

use nonos_libc::{mk_ipc_call, mk_time_millis};

use super::icmp::{parse_reply, ECHO_REPLY};
use super::{HDR_LEN, IP_MAGIC, OP_POLL_PACKET, PING_ID, PING_SEQ, PROTO_ICMP};

const E_OK: u16 = 0;

pub fn poll_once(port: u32, dst: [u8; 4], t0: i64) -> Option<u64> {
    let mut req = [0u8; HDR_LEN + 1];
    req[0..4].copy_from_slice(&IP_MAGIC.to_le_bytes());
    req[4..6].copy_from_slice(&1u16.to_le_bytes());
    req[6..8].copy_from_slice(&OP_POLL_PACKET.to_le_bytes());
    req[16..20].copy_from_slice(&1u32.to_le_bytes());
    req[20] = PROTO_ICMP;
    let mut rx = [0u8; 96];
    let n = mk_ipc_call(port as u64, req.as_ptr(), req.len(), rx.as_mut_ptr(), rx.len());
    if n < (HDR_LEN + 9) as i64 || u16::from_le_bytes([rx[8], rx[9]]) != E_OK {
        return None;
    }
    if rx[HDR_LEN + 8] != PROTO_ICMP || rx[HDR_LEN..HDR_LEN + 4] != dst {
        return None;
    }
    let (ty, id, seq) = parse_reply(&rx[HDR_LEN + 9..n as usize])?;
    if ty != ECHO_REPLY || id != PING_ID || seq != PING_SEQ {
        return None;
    }
    Some(mk_time_millis().wrapping_sub(t0) as u64)
}
