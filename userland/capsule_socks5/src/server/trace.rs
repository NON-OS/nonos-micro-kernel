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

/// Say which step of opening a tunnel refused. The client only learns the
/// connect was rejected; no session, no exit and a failed send look identical
/// from there.
pub fn open_failed(step: &[u8], code: u16) {
    let mut line = [0u8; 64];
    let mut n = 0;
    for &b in b"[SOCKS5] open refused: " {
        line[n] = b;
        n += 1;
    }
    for &b in step {
        if n < line.len() - 10 {
            line[n] = b;
            n += 1;
        }
    }
    if code != 0 {
        line[n] = b' ';
        n += 1;
        n += write_u16(&mut line[n..], code);
    }
    line[n] = b'\n';
    mk_debug(line.as_ptr(), n + 1);
}

fn write_u16(out: &mut [u8], mut v: u16) -> usize {
    let mut digits = [0u8; 5];
    let mut k = 0;
    loop {
        digits[k] = b'0' + (v % 10) as u8;
        v /= 10;
        k += 1;
        if v == 0 {
            break;
        }
    }
    for i in 0..k {
        out[i] = digits[k - 1 - i];
    }
    k
}

/// Say what a tunnel did, not only what refused it.
///
/// A connection that opens and then goes quiet looks the same from the client
/// as one that was never opened, so the steps that succeed are worth as much
/// in a log as the ones that fail.
pub fn step(what: &[u8], value: u64) {
    let mut line = [0u8; 80];
    let mut n = 0;
    for &b in b"[SOCKS5] " {
        line[n] = b;
        n += 1;
    }
    for &b in what {
        if n < line.len() - 24 {
            line[n] = b;
            n += 1;
        }
    }
    line[n] = b' ';
    n += 1;
    n += write_u64(&mut line[n..], value);
    line[n] = b'\n';
    mk_debug(line.as_ptr(), n + 1);
}

fn write_u64(out: &mut [u8], mut v: u64) -> usize {
    let mut digits = [0u8; 20];
    let mut k = 0;
    loop {
        digits[k] = b'0' + (v % 10) as u8;
        v /= 10;
        k += 1;
        if v == 0 {
            break;
        }
    }
    for i in 0..k {
        out[i] = digits[k - 1 - i];
    }
    k
}
