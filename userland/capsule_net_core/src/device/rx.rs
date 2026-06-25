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

use alloc::vec;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicU32, Ordering};

use nonos_libc::mk_ipc_call;

use crate::protocol::header::{parse_response, write_request};
use crate::protocol::ops::{HDR_LEN, OP_RX_PACKET};

const MAX_FRAME: usize = 1514;
const RESP_CAP: usize = HDR_LEN + 4 + 4 + 12 + MAX_FRAME;

static SEQ: AtomicU32 = AtomicU32::new(1);

fn next_rid() -> u32 {
    let v = SEQ.fetch_add(1, Ordering::Relaxed);
    if v == 0 { SEQ.fetch_add(1, Ordering::Relaxed) } else { v }
}

pub fn poll_frame(port: u32) -> Option<Vec<u8>> {
    let mut req = [0u8; HDR_LEN];
    if write_request(&mut req, OP_RX_PACKET, next_rid(), 0).is_none() {
        return None;
    }
    let mut resp = vec![0u8; RESP_CAP];
    let n = mk_ipc_call(port as u64, req.as_ptr(), HDR_LEN, resp.as_mut_ptr(), resp.len());
    if n < 0 {
        return None;
    }
    let got = n as usize;
    if got > resp.len() {
        return None;
    }
    let view = &resp[..got];
    let (op, _, plen) = parse_response(view)?;
    if op != OP_RX_PACKET || (plen as usize) < 4 || view.len() < HDR_LEN + 4 {
        return None;
    }
    let status = i32::from_le_bytes([view[HDR_LEN], view[HDR_LEN+1], view[HDR_LEN+2], view[HDR_LEN+3]]);
    if status != 0 {
        return None;
    }
    if (plen as usize) < 8 || view.len() < HDR_LEN + 8 {
        return None;
    }
    let body = HDR_LEN + 4;
    let frame_len = u32::from_le_bytes([view[body], view[body+1], view[body+2], view[body+3]]) as usize;
    let frame_start = body + 4;
    if frame_start + frame_len > view.len() {
        return None;
    }
    Some(view[frame_start..frame_start + frame_len].to_vec())
}
