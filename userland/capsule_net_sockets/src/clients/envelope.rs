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

use nonos_libc::mk_ipc_call_timeout;

pub const HDR: usize = 20;

pub fn call(port: u32, magic: u32, op: u16, body: &[u8], out: &mut [u8]) -> Result<usize, u16> {
    call_t(port, magic, op, body, out, 0)
}

pub fn call_t(
    port: u32,
    magic: u32,
    op: u16,
    body: &[u8],
    out: &mut [u8],
    timeout_ms: u64,
) -> Result<usize, u16> {
    let mut req = vec![0u8; HDR + body.len()];
    write(&mut req, magic, op, body.len() as u32);
    req[HDR..].copy_from_slice(body);
    let mut resp = vec![0u8; HDR + out.len()];
    let n = mk_ipc_call_timeout(
        port as u64,
        req.as_ptr(),
        req.len(),
        resp.as_mut_ptr(),
        resp.len(),
        timeout_ms,
    );
    if n < 0 {
        return Err(15);
    }
    parse(magic, op, &resp, out)
}

fn write(out: &mut [u8], magic: u32, op: u16, len: u32) {
    out[0..4].copy_from_slice(&magic.to_le_bytes());
    out[4..6].copy_from_slice(&1u16.to_le_bytes());
    out[6..8].copy_from_slice(&op.to_le_bytes());
    out[8..12].fill(0);
    out[12..16].copy_from_slice(&1u32.to_le_bytes());
    out[16..20].copy_from_slice(&len.to_le_bytes());
}

fn parse(magic: u32, op: u16, resp: &[u8], out: &mut [u8]) -> Result<usize, u16> {
    if resp.len() < HDR || le32(resp, 0) != magic {
        return Err(4);
    }
    let got_op = u16::from_le_bytes([resp[6], resp[7]]);
    let errno = u16::from_le_bytes([resp[8], resp[9]]);
    let len = le32(resp, 16) as usize;
    if got_op != op || errno != 0 || HDR + len > resp.len() || len > out.len() {
        return Err(errno.max(4));
    }
    out[..len].copy_from_slice(&resp[HDR..HDR + len]);
    Ok(len)
}

fn le32(buf: &[u8], off: usize) -> u32 {
    u32::from_le_bytes([buf[off], buf[off + 1], buf[off + 2], buf[off + 3]])
}
