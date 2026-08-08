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

/// Say how a handshake flight ended and how much of it had arrived.
///
/// A handshake that stops carries no message of its own: the page simply
/// stays on "downloading" whether the flight was abandoned, believed early,
/// or completed. Over the mixnet those three have very different causes and
/// look identical from the outside, so the reason and the byte count are
/// worth saying out loud.
pub fn flight(reason: &[u8], have: usize, idle: u32) {
    let mut line = [0u8; 96];
    let mut n = 0;
    for &b in b"[BROWSER] tls flight " {
        line[n] = b;
        n += 1;
    }
    for &b in reason {
        if n < line.len() - 32 {
            line[n] = b;
            n += 1;
        }
    }
    for &b in b" bytes " {
        line[n] = b;
        n += 1;
    }
    n += write_num(&mut line[n..], have as u64);
    for &b in b" idle " {
        line[n] = b;
        n += 1;
    }
    n += write_num(&mut line[n..], idle as u64);
    line[n] = b'\n';
    n += 1;
    unsafe { mk_debug(line.as_ptr(), n) };
}

fn write_num(out: &mut [u8], mut v: u64) -> usize {
    let mut digits = [0u8; 20];
    let mut d = 0;
    loop {
        digits[d] = b'0' + (v % 10) as u8;
        d += 1;
        v /= 10;
        if v == 0 {
            break;
        }
    }
    for i in 0..d {
        out[i] = digits[d - 1 - i];
    }
    d
}
