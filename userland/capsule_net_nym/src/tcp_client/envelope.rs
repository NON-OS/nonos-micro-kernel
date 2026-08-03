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

use super::errno::{E_CALL, E_ERRNO, E_LEN, E_MAGIC, E_OP, E_SHORT};
use nonos_libc::mk_ipc_call;

pub const HDR: usize = 20;
const MAGIC: u32 = 0x4E54_4350;

pub fn call(port: u32, op: u16, body: &[u8], out: &mut [u8]) -> Result<usize, u16> {
    let mut req = vec![0u8; HDR + body.len()];
    write(&mut req, op, body.len() as u32);
    req[HDR..].copy_from_slice(body);
    let mut resp = vec![0u8; HDR + out.len()];
    let n = mk_ipc_call(port as u64, req.as_ptr(), req.len(), resp.as_mut_ptr(), resp.len());
    if n < 0 {
        return Err(E_CALL);
    }
    parse(op, &resp[..n as usize], out)
}

fn write(out: &mut [u8], op: u16, len: u32) {
    out[0..4].copy_from_slice(&MAGIC.to_le_bytes());
    out[4..6].copy_from_slice(&1u16.to_le_bytes());
    out[6..8].copy_from_slice(&op.to_le_bytes());
    out[8..12].fill(0);
    out[12..16].copy_from_slice(&1u32.to_le_bytes());
    out[16..20].copy_from_slice(&len.to_le_bytes());
}

/// Each rejection reports its own reason, so a log line says what was wrong
/// with the answer rather than only that there was something.
fn parse(op: u16, resp: &[u8], out: &mut [u8]) -> Result<usize, u16> {
    if resp.len() < HDR {
        return Err(E_SHORT);
    }
    if le32(resp, 0) != MAGIC {
        return Err(E_MAGIC);
    }
    let got_op = u16::from_le_bytes([resp[6], resp[7]]);
    let errno = u16::from_le_bytes([resp[8], resp[9]]);
    let len = le32(resp, 16) as usize;
    if got_op != op {
        return Err(E_OP);
    }
    if errno != 0 {
        return Err(E_ERRNO + errno);
    }
    if HDR + len > resp.len() || len > out.len() {
        return Err(E_LEN);
    }
    out[..len].copy_from_slice(&resp[HDR..HDR + len]);
    Ok(len)
}

fn le32(buf: &[u8], off: usize) -> u32 {
    u32::from_le_bytes([buf[off], buf[off + 1], buf[off + 2], buf[off + 3]])
}
