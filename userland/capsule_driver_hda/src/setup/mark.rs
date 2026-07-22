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

pub(super) fn mark(s: &str) {
    mk_debug(s.as_ptr(), s.len());
}

pub(super) fn mark_corb_vid(vid: u16) {
    let prefix = b"[HDA] corb vid=";
    let mut buf = [0u8; 20];
    let mut n = prefix.len();
    buf[..n].copy_from_slice(prefix);
    let mut shift: i32 = 12;
    while shift >= 0 {
        let nib = ((vid >> shift) & 0xf) as u8;
        buf[n] = if nib < 10 { b'0' + nib } else { b'a' + nib - 10 };
        n += 1;
        shift -= 4;
    }
    buf[n] = b'\n';
    n += 1;
    mk_debug(buf.as_ptr(), n);
}

fn put_str(buf: &mut [u8], n: usize, s: &[u8]) -> usize {
    buf[n..n + s.len()].copy_from_slice(s);
    n + s.len()
}

fn put_dec(buf: &mut [u8], mut n: usize, v: u8) -> usize {
    if v >= 100 {
        buf[n] = b'0' + v / 100;
        n += 1;
    }
    if v >= 10 {
        buf[n] = b'0' + (v / 10) % 10;
        n += 1;
    }
    buf[n] = b'0' + v % 10;
    n + 1
}

pub(super) fn mark_path(cad: u8, dac: u8, pin: u8) {
    let mut buf = [0u8; 48];
    let mut n = put_str(&mut buf, 0, b"[HDA] path cad=");
    n = put_dec(&mut buf, n, cad);
    n = put_str(&mut buf, n, b" dac=");
    n = put_dec(&mut buf, n, dac);
    n = put_str(&mut buf, n, b" pin=");
    n = put_dec(&mut buf, n, pin);
    buf[n] = b'\n';
    mk_debug(buf.as_ptr(), n + 1);
}
