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

/// Report which stage of a gateway connection failed, and with what. Without
/// this a failure is only an absence of traffic in a capture.
pub fn fail(stage: &[u8], code: u16) {
    let mut line = [0u8; 64];
    let mut n = 0;
    for &b in b"[NET-NYM] gateway " {
        line[n] = b;
        n += 1;
    }
    for &b in stage {
        if n < line.len() - 8 {
            line[n] = b;
            n += 1;
        }
    }
    line[n] = b' ';
    n += 1;
    n += write_u16(&mut line[n..], code);
    line[n] = b'\n';
    mk_debug(line.as_ptr(), n + 1);
}

/// Report the gateway a session was established with.
pub fn bound(ip: [u8; 4]) {
    let mut line = [0u8; 64];
    let mut n = 0;
    for &b in b"[NET-NYM] gateway bound " {
        line[n] = b;
        n += 1;
    }
    for (i, &octet) in ip.iter().enumerate() {
        if i > 0 {
            line[n] = b'.';
            n += 1;
        }
        n += write_u16(&mut line[n..], octet as u16);
    }
    line[n] = b'\n';
    mk_debug(line.as_ptr(), n + 1);
}

/// Report how many nodes a directory fetch installed, so a live list is
/// distinguishable from the compiled one it replaced.
pub fn directory(count: usize) {
    let mut line = [0u8; 64];
    let mut n = 0;
    for &b in b"[NET-NYM] directory nodes " {
        line[n] = b;
        n += 1;
    }
    n += write_u16(&mut line[n..], count.min(u16::MAX as usize) as u16);
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
