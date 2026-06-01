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

use nonos_libc::mk_debug;

const PREFIX: &[u8] = b"[virtio_blk] ";
const MAX_LABEL: usize = 200;

pub fn marker(label: &[u8]) {
    let n = if label.len() > MAX_LABEL { MAX_LABEL } else { label.len() };
    let mut buf = [0u8; PREFIX.len() + MAX_LABEL + 1];
    let p = PREFIX.len();
    buf[..p].copy_from_slice(PREFIX);
    buf[p..p + n].copy_from_slice(&label[..n]);
    buf[p + n] = b'\n';
    let _ = mk_debug(buf.as_ptr(), p + n + 1);
}

pub fn marker_i64(label: &[u8], value: i64) {
    let n = if label.len() > MAX_LABEL { MAX_LABEL } else { label.len() };
    let mut buf = [0u8; 256];
    let p = PREFIX.len();
    buf[..p].copy_from_slice(PREFIX);
    buf[p..p + n].copy_from_slice(&label[..n]);
    let mut pos = p + n;
    buf[pos] = b' ';
    pos += 1;
    pos += write_dec(&mut buf[pos..], value);
    buf[pos] = b'\n';
    pos += 1;
    let _ = mk_debug(buf.as_ptr(), pos);
}

fn write_dec(out: &mut [u8], value: i64) -> usize {
    let mut tmp = [0u8; 20];
    let neg = value < 0;
    let mut v = if neg { (value as i128).unsigned_abs() as u64 } else { value as u64 };
    let mut idx = tmp.len();
    if v == 0 {
        idx -= 1;
        tmp[idx] = b'0';
    }
    while v > 0 {
        idx -= 1;
        tmp[idx] = b'0' + (v % 10) as u8;
        v /= 10;
    }
    let mut w = 0;
    if neg {
        out[w] = b'-';
        w += 1;
    }
    let digits = &tmp[idx..];
    out[w..w + digits.len()].copy_from_slice(digits);
    w + digits.len()
}
