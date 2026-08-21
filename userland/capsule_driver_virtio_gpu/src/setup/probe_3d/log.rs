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

pub fn log(msg: &[u8]) {
    mk_debug(msg.as_ptr(), msg.len());
}

pub fn log_err(prefix: &[u8], e: &str) {
    let mut buf = [0u8; 96];
    let mut n = 0;
    push(&mut buf, &mut n, prefix);
    push(&mut buf, &mut n, e.as_bytes());
    mk_debug(buf.as_ptr(), n);
}

pub fn log_caps(id: u32, ver: u32, size: u32) {
    let mut buf = [0u8; 96];
    let mut n = 0;
    push(&mut buf, &mut n, b"[GPU-3D] virgl render ok ctx=1 capset id=");
    push_dec(&mut buf, &mut n, id);
    push(&mut buf, &mut n, b" ver=");
    push_dec(&mut buf, &mut n, ver);
    push(&mut buf, &mut n, b" max=");
    push_dec(&mut buf, &mut n, size);
    mk_debug(buf.as_ptr(), n);
}

fn push(buf: &mut [u8], n: &mut usize, bytes: &[u8]) {
    let take = bytes.len().min(buf.len() - *n);
    buf[*n..*n + take].copy_from_slice(&bytes[..take]);
    *n += take;
}

fn push_dec(buf: &mut [u8], n: &mut usize, mut v: u32) {
    let mut tmp = [0u8; 10];
    let mut k = 0;
    loop {
        tmp[k] = b'0' + (v % 10) as u8;
        v /= 10;
        k += 1;
        if v == 0 {
            break;
        }
    }
    for i in 0..k {
        push(buf, n, &tmp[k - 1 - i..k - i]);
    }
}
